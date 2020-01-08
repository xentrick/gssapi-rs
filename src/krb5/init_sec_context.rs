use ::libc;

pub mod k5_thread_h {

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
    #[inline]

    pub unsafe extern "C" fn store_16_le(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_5)).i = val as crate::stdlib::uint16_t;
    }
    #[inline]

    pub unsafe extern "C" fn store_32_le(mut val: u32, mut vp: *mut libc::c_void) {
        let mut p = vp as *mut u8;
        (*(p as *mut crate::k5_platform_h::C2RustUnnamed_6)).i = val;
    }

    use crate::src::krb5::init_sec_context::byteswap_h::__bswap_16;
    use crate::src::krb5::init_sec_context::byteswap_h::__bswap_32;

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

    /* could be used in a table to find an etype and initialize a block */
    /* internal message representations */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time,
    optional */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /* data integrity checksum */
    /* encrypted part */
    /* user data */
    /* client time, optional */
    /* microsecond portion of time, opt. */
    /* sequence #, optional */
    /* sender address */
    /* recipient address, optional */
    /*
     * Begin "asn1.h"
     */
    /* ASN.1 encoding knowledge; KEEP IN SYNC WITH ASN.1 defs! */
    /* here we use some knowledge of ASN.1 encodings */
    /*
      Ticket is APPLICATION 1.
      Authenticator is APPLICATION 2.
      AS_REQ is APPLICATION 10.
      AS_REP is APPLICATION 11.
      TGS_REQ is APPLICATION 12.
      TGS_REP is APPLICATION 13.
      AP_REQ is APPLICATION 14.
      AP_REP is APPLICATION 15.
      KRB_SAFE is APPLICATION 20.
      KRB_PRIV is APPLICATION 21.
      KRB_CRED is APPLICATION 22.
      EncASRepPart is APPLICATION 25.
      EncTGSRepPart is APPLICATION 26.
      EncAPRepPart is APPLICATION 27.
      EncKrbPrivPart is APPLICATION 28.
      EncKrbCredPart is APPLICATION 29.
      KRB_ERROR is APPLICATION 30.
    */
    /* allow either constructed or primitive encoding, so check for bit 6
    set or reset */
    /* ************************************************************************
     * Prototypes for krb5_encode.c
     *************************************************************************/
    /*
      krb5_error_code encode_krb5_structure(const krb5_structure *rep,
      krb5_data **code);
      modifies  *code
      effects   Returns the ASN.1 encoding of *rep in **code.
      Returns ASN1_MISSING_FIELD if a required field is emtpy in *rep.
      Returns ENOMEM if memory runs out.
    */
    /* yes, the translation is identical to that used for KDC__REP */
    /* yes, the translation is identical to that used for KDC__REP */
    /* ************************************************************************
     * End of prototypes for krb5_encode.c
     *************************************************************************/
    /* ************************************************************************
     * Prototypes for krb5_decode.c
     *************************************************************************/
    /*
      krb5_error_code decode_krb5_structure(const krb5_data *code,
      krb5_structure **rep);

      requires  Expects **rep to not have been allocated;
      a new *rep is allocated regardless of the old value.
      effects   Decodes *code into **rep.
      Returns ENOMEM if memory is exhausted.
      Returns asn1 and krb5 errors.
    */
    /* kdb.h */
    /* Master key version number */
    /* kvno of key_data elements (all the same) */
    /* ************************************************************************
     * End of prototypes for krb5_decode.c
     *************************************************************************/
    /* KRB5_ASN1__ */
    /*
     * End "asn1.h"
     */
    /*
     * Internal krb5 library routines
     */
    /* Return true if s is non-empty and composed solely of digits. */
    /*
     * Initialization routines.
     */
    /* [De]serialize 4-byte integer */
    /* [De]serialize 8-byte integer */
    /* [De]serialize byte string */
    /* Fill in the buffer with random alpha-numeric data. */
    /* value to use when requesting a keytab entry and KVNO doesn't matter */
    /* value to use when requesting a keytab entry and enctype doesn't matter */
    /* To keep happy libraries which are (for now) accessing internal stuff */
    /* Make sure to increment by one when changing the struct */
    /* Used for KDB LDAP back end.  */
    /*
     * pkinit asn.1 encode/decode functions
     */

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
    #[inline]

    pub unsafe extern "C" fn empty_data() -> crate::krb5_h::krb5_data {
        return make_data(0 as *mut libc::c_void, 0u32);
    }
    /* Return the delta between two timestamps (a - b) as a signed 32-bit value,
     * without relying on undefined behavior. */
    #[inline]

    pub unsafe extern "C" fn ts_delta(
        mut a: crate::krb5_h::krb5_timestamp,
        mut b: crate::krb5_h::krb5_timestamp,
    ) -> crate::krb5_h::krb5_deltat {
        return (a as crate::stdlib::uint32_t).wrapping_sub(b as crate::stdlib::uint32_t)
            as crate::krb5_h::krb5_deltat;
    }
    /* Increment a timestamp by a signed 32-bit interval, without relying on
     * undefined behavior. */
    #[inline]

    pub unsafe extern "C" fn ts_incr(
        mut ts: crate::krb5_h::krb5_timestamp,
        mut delta: crate::krb5_h::krb5_deltat,
    ) -> crate::krb5_h::krb5_timestamp {
        return (ts as crate::stdlib::uint32_t).wrapping_add(delta as crate::stdlib::uint32_t)
            as crate::krb5_h::krb5_timestamp;
    }
    /* Return true if a comes after b. */
    #[inline]

    pub unsafe extern "C" fn ts_after(
        mut a: crate::krb5_h::krb5_timestamp,
        mut b: crate::krb5_h::krb5_timestamp,
    ) -> crate::krb5_h::krb5_boolean {
        return (a as crate::stdlib::uint32_t > b as crate::stdlib::uint32_t)
            as crate::krb5_h::krb5_boolean;
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

    use crate::src::krb5::init_sec_context::k5_int_h::empty_data;

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
}

/* _GSSAPI_KRB5_H_ */

/* __cplusplus */
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
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

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::k5_err_h::errinfo;
pub use crate::k5_int_h::_kdb5_dal_handle;
pub use crate::k5_int_h::_kdb_log_context;
pub use crate::k5_int_h::_krb5_authdata_context;
pub use crate::k5_int_h::_krb5_authdata_context_module;
pub use crate::k5_int_h::_krb5_context;
pub use crate::k5_int_h::_krb5_key_data;
pub use crate::k5_int_h::_krb5_kt;
pub use crate::k5_int_h::_krb5_kt_ops;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::_krb5int_access;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::derived_key;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_auth_con_set_authdata_context;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_authdata_export_authdata;
pub use crate::k5_int_h::krb5_key_st;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::krb5int_access;
pub use crate::k5_int_h::krb5int_accessor;
pub use crate::k5_int_h::krb5int_init_context_kdc;
pub use crate::k5_int_h::ldap_seqof_key_data;
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
pub use crate::k5_thread_h::k5_os_mutex_lock;
pub use crate::k5_thread_h::k5_os_mutex_unlock;
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_rep_enc_part;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
pub use crate::krb5_h::_krb5_ccache;
pub use crate::krb5_h::_krb5_checksum;
pub use crate::krb5_h::_krb5_creds;
pub use crate::krb5_h::_krb5_data;
pub use crate::krb5_h::_krb5_enc_data;
pub use crate::krb5_h::_krb5_enc_tkt_part;
pub use crate::krb5_h::_krb5_error;
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
pub use crate::krb5_h::krb5_ap_rep_enc_part;
pub use crate::krb5_h::krb5_ap_req;
pub use crate::krb5_h::krb5_auth_con_free;
pub use crate::krb5_h::krb5_auth_con_getflags;
pub use crate::krb5_h::krb5_auth_con_getlocalseqnumber;
pub use crate::krb5_h::krb5_auth_con_getsendsubkey;
pub use crate::krb5_h::krb5_auth_con_getsendsubkey_k;
pub use crate::krb5_h::krb5_auth_con_init;
pub use crate::krb5_h::krb5_auth_con_set_checksum_func;
pub use crate::krb5_h::krb5_auth_con_set_req_cksumtype;
pub use crate::krb5_h::krb5_auth_con_setflags;
pub use crate::krb5_h::krb5_auth_con_setsendsubkey_k;
pub use crate::krb5_h::krb5_auth_con_setuseruserkey;
pub use crate::krb5_h::krb5_auth_context;
pub use crate::krb5_h::krb5_authdata;
pub use crate::krb5_h::krb5_authdatatype;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_cc_retrieve_cred;
pub use crate::krb5_h::krb5_ccache;
pub use crate::krb5_h::krb5_checksum;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_const_principal;
pub use crate::krb5_h::krb5_context;
pub use crate::krb5_h::krb5_creds;
pub use crate::krb5_h::krb5_data;
pub use crate::krb5_h::krb5_deltat;
pub use crate::krb5_h::krb5_enc_data;
pub use crate::krb5_h::krb5_enc_tkt_part;
pub use crate::krb5_h::krb5_enctype;
pub use crate::krb5_h::krb5_error;
pub use crate::krb5_h::krb5_error_code;
pub use crate::krb5_h::krb5_flags;
pub use crate::krb5_h::krb5_free_ap_rep_enc_part;
pub use crate::krb5_h::krb5_free_authdata;
pub use crate::krb5_h::krb5_free_checksum_contents;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_cred_contents;
pub use crate::krb5_h::krb5_free_creds;
pub use crate::krb5_h::krb5_free_data;
pub use crate::krb5_h::krb5_free_data_contents;
pub use crate::krb5_h::krb5_free_error;
pub use crate::krb5_h::krb5_free_keyblock;
pub use crate::krb5_h::krb5_fwd_tgt_creds;
pub use crate::krb5_h::krb5_get_credentials;
pub use crate::krb5_h::krb5_init_context;
pub use crate::krb5_h::krb5_int16;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_create_key;
pub use crate::krb5_h::krb5_k_free_key;
pub use crate::krb5_h::krb5_kdc_req;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_mk_rep_dce;
pub use crate::krb5_h::krb5_mk_req_checksum_func;
pub use crate::krb5_h::krb5_mk_req_extended;
pub use crate::krb5_h::krb5_msgtype;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pa_data;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_preauthtype;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_compare;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_rc_st;
pub use crate::krb5_h::krb5_rcache;
pub use crate::krb5_h::krb5_rd_error;
pub use crate::krb5_h::krb5_rd_rep;
pub use crate::krb5_h::krb5_set_default_tgs_enctypes;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timeofday;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::krb5_h::krb5_ui_2;
pub use crate::krb5_h::krb5_ui_4;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::init_sec_context::k5_int_h::empty_data;
pub use crate::src::krb5::init_sec_context::k5_int_h::make_data;
pub use crate::src::krb5::init_sec_context::k5_int_h::ts_after;
pub use crate::src::krb5::init_sec_context::k5_int_h::ts_delta;
pub use crate::src::krb5::init_sec_context::k5_int_h::ts_incr;
pub use crate::src::krb5::init_sec_context::k5_platform_h::store_16_be;
pub use crate::src::krb5::init_sec_context::k5_platform_h::store_16_le;
pub use crate::src::krb5::init_sec_context::k5_platform_h::store_32_be;
pub use crate::src::krb5::init_sec_context::k5_platform_h::store_32_le;
pub use crate::src::krb5::init_sec_context::k5_thread_h::k5_mutex_lock;
pub use crate::src::krb5::init_sec_context::k5_thread_h::k5_mutex_unlock;

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
pub use crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapiP_krb5_h::C2RustUnnamed_0;
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_ext_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_channel_bindings_struct;
pub use crate::gssapi_h::gss_channel_bindings_t;
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_int32;
pub use crate::gssapi_h::gss_name_t;
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
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::generic::oid_ops::generic_gss_copy_oid;
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_init;
pub use crate::src::generic::util_token::gssint_g_make_token_header;
pub use crate::src::generic::util_token::gssint_g_token_size;
pub use crate::src::generic::util_token::gssint_g_verify_token_header;
pub use crate::src::krb5::acquire_cred::kg_cred_resolve;
pub use crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;
pub use crate::src::krb5::gssapi_krb5::gss_krb5int_initialize_library;

pub use crate::src::krb5::gssapi_krb5::kg_get_defcred;
pub use crate::src::krb5::gssapi_krb5::kg_sync_ccache_name;
pub use crate::src::krb5::iakerb::iakerb_make_finished;
pub use crate::src::krb5::import_sec_context::krb5_gss_convert_static_mech_oid;
pub use crate::src::krb5::init_sec_context::byteswap_h::__bswap_16;
pub use crate::src::krb5::init_sec_context::byteswap_h::__bswap_32;
pub use crate::src::krb5::init_sec_context::gssapiP_krb5_h::data_to_gss;
pub use crate::src::krb5::init_sec_context::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::krb5::naming_exts::kg_compare_name;
pub use crate::src::krb5::naming_exts::kg_duplicate_name;
pub use crate::src::krb5::naming_exts::kg_release_name;
pub use crate::src::krb5::rel_cred::krb5_gss_release_cred;
pub use crate::src::krb5::util_cksum::kg_checksum_channel_bindings;
pub use crate::src::krb5::util_crypt::kg_setup_keys;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gss_checksum_data {
    pub ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    pub cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    pub md5: crate::krb5_h::krb5_checksum,
    pub checksum_data: crate::krb5_h::krb5_data,
    pub exts: crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2000, 2002, 2003, 2007, 2008 by the Massachusetts Institute of
 * Technology.  All Rights Reserved.
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
/*
 * $Id$
 */
/* XXX This is for debugging only!!!  Should become a real bitfield
at some point */
#[no_mangle]

pub static mut krb5_gss_dbg_client_expcreds: i32 = 0i32;
/*
 * Common code which fetches the correct krb5 credentials from the
 * ccache.
 */

unsafe extern "C" fn get_credentials(
    mut context: crate::krb5_h::krb5_context,
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut server: crate::gssapiP_krb5_h::krb5_gss_name_t,
    mut now: crate::krb5_h::krb5_timestamp,
    mut endtime: crate::krb5_h::krb5_timestamp,
    mut out_creds: *mut *mut crate::krb5_h::krb5_creds,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut in_creds = crate::krb5_h::krb5_creds {
        magic: 0,
        client: 0 as *mut crate::krb5_h::krb5_principal_data,
        server: 0 as *mut crate::krb5_h::krb5_principal_data,
        keyblock: crate::krb5_h::krb5_keyblock {
            magic: 0,
            enctype: 0,
            length: 0,
            contents: 0 as *mut crate::krb5_h::krb5_octet,
        },
        times: crate::krb5_h::krb5_ticket_times {
            authtime: 0,
            starttime: 0,
            endtime: 0,
            renew_till: 0,
        },
        is_skey: 0,
        ticket_flags: 0,
        addresses: 0 as *mut *mut crate::krb5_h::krb5_address,
        ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        second_ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        authdata: 0 as *mut *mut crate::krb5_h::krb5_authdata,
    };
    let mut evidence_creds = crate::krb5_h::krb5_creds {
        magic: 0,
        client: 0 as *mut crate::krb5_h::krb5_principal_data,
        server: 0 as *mut crate::krb5_h::krb5_principal_data,
        keyblock: crate::krb5_h::krb5_keyblock {
            magic: 0,
            enctype: 0,
            length: 0,
            contents: 0 as *mut crate::krb5_h::krb5_octet,
        },
        times: crate::krb5_h::krb5_ticket_times {
            authtime: 0,
            starttime: 0,
            endtime: 0,
            renew_till: 0,
        },
        is_skey: 0,
        ticket_flags: 0,
        addresses: 0 as *mut *mut crate::krb5_h::krb5_address,
        ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        second_ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        authdata: 0 as *mut *mut crate::krb5_h::krb5_authdata,
    };
    let mut mcreds = crate::krb5_h::krb5_creds {
        magic: 0,
        client: 0 as *mut crate::krb5_h::krb5_principal_data,
        server: 0 as *mut crate::krb5_h::krb5_principal_data,
        keyblock: crate::krb5_h::krb5_keyblock {
            magic: 0,
            enctype: 0,
            length: 0,
            contents: 0 as *mut crate::krb5_h::krb5_octet,
        },
        times: crate::krb5_h::krb5_ticket_times {
            authtime: 0,
            starttime: 0,
            endtime: 0,
            renew_till: 0,
        },
        is_skey: 0,
        ticket_flags: 0,
        addresses: 0 as *mut *mut crate::krb5_h::krb5_address,
        ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        second_ticket: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        authdata: 0 as *mut *mut crate::krb5_h::krb5_authdata,
    };
    let mut result_creds = 0 as *mut crate::krb5_h::krb5_creds;
    let mut flags = 0i32;
    let mut server_data = crate::krb5_h::krb5_principal_data {
        magic: 0,
        realm: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        data: 0 as *mut crate::krb5_h::krb5_data,
        length: 0,
        type_0: 0,
    };
    *out_creds = 0 as *mut crate::krb5_h::krb5_creds;
    crate::stdlib::memset(
        &mut in_creds as *mut crate::krb5_h::krb5_creds as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::krb5_h::krb5_creds>(),
    );
    crate::stdlib::memset(
        &mut evidence_creds as *mut crate::krb5_h::krb5_creds as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::krb5_h::krb5_creds>(),
    );
    in_creds.server = 0 as crate::krb5_h::krb5_principal;
    in_creds.client = in_creds.server;
    if !(*cred).name.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"cred->name != NULL\x00" as *const u8 as *const i8,
            b"init_sec_context.c\x00" as *const u8 as *const i8,
            141u32,
            (*::std::mem::transmute::<&[u8; 34], &[i8; 34]>(
                b"krb5_error_code get_credentials()\x00",
            ))
            .as_ptr(),
        );
    }
    /* Remove assumed realm from host-based S4U2Proxy requests as they must
     * start in the client realm. */
    server_data = *(*server).princ;
    if !(*cred).impersonator.is_null() && server_data.type_0 == 3i32 {
        server_data.realm = empty_data()
    }
    in_creds.server = &mut server_data;
    in_creds.client = (*(*cred).name).princ;
    in_creds.times.endtime = endtime;
    in_creds.authdata = 0 as *mut *mut crate::krb5_h::krb5_authdata;
    in_creds.keyblock.enctype = 0i32;
    /*
     * cred->name is immutable, so there is no need to acquire
     * cred->name->lock.
     */
    if !(*(*cred).name).ad_context.is_null() {
        code = crate::k5_int_h::krb5_authdata_export_authdata(
            context,
            (*(*cred).name).ad_context,
            0x2i32,
            &mut in_creds.authdata,
        );
        if code != 0i32 {
            current_block = 4109192512989772929;
        } else {
            current_block = 4956146061682418353;
        }
    } else {
        current_block = 4956146061682418353;
    }
    match current_block {
        4956146061682418353 => {
            /*
             * For IAKERB or constrained delegation, only check the cache in this step.
             * For IAKERB we will ask the server to make any necessary TGS requests;
             * for constrained delegation we will adjust in_creds and make an S4U2Proxy
             * request below if the cache lookup fails.
             */
            if !(*cred).impersonator.is_null() || (*cred).iakerb_mech() as i32 != 0 {
                flags |= 2i32
            }
            code = crate::krb5_h::krb5_get_credentials(
                context,
                flags,
                (*cred).ccache,
                &mut in_creds,
                &mut result_creds,
            );
            /*
             * Try constrained delegation if we have proxy credentials, unless
             * we are trying to get a ticket to ourselves (in which case we could
             * just use the evidence ticket directly from cache).
             */
            if code as isize == -(1765328243 as isize)
                && !(*cred).impersonator.is_null()
                && (*cred).iakerb_mech() == 0
                && crate::krb5_h::krb5_principal_compare(
                    context,
                    (*cred).impersonator as crate::krb5_h::krb5_const_principal,
                    (*server).princ as crate::krb5_h::krb5_const_principal,
                ) == 0
            {
                crate::stdlib::memset(
                    &mut mcreds as *mut crate::krb5_h::krb5_creds as *mut libc::c_void,
                    0i32,
                    ::std::mem::size_of::<crate::krb5_h::krb5_creds>(),
                );
                mcreds.magic = -(1760647408 as isize) as crate::krb5_h::krb5_magic;
                mcreds.server = (*cred).impersonator;
                mcreds.client = (*(*cred).name).princ;
                code = crate::krb5_h::krb5_cc_retrieve_cred(
                    context,
                    (*cred).ccache,
                    0x20i32,
                    &mut mcreds,
                    &mut evidence_creds,
                );
                if code != 0 {
                    current_block = 4109192512989772929;
                } else {
                    in_creds.client = (*cred).impersonator;
                    in_creds.second_ticket = evidence_creds.ticket;
                    flags = 4i32 | 64i32;
                    code = crate::krb5_h::krb5_get_credentials(
                        context,
                        flags,
                        (*cred).ccache,
                        &mut in_creds,
                        &mut result_creds,
                    );
                    current_block = 5689316957504528238;
                }
            } else {
                current_block = 5689316957504528238;
            }
            match current_block {
                4109192512989772929 => {}
                _ => {
                    if !(code != 0) {
                        if flags & 64i32 != 0 {
                            if crate::krb5_h::krb5_principal_compare(
                                context,
                                (*(*cred).name).princ as crate::krb5_h::krb5_const_principal,
                                (*result_creds).client as crate::krb5_h::krb5_const_principal,
                            ) == 0
                            {
                                /* server did not support constrained delegation */
                                code = -(1765328237 as isize) as crate::krb5_h::krb5_error_code;
                                current_block = 4109192512989772929;
                            } else {
                                current_block = 2873832966593178012;
                            }
                        } else {
                            current_block = 2873832966593178012;
                        }
                        match current_block {
                            4109192512989772929 => {}
                            _ =>
                            /*
                             * Enforce a stricter limit (without timeskew forgiveness at the
                             * boundaries) because accept_sec_context code is also similarly
                             * non-forgiving.
                             */
                            {
                                if krb5_gss_dbg_client_expcreds == 0
                                    && ts_after(now, (*result_creds).times.endtime) != 0
                                {
                                    code = -(1765328352 as isize) as crate::krb5_h::krb5_error_code
                                } else {
                                    *out_creds = result_creds;
                                    result_creds = 0 as *mut crate::krb5_h::krb5_creds
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    crate::krb5_h::krb5_free_authdata(context, in_creds.authdata);
    crate::krb5_h::krb5_free_cred_contents(context, &mut evidence_creds);
    crate::krb5_h::krb5_free_creds(context, result_creds);
    return code;
}

unsafe extern "C" fn make_gss_checksum(
    mut context: crate::krb5_h::krb5_context,
    mut auth_context: crate::krb5_h::krb5_auth_context,
    mut cksum_data: *mut libc::c_void,
    mut out: *mut *mut crate::krb5_h::krb5_data,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut con_flags: crate::krb5_h::krb5_int32 = 0;
    let mut ptr = 0 as *mut u8;
    let mut data = cksum_data as *mut gss_checksum_data;
    let mut credmsg = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut junk: u32 = 0;
    let mut finished = 0 as *mut crate::krb5_h::krb5_data;
    let mut send_subkey = 0 as *mut crate::k5_int_h::krb5_key_st;
    (*data).checksum_data.data = 0 as *mut i8;
    credmsg.data = 0 as *mut i8;
    /* build the checksum field */
    if (*(*data).ctx).gss_flags & 1u32 != 0 {
        /* first get KRB_CRED message, so we know its length */
        /* clear the time check flag that was set in krb5_auth_con_init() */
        crate::krb5_h::krb5_auth_con_getflags(context, auth_context, &mut con_flags);
        crate::krb5_h::krb5_auth_con_setflags(context, auth_context, con_flags & !(0x1i32));
        if !(*(*data).cred).name.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"data->cred->name != NULL\x00" as *const u8 as
                              *const i8,
                          b"init_sec_context.c\x00" as *const u8 as
                              *const i8,
                          275u32,
                          (*::std::mem::transmute::<&[u8; 89],
                                                    &[i8; 89]>(b"krb5_error_code make_gss_checksum(krb5_context, krb5_auth_context, void *, krb5_data **)\x00")).as_ptr());
        }
        /*
         * RFC 4121 4.1.1 specifies forwarded credentials must be encrypted in
         * the session key, but krb5_fwd_tgt_creds will use the send subkey if
         * it's set in the auth context.  Suppress the send subkey
         * temporarily.
         */
        crate::krb5_h::krb5_auth_con_getsendsubkey_k(context, auth_context, &mut send_subkey);
        crate::krb5_h::krb5_auth_con_setsendsubkey_k(
            context,
            auth_context,
            0 as crate::krb5_h::krb5_key,
        );
        code = crate::krb5_h::krb5_fwd_tgt_creds(
            context,
            auth_context,
            0 as *const i8,
            (*(*(*data).cred).name).princ,
            (*(*(*data).ctx).there).princ,
            (*(*data).cred).ccache,
            1i32,
            &mut credmsg,
        );
        /* Turn KRB5_AUTH_CONTEXT_DO_TIME back on and reset the send subkey. */
        crate::krb5_h::krb5_auth_con_setflags(context, auth_context, con_flags);
        crate::krb5_h::krb5_auth_con_setsendsubkey_k(context, auth_context, send_subkey);
        crate::krb5_h::krb5_k_free_key(context, send_subkey);
        if code != 0 {
            /* don't fail here; just don't accept/do the delegation
            request */
            (*(*data).ctx).gss_flags &= !(1i32 | 32768i32) as u32;
            (*data).checksum_data.length = 24u32;
            current_block = 13472856163611868459;
        } else if credmsg.length.wrapping_add(28u32) > 65535u32 {
            code = -(1765328323 as isize) as crate::krb5_h::krb5_error_code;
            current_block = 11340719365295539846;
        } else {
            (*data).checksum_data.length = (28u32).wrapping_add(credmsg.length);
            current_block = 13472856163611868459;
        }
    } else {
        (*data).checksum_data.length = 24u32;
        current_block = 13472856163611868459;
    }
    match current_block {
        13472856163611868459 => {
            junk = 0u32;
            if !(*data).exts.is_null() {
            } else {
                crate::stdlib::__assert_fail(b"data->exts != NULL\x00" as *const u8 as
                                  *const i8,
                              b"init_sec_context.c\x00" as *const u8 as
                                  *const i8,
                              328u32,
                              (*::std::mem::transmute::<&[u8; 89],
                                                        &[i8; 89]>(b"krb5_error_code make_gss_checksum(krb5_context, krb5_auth_context, void *, krb5_data **)\x00")).as_ptr());
            }
            if !(*(*data).exts).iakerb.conv.is_null() {
                let mut key = 0 as *mut crate::k5_int_h::krb5_key_st;
                code =
                    crate::krb5_h::krb5_auth_con_getsendsubkey_k(context, auth_context, &mut key);
                if code != 0i32 {
                    current_block = 11340719365295539846;
                } else {
                    code = crate::src::krb5::iakerb::iakerb_make_finished(
                        context,
                        key,
                        (*(*data).exts).iakerb.conv,
                        &mut finished,
                    );
                    if code != 0i32 {
                        crate::krb5_h::krb5_k_free_key(context, key);
                        current_block = 11340719365295539846;
                    } else {
                        crate::krb5_h::krb5_k_free_key(context, key);
                        (*data).checksum_data.length = (*data)
                            .checksum_data
                            .length
                            .wrapping_add((8u32).wrapping_add((*finished).length));
                        current_block = 2873832966593178012;
                    }
                }
            } else {
                current_block = 2873832966593178012;
            }
            match current_block {
                11340719365295539846 => {}
                _ => {
                    (*data).checksum_data.length = (*data).checksum_data.length.wrapping_add(junk);
                    /* now allocate a buffer to hold the checksum data and
                    (maybe) KRB_CRED msg */
                    (*data).checksum_data.data =
                        crate::stdlib::malloc((*data).checksum_data.length as usize) as *mut i8;
                    if (*data).checksum_data.data.is_null() {
                        code = 12i32
                    } else {
                        ptr = (*data).checksum_data.data as *mut u8;
                        store_32_le((*data).md5.length, ptr as *mut libc::c_void);
                        ptr = ptr.offset(4isize);
                        crate::stdlib::memcpy(
                            ptr as *mut libc::c_void,
                            (*data).md5.contents as *const libc::c_void,
                            (*data).md5.length as usize,
                        );
                        ptr = ptr.offset((*data).md5.length as isize);
                        store_32_le((*(*data).ctx).gss_flags, ptr as *mut libc::c_void);
                        ptr = ptr.offset(4isize);
                        if !credmsg.data.is_null() {
                            store_16_le(1u32, ptr as *mut libc::c_void);
                            ptr = ptr.offset(2isize);
                            store_16_le(credmsg.length, ptr as *mut libc::c_void);
                            ptr = ptr.offset(2isize);
                            crate::stdlib::memcpy(
                                ptr as *mut libc::c_void,
                                credmsg.data as *const libc::c_void,
                                credmsg.length as usize,
                            );
                            ptr = ptr.offset(credmsg.length as isize)
                        }
                        if !(*(*data).exts).iakerb.conv.is_null() {
                            store_32_be(1u32, ptr as *mut libc::c_void);
                            ptr = ptr.offset(4isize);
                            store_32_be((*finished).length, ptr as *mut libc::c_void);
                            ptr = ptr.offset(4isize);
                            crate::stdlib::memcpy(
                                ptr as *mut libc::c_void,
                                (*finished).data as *const libc::c_void,
                                (*finished).length as usize,
                            );
                            ptr = ptr.offset((*finished).length as isize)
                        }
                        if junk != 0 {
                            crate::stdlib::memset(
                                ptr as *mut libc::c_void,
                                'i' as i32,
                                junk as usize,
                            );
                        }
                        *out = &mut (*data).checksum_data;
                        code = 0i32
                    }
                }
            }
        }
        _ => {}
    }
    crate::krb5_h::krb5_free_data_contents(context, &mut credmsg);
    crate::krb5_h::krb5_free_data(context, finished);
    return code;
}

unsafe extern "C" fn make_ap_req_v1(
    mut context: crate::krb5_h::krb5_context,
    mut ctx: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut k_cred: *mut crate::krb5_h::krb5_creds,
    mut ad_context: crate::k5_int_h::krb5_authdata_context,
    mut chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut token: crate::gssapi_h::gss_buffer_t,
    mut exts: crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut mk_req_flags = 0i32;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut cksum_struct = gss_checksum_data {
        ctx: 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
        cred: 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec,
        md5: crate::krb5_h::krb5_checksum {
            magic: 0,
            checksum_type: 0,
            length: 0,
            contents: 0 as *mut crate::krb5_h::krb5_octet,
        },
        checksum_data: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        exts: 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_ctx_ext_rec,
    };
    let mut md5 = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut ap_req = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut ptr = 0 as *mut u8;
    let mut t = 0 as *mut u8;
    let mut tlen: u32 = 0;
    ap_req.data = 0 as *mut i8;
    /* compute the hash of the channel bindings */
    code = crate::src::krb5::util_cksum::kg_checksum_channel_bindings(
        context,
        chan_bindings,
        &mut md5,
    );
    if code != 0 {
        return code;
    }
    crate::krb5_h::krb5_auth_con_set_req_cksumtype(context, (*ctx).auth_context, 0x8003i32);
    cksum_struct.md5 = md5;
    cksum_struct.ctx = ctx;
    cksum_struct.cred = cred;
    cksum_struct.checksum_data.data = 0 as *mut i8;
    cksum_struct.exts = exts;
    crate::krb5_h::krb5_auth_con_set_checksum_func(
        context,
        (*ctx).auth_context,
        Some(
            make_gss_checksum
                as unsafe extern "C" fn(
                    _: crate::krb5_h::krb5_context,
                    _: crate::krb5_h::krb5_auth_context,
                    _: *mut libc::c_void,
                    _: *mut *mut crate::krb5_h::krb5_data,
                ) -> crate::krb5_h::krb5_error_code,
        ),
        &mut cksum_struct as *mut gss_checksum_data as *mut libc::c_void,
    );
    /* call mk_req.  subkey and ap_req need to be used or destroyed */
    mk_req_flags = 0x1i32;
    if (*ctx).gss_flags & 2u32 != 0 {
        mk_req_flags |= 0x20000000i32 | 0x2i32
    }
    crate::k5_int_h::krb5_auth_con_set_authdata_context(context, (*ctx).auth_context, ad_context);
    code = crate::krb5_h::krb5_mk_req_extended(
        context,
        &mut (*ctx).auth_context,
        mk_req_flags,
        0 as *mut crate::krb5_h::krb5_data,
        k_cred,
        &mut ap_req,
    );
    crate::k5_int_h::krb5_auth_con_set_authdata_context(
        context,
        (*ctx).auth_context,
        0 as crate::k5_int_h::krb5_authdata_context,
    );
    crate::krb5_h::krb5_free_checksum_contents(context, &mut cksum_struct.md5);
    crate::krb5_h::krb5_free_data_contents(context, &mut cksum_struct.checksum_data);
    if !(code != 0) {
        /* store the interesting stuff from creds and authent */
        (*ctx).krb_times = (*k_cred).times;
        (*ctx).krb_flags = (*k_cred).ticket_flags;
        /* build up the token */
        if (*ctx).gss_flags & 0x1000u32 != 0 {
            /*
             * For DCE RPC, do not encapsulate the AP-REQ in the
             * typical GSS wrapping.
             */
            code = data_to_gss(&mut ap_req, token);
            if code != 0 {
                current_block = 941242863683976212;
            } else {
                current_block = 10692455896603418738;
            }
        } else {
            /* allocate space for the token */
            tlen = crate::src::generic::util_token::gssint_g_token_size(
                mech_type as *const crate::gssapi_h::gss_OID_desc,
                ap_req.length,
            );
            t = gssalloc_malloc(tlen as crate::stddef_h::size_t) as *mut u8;
            if t.is_null() {
                code = 12i32;
                current_block = 941242863683976212;
            } else {
                /* fill in the buffer */
                ptr = t;
                crate::src::generic::util_token::gssint_g_make_token_header(
                    mech_type as *const crate::gssapi_h::gss_OID_desc,
                    ap_req.length,
                    &mut ptr,
                    0x100i32,
                );
                crate::stdlib::memcpy(
                    ptr as *mut libc::c_void,
                    ap_req.data as *const libc::c_void,
                    ap_req.length as usize,
                );
                ptr = ptr.offset(ap_req.length as isize);
                /* pass it back */
                (*token).length = tlen as crate::stddef_h::size_t;
                (*token).value = t as *mut libc::c_void;
                current_block = 10692455896603418738;
            }
        }
        match current_block {
            941242863683976212 => {}
            _ => code = 0i32,
        }
    }
    if !ap_req.data.is_null() {
        crate::krb5_h::krb5_free_data_contents(context, &mut ap_req);
    }
    return code;
}
/*
 * new_connection
 *
 * Do the grunt work of setting up a new context.
 */

unsafe extern "C" fn kg_new_connection(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut actual_mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut context: crate::krb5_h::krb5_context,
    mut exts: crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut k_cred = 0 as *mut crate::krb5_h::krb5_creds;
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut ctx_free = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut now: crate::krb5_h::krb5_timestamp = 0;
    let mut token = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut keyblock = 0 as *mut crate::krb5_h::krb5_keyblock;
    major_status = (13u32) << 16i32;
    token.length = 0usize;
    token.value = 0 as *mut libc::c_void;
    /* make sure the cred is usable for init */
    if (*cred).usage != 1i32 && (*cred).usage != 0i32 {
        *minor_status = 0u32;
        return (7u32) << 16i32;
    }
    /* complain if the input token is non-null */
    if !input_token.is_null() && (*input_token).length != 0usize {
        *minor_status = 0u32;
        return (9u32) << 16i32;
    }
    /* create the ctx */
    ctx = crate::stdlib::malloc(::std::mem::size_of::<
        crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec,
    >()) as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    if ctx.is_null() {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    /* fill in the ctx */
    crate::stdlib::memset(
        ctx as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec>(),
    );
    (*ctx).magic = 39756040i32;
    ctx_free = ctx;
    code = crate::krb5_h::krb5_auth_con_init(context, &mut (*ctx).auth_context);
    if !(code != 0) {
        crate::krb5_h::krb5_auth_con_setflags(context, (*ctx).auth_context, 0x4i32);
        /* limit the encryption types negotiated (if requested) */
        if !(*cred).req_enctypes.is_null() {
            code = crate::krb5_h::krb5_set_default_tgs_enctypes(context, (*cred).req_enctypes);
            if code != 0 {
                current_block = 2643310281128592672;
            } else {
                current_block = 17478428563724192186;
            }
        } else {
            current_block = 17478428563724192186;
        }
        match current_block {
            2643310281128592672 => {}
            _ => {
                (*ctx).set_initiate(1u32);
                (*ctx).set_seed_init(0u32);
                (*ctx).seqstate = 0 as crate::gssapiP_generic_h::g_seqnum_state;
                /* enforce_ok_as_delegate causes GSS_C_DELEG_FLAG to be treated as
                 * GSS_C_DELEG_POLICY_FLAG (so ok-as-delegate is always enforced). */
                if (*context).enforce_ok_as_delegate != 0 && req_flags & 1u32 != 0 {
                    req_flags &= !(1i32) as u32;
                    req_flags |= 32768u32
                }
                (*ctx).gss_flags = req_flags
                    & (16i32
                        | 32i32
                        | 2i32
                        | 4i32
                        | 8i32
                        | 1i32
                        | 0x1000i32
                        | 0x2000i32
                        | 0x4000i32) as u32;
                (*ctx).gss_flags |= 256u32;
                if (*cred).suppress_ci_flags() == 0 {
                    (*ctx).gss_flags |= (16i32 | 32i32) as u32
                }
                if req_flags & 0x1000u32 != 0 {
                    (*ctx).gss_flags |= 2u32
                }
                code = crate::krb5_h::krb5_timeofday(context, &mut now);
                if !(code != 0) {
                    if time_req == 0u32 || time_req == 0xffffffffu32 {
                        (*ctx).krb_times.endtime = 0i32
                    } else {
                        (*ctx).krb_times.endtime =
                            ts_incr(now, time_req as crate::krb5_h::krb5_deltat)
                    }
                    code = crate::src::krb5::naming_exts::kg_duplicate_name(
                        context,
                        (*cred).name,
                        &mut (*ctx).here,
                    );
                    if !(code != 0) {
                        code = crate::src::krb5::naming_exts::kg_duplicate_name(
                            context,
                            target_name as crate::gssapiP_krb5_h::krb5_gss_name_t,
                            &mut (*ctx).there,
                        );
                        if !(code != 0) {
                            code = get_credentials(
                                context,
                                cred,
                                (*ctx).there,
                                now,
                                (*ctx).krb_times.endtime,
                                &mut k_cred,
                            );
                            if !(code != 0) {
                                (*ctx).krb_times = (*k_cred).times;
                                /*
                                 * GSS_C_DELEG_POLICY_FLAG means to delegate only if the
                                 * ok-as-delegate ticket flag is set.
                                 */
                                if req_flags & 32768u32 != 0
                                    && (*k_cred).ticket_flags & 0x40000i32 != 0
                                {
                                    (*ctx).gss_flags |= (1i32 | 32768i32) as u32
                                }
                                if crate::src::generic::oid_ops::generic_gss_copy_oid(
                                    minor_status,
                                    mech_type as *const crate::gssapi_h::gss_OID_desc,
                                    &mut (*ctx).mech_used,
                                ) != 0u32
                                {
                                    code = *minor_status as crate::krb5_h::krb5_error_code
                                } else {
                                    /*
                                     * Now try to make it static if at all possible....
                                     */
                                    (*ctx).mech_used =
                                        crate::src::krb5::import_sec_context::krb5_gss_convert_static_mech_oid((*ctx).mech_used);
                                    /* gsskrb5 v1 */
                                    let mut seq_temp: crate::krb5_h::krb5_int32 = 0;
                                    code = make_ap_req_v1(
                                        context,
                                        ctx,
                                        cred,
                                        k_cred,
                                        (*(*ctx).here).ad_context,
                                        input_chan_bindings,
                                        mech_type,
                                        &mut token,
                                        exts,
                                    );
                                    if code != 0 {
                                        if code as isize == -(1765328189 as isize)
                                            || code as isize == -(1765328243 as isize)
                                            || code as isize == 39756044 as isize
                                        {
                                            major_status = (7u32) << 16i32
                                        }
                                        if code as isize == -(1765328352 as isize) {
                                            major_status = (11u32) << 16i32
                                        }
                                    } else {
                                        crate::krb5_h::krb5_auth_con_getlocalseqnumber(
                                            context,
                                            (*ctx).auth_context,
                                            &mut seq_temp,
                                        );
                                        (*ctx).seq_send = seq_temp as crate::stdlib::uint64_t;
                                        code = crate::krb5_h::krb5_auth_con_getsendsubkey(
                                            context,
                                            (*ctx).auth_context,
                                            &mut keyblock,
                                        );
                                        if !(code != 0i32) {
                                            code = crate::krb5_h::krb5_k_create_key(
                                                context,
                                                keyblock,
                                                &mut (*ctx).subkey,
                                            );
                                            crate::krb5_h::krb5_free_keyblock(context, keyblock);
                                            if !(code != 0i32) {
                                                (*ctx).enc = 0 as crate::krb5_h::krb5_key;
                                                (*ctx).seq = 0 as crate::krb5_h::krb5_key;
                                                (*ctx).set_have_acceptor_subkey(0u32);
                                                code = crate::src::krb5::util_crypt::kg_setup_keys(
                                                    context,
                                                    ctx,
                                                    (*ctx).subkey,
                                                    &mut (*ctx).cksumtype,
                                                );
                                                if !(code != 0i32) {
                                                    if (*ctx).gss_flags & 2u32 == 0 {
                                                        /* There will be no AP-REP, so set up sequence state now. */
                                                        (*ctx).seq_recv = (*ctx).seq_send;
                                                        code =
                                                            crate::src::generic::util_seqstate::gssint_g_seqstate_init(&mut (*ctx).seqstate,
                                                                                   (*ctx).seq_recv,
                                                                                   ((*ctx).gss_flags
                                                                                        &
                                                                                        4u32
                                                                                        !=
                                                                                        0u32)
                                                                                       as
                                                                                       i32,
                                                                                   ((*ctx).gss_flags
                                                                                        &
                                                                                        8u32
                                                                                        !=
                                                                                        0u32)
                                                                                       as
                                                                                       i32,
                                                                                   (*ctx).proto)
                                                                as
                                                                crate::krb5_h::krb5_error_code;
                                                        if code != 0i32 {
                                                            current_block = 2643310281128592672;
                                                        } else {
                                                            current_block = 3229571381435211107;
                                                        }
                                                    } else {
                                                        current_block = 3229571381435211107;
                                                    }
                                                    match current_block {
                                                        2643310281128592672 => {}
                                                        _ =>
                                                        /* compute time_rec */
                                                        {
                                                            if !time_rec.is_null() {
                                                                code =
                                                                    crate::krb5_h::krb5_timeofday(
                                                                        context, &mut now,
                                                                    );
                                                                if code != 0 {
                                                                    current_block =
                                                                        2643310281128592672;
                                                                } else {
                                                                    *time_rec
                                                                        =
                                                                        ts_delta((*ctx).krb_times.endtime,
                                                                                 now)
                                                                            as
                                                                            crate::gssapi_h::OM_uint32;
                                                                    current_block =
                                                                        5793491756164225964;
                                                                }
                                                            } else {
                                                                current_block = 5793491756164225964;
                                                            }
                                                            match current_block {
                                                                2643310281128592672 => {}
                                                                _ => {
                                                                    /* set the other returns */
                                                                    *output_token = token;
                                                                    if !ret_flags.is_null() {
                                                                        *ret_flags =
                                                                            (*ctx).gss_flags
                                                                    }
                                                                    if !actual_mech_type.is_null() {
                                                                        *actual_mech_type =
                                                                            mech_type
                                                                    }
                                                                    /* return successfully */
                                                                    *context_handle
                                                                        =
                                                                        ctx as
                                                                            crate::gssapi_h::gss_ctx_id_t;
                                                                    ctx_free =
                                                                        0 as
                                                                            *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
                                                                    if (*ctx).gss_flags & 2u32 != 0
                                                                    {
                                                                        (*ctx)
                                                                            .set_established(0u32);
                                                                        major_status
                                                                            =
                                                                            ((1i32)
                                                                                 <<
                                                                                 0i32
                                                                                     +
                                                                                     0i32)
                                                                                as
                                                                                crate::gssapi_h::OM_uint32
                                                                    } else {
                                                                        (*ctx).gss_flags |= 128u32;
                                                                        (*ctx)
                                                                            .set_established(1u32);
                                                                        major_status = 0u32
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
                }
            }
        }
    }
    crate::krb5_h::krb5_free_creds(context, k_cred);
    if !ctx_free.is_null() {
        if !(*ctx_free).auth_context.is_null() {
            crate::krb5_h::krb5_auth_con_free(context, (*ctx_free).auth_context);
        }
        if !(*ctx_free).here.is_null() {
            crate::src::krb5::naming_exts::kg_release_name(context, &mut (*ctx_free).here);
        }
        if !(*ctx_free).there.is_null() {
            crate::src::krb5::naming_exts::kg_release_name(context, &mut (*ctx_free).there);
        }
        if !(*ctx_free).subkey.is_null() {
            crate::krb5_h::krb5_k_free_key(context, (*ctx_free).subkey);
        }
        crate::stdlib::free(ctx_free as *mut libc::c_void);
    }
    *minor_status = code as crate::gssapi_h::OM_uint32;
    return major_status;
}
/*
 * mutual_auth
 *
 * Handle the reply from the acceptor, if we're doing mutual auth.
 */

unsafe extern "C" fn mutual_auth(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut _req_flags: crate::gssapi_h::OM_uint32,
    mut _time_req: crate::gssapi_h::OM_uint32,
    mut _input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut actual_mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut context: crate::krb5_h::krb5_context,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut ptr = 0 as *mut u8;
    let mut sptr = 0 as *mut i8;
    let mut ap_rep = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut ap_rep_data = 0 as *mut crate::krb5_h::krb5_ap_rep_enc_part;
    let mut now: crate::krb5_h::krb5_timestamp = 0;
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut krb_error = 0 as *mut crate::krb5_h::krb5_error;
    let mut code: crate::krb5_h::krb5_error_code = 0;
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
    major_status = (13u32) << 16i32;
    code = crate::k5_int_h::krb5int_accessor(
        &mut kaccess,
        ((::std::mem::size_of::<crate::k5_int_h::krb5int_access>() & 0xffffusize
            | ((23i32) << 16i32) as usize) as u32
            & 0xffffffffu32) as crate::krb5_h::krb5_int32,
    );
    if !(code != 0) {
        ctx = *context_handle as crate::gssapiP_krb5_h::krb5_gss_ctx_id_t;
        /* make sure the context is non-established, and that certain
        arguments are unchanged */
        if (*ctx).established() as i32 != 0 || (*ctx).gss_flags & 2u32 == 0u32 {
            code = 39756036i32
        } else if crate::src::krb5::naming_exts::kg_compare_name(
            context,
            (*ctx).there,
            target_name as crate::gssapiP_krb5_h::krb5_gss_name_t,
        ) == 0
        {
            crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context(
                minor_status,
                context_handle,
                0 as crate::gssapi_h::gss_buffer_t,
            );
            code = 0i32;
            major_status = (2u32) << 16i32
        } else if input_token.is_null() {
            crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context(
                minor_status,
                context_handle,
                0 as crate::gssapi_h::gss_buffer_t,
            );
            code = 0i32;
            major_status = (9u32) << 16i32
        } else {
            ptr = (*input_token).value as *mut u8;
            if (*ctx).gss_flags & 0x1000u32 != 0 {
                /* verify the token and leave the AP_REP message in ap_rep */
                /* Raw AP-REP */
                ap_rep.length = (*input_token).length as u32;
                ap_rep.data = (*input_token).value as *mut i8;
                current_block = 7746103178988627676;
            } else if crate::src::generic::util_token::gssint_g_verify_token_header(
                (*ctx).mech_used,
                &mut ap_rep.length,
                &mut ptr,
                0x200i32,
                (*input_token).length as u32,
                1i32,
            ) != 0
            {
                if crate::src::generic::util_token::gssint_g_verify_token_header(
                    (*ctx).mech_used as *const crate::gssapi_h::gss_OID_desc,
                    &mut ap_rep.length,
                    &mut ptr,
                    0x300i32,
                    (*input_token).length as u32,
                    1i32,
                ) == 0i32
                {
                    /* Handle a KRB_ERROR message from the server */
                    sptr = ptr as *mut i8; /* PC compiler bug */
                    ap_rep.data = sptr; /* PC compiler bug */
                    sptr = sptr.offset(ap_rep.length as isize);
                    code = crate::krb5_h::krb5_rd_error(context, &mut ap_rep, &mut krb_error);
                    if !(code != 0) {
                        if (*krb_error).error != 0 {
                            code = ((*krb_error).error as crate::krb5_h::krb5_error_code as isize
                                + -(1765328384 as isize))
                                as crate::krb5_h::krb5_error_code
                        } else {
                            code = 0i32
                        }
                        crate::krb5_h::krb5_free_error(context, krb_error);
                    }
                } else {
                    *minor_status = 0u32;
                    return (9u32) << 16i32;
                }
                current_block = 16296328345790888590;
            } else {
                current_block = 7746103178988627676;
            }
            match current_block {
                16296328345790888590 => {}
                _ => {
                    sptr = ptr as *mut i8;
                    ap_rep.data = sptr;
                    sptr = sptr.offset(ap_rep.length as isize);
                    /* decode the ap_rep */
                    code = crate::krb5_h::krb5_rd_rep(
                        context,
                        (*ctx).auth_context,
                        &mut ap_rep,
                        &mut ap_rep_data,
                    );
                    if code != 0 {
                        /*
                         * XXX A hack for backwards compatibility.
                         * To be removed in 1999 -- proven
                         */
                        crate::krb5_h::krb5_auth_con_setuseruserkey(
                            context,
                            (*ctx).auth_context,
                            &mut (*(*ctx).subkey).keyblock,
                        );
                        if crate::krb5_h::krb5_rd_rep(
                            context,
                            (*ctx).auth_context,
                            &mut ap_rep,
                            &mut ap_rep_data,
                        ) != 0
                        {
                            current_block = 16296328345790888590;
                        } else {
                            current_block = 6717214610478484138;
                        }
                    } else {
                        current_block = 6717214610478484138;
                    }
                    match current_block {
                        16296328345790888590 => {}
                        _ => {
                            /* store away the sequence number */
                            (*ctx).seq_recv = (*ap_rep_data).seq_number as crate::stdlib::uint64_t;
                            code = crate::src::generic::util_seqstate::gssint_g_seqstate_init(
                                &mut (*ctx).seqstate,
                                (*ctx).seq_recv,
                                ((*ctx).gss_flags & 4u32 != 0u32) as i32,
                                ((*ctx).gss_flags & 8u32 != 0u32) as i32,
                                (*ctx).proto,
                            ) as crate::krb5_h::krb5_error_code;
                            if code != 0 {
                                crate::krb5_h::krb5_free_ap_rep_enc_part(context, ap_rep_data);
                            } else {
                                if !(*ap_rep_data).subkey.is_null()
                                    && ((*ctx).proto == 1i32
                                        || (*ctx).gss_flags & 0x1000u32 != 0
                                        || (*(*ap_rep_data).subkey).enctype
                                            != (*(*ctx).subkey).keyblock.enctype)
                                {
                                    /* Keep acceptor's subkey.  */
                                    (*ctx).set_have_acceptor_subkey(1u32);
                                    code = crate::krb5_h::krb5_k_create_key(
                                        context,
                                        (*ap_rep_data).subkey,
                                        &mut (*ctx).acceptor_subkey,
                                    );
                                    if code != 0 {
                                        crate::krb5_h::krb5_free_ap_rep_enc_part(
                                            context,
                                            ap_rep_data,
                                        );
                                        current_block = 16296328345790888590;
                                    } else {
                                        code = crate::src::krb5::util_crypt::kg_setup_keys(
                                            context,
                                            ctx,
                                            (*ctx).acceptor_subkey,
                                            &mut (*ctx).acceptor_subkey_cksumtype,
                                        );
                                        if code != 0 {
                                            crate::krb5_h::krb5_free_ap_rep_enc_part(
                                                context,
                                                ap_rep_data,
                                            );
                                            current_block = 16296328345790888590;
                                        } else {
                                            current_block = 168769493162332264;
                                        }
                                    }
                                } else {
                                    current_block = 168769493162332264;
                                }
                                match current_block {
                                    16296328345790888590 => {}
                                    _ => {
                                        /* free the ap_rep_data */
                                        crate::krb5_h::krb5_free_ap_rep_enc_part(
                                            context,
                                            ap_rep_data,
                                        );
                                        if (*ctx).gss_flags & 0x1000u32 != 0 {
                                            let mut outbuf = crate::krb5_h::krb5_data {
                                                magic: 0,
                                                length: 0,
                                                data: 0 as *mut i8,
                                            };
                                            code = crate::krb5_h::krb5_mk_rep_dce(
                                                context,
                                                (*ctx).auth_context,
                                                &mut outbuf,
                                            );
                                            if code != 0 {
                                                current_block = 16296328345790888590;
                                            } else {
                                                code = data_to_gss(&mut outbuf, output_token);
                                                if code != 0 {
                                                    current_block = 16296328345790888590;
                                                } else {
                                                    current_block = 7494008139977416618;
                                                }
                                            }
                                        } else {
                                            current_block = 7494008139977416618;
                                        }
                                        match current_block {
                                            16296328345790888590 => {}
                                            _ => {
                                                /* set established */
                                                (*ctx).set_established(1u32);
                                                /* set returns */
                                                if !time_rec.is_null() {
                                                    code = crate::krb5_h::krb5_timeofday(
                                                        context, &mut now,
                                                    );
                                                    if code != 0 {
                                                        current_block = 16296328345790888590;
                                                    } else {
                                                        *time_rec =
                                                            ts_delta((*ctx).krb_times.endtime, now)
                                                                as crate::gssapi_h::OM_uint32;
                                                        current_block = 7858101417678297991;
                                                    }
                                                } else {
                                                    current_block = 7858101417678297991;
                                                }
                                                match current_block {
                                                    16296328345790888590 => {}
                                                    _ => {
                                                        if !ret_flags.is_null() {
                                                            *ret_flags = (*ctx).gss_flags
                                                        }
                                                        if !actual_mech_type.is_null() {
                                                            *actual_mech_type = mech_type
                                                        }
                                                        /* success */
                                                        *minor_status = 0u32;
                                                        return 0u32;
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
    crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context(
        minor_status,
        context_handle,
        0 as crate::gssapi_h::gss_buffer_t,
    );
    *minor_status = code as crate::gssapi_h::OM_uint32;
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_init_sec_context_ext(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut claimant_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut actual_mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut exts: crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut defcred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut cred = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
    let mut kerr: crate::krb5_h::krb5_error_code = 0;
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut tmp_min_stat: crate::gssapi_h::OM_uint32 = 0;
    if (*context_handle).is_null() {
        kerr = krb5_gss_init_context(&mut context);
        if kerr != 0 {
            *minor_status = kerr as crate::gssapi_h::OM_uint32;
            return (13u32) << 16i32;
        }
        if crate::src::krb5::gssapi_krb5::kg_sync_ccache_name(context, minor_status)
            & ((0o377u32) << 24i32 | (0o377u32) << 16i32)
            != 0
        {
            crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
            crate::krb5_h::krb5_free_context(context);
            return (13u32) << 16i32;
        }
    } else {
        context = (*(*context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec)).k5_context
    }
    /* set up return values so they can be "freed" successfully */
    major_status = (13u32) << 16i32; /* Default major code */
    (*output_token).length = 0usize;
    (*output_token).value = 0 as *mut libc::c_void;
    if !actual_mech_type.is_null() {
        *actual_mech_type = 0 as crate::gssapi_h::gss_OID
    }
    /* verify the mech_type */
    if mech_type.is_null()
        || (*mech_type).length == (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).length
            && crate::stdlib::memcmp(
                (*mech_type).elements,
                (*crate::src::krb5::gssapi_krb5::gss_mech_krb5).elements,
                (*mech_type).length as usize,
            ) == 0i32
    {
        mech_type = crate::src::krb5::gssapi_krb5::gss_mech_krb5
    } else if (*mech_type).length == (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_old).length
        && crate::stdlib::memcmp(
            (*mech_type).elements,
            (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_old).elements,
            (*mech_type).length as usize,
        ) == 0i32
    {
        mech_type = crate::src::krb5::gssapi_krb5::gss_mech_krb5_old
    } else if (*mech_type).length == (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_wrong).length
        && crate::stdlib::memcmp(
            (*mech_type).elements,
            (*crate::src::krb5::gssapi_krb5::gss_mech_krb5_wrong).elements,
            (*mech_type).length as usize,
        ) == 0i32
    {
        mech_type = crate::src::krb5::gssapi_krb5::gss_mech_krb5_wrong
    } else if (*mech_type).length == (*crate::src::krb5::gssapi_krb5::gss_mech_iakerb).length
        && crate::stdlib::memcmp(
            (*mech_type).elements,
            (*crate::src::krb5::gssapi_krb5::gss_mech_iakerb).elements,
            (*mech_type).length as usize,
        ) == 0i32
    {
        mech_type = crate::src::krb5::gssapi_krb5::gss_mech_iakerb
    } else {
        *minor_status = 0u32;
        if (*context_handle).is_null() {
            crate::krb5_h::krb5_free_context(context);
        }
        return (1u32) << 16i32;
    }
    /* is this a new connection or not? */
    /*SUPPRESS 29*/
    if (*context_handle).is_null() {
        /* verify the credential, or use the default */
        /*SUPPRESS 29*/
        if claimant_cred_handle.is_null() {
            major_status =
                crate::src::krb5::gssapi_krb5::kg_get_defcred(minor_status, &mut defcred);
            if major_status != 0 && major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0
            {
                if (*context_handle).is_null() {
                    crate::krb5_h::krb5_free_context(context);
                }
                return major_status;
            }
            claimant_cred_handle = defcred
        }
        major_status = crate::src::krb5::acquire_cred::kg_cred_resolve(
            minor_status,
            context,
            claimant_cred_handle,
            target_name,
        );
        if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
            crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
            crate::src::krb5::rel_cred::krb5_gss_release_cred(&mut tmp_min_stat, &mut defcred);
            if (*context_handle).is_null() {
                crate::krb5_h::krb5_free_context(context);
            }
            return major_status;
        }
        cred = claimant_cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
        major_status = kg_new_connection(
            minor_status,
            cred,
            context_handle,
            target_name,
            mech_type,
            req_flags,
            time_req,
            input_chan_bindings,
            input_token,
            actual_mech_type,
            output_token,
            ret_flags,
            time_rec,
            context,
            exts,
        );
        k5_mutex_unlock(&mut (*cred).lock);
        crate::src::krb5::rel_cred::krb5_gss_release_cred(&mut tmp_min_stat, &mut defcred);
        if (*context_handle).is_null() {
            crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
            crate::krb5_h::krb5_free_context(context);
        } else {
            let ref mut fresh0 =
                (*(*context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec)).k5_context;
            *fresh0 = context
        }
    } else {
        /* mutual_auth doesn't care about the credentials */
        major_status = mutual_auth(
            minor_status,
            context_handle,
            target_name,
            mech_type,
            req_flags,
            time_req,
            input_chan_bindings,
            input_token,
            actual_mech_type,
            output_token,
            ret_flags,
            time_rec,
            context,
        )
        /* If context_handle is now NO_CONTEXT, mutual_auth called
        delete_sec_context, which would've zapped the krb5 context
        too.  */
    }
    return major_status;
}
#[no_mangle]

pub static mut kg_kdc_flag_mutex: crate::k5_thread_h::k5_mutex_t = crate::stdlib::pthread_mutex_t {
    __data: {
        let mut init = crate::stdlib::__pthread_mutex_s {
            __lock: 0i32,
            __count: 0u32,
            __owner: 0i32,
            __nusers: 0u32,
            __kind: 0i32,
            __spins: 0i16,
            __elision: 0i16,
            __list: {
                let mut init = crate::stdlib::__pthread_internal_list {
                    __prev: 0 as *mut crate::stdlib::__pthread_internal_list,
                    __next: 0 as *mut crate::stdlib::__pthread_internal_list,
                };
                init
            },
        };
        init
    },
};

static mut kdc_flag: i32 = 0i32;
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_init_context(
    mut ctxp: *mut crate::krb5_h::krb5_context,
) -> crate::krb5_h::krb5_error_code {
    let mut err: crate::krb5_h::krb5_error_code = 0;
    let mut is_kdc: i32 = 0;
    err = crate::src::krb5::gssapi_krb5::gss_krb5int_initialize_library()
        as crate::krb5_h::krb5_error_code;
    if err != 0 {
        return err;
    }
    k5_mutex_lock(&mut kg_kdc_flag_mutex);
    is_kdc = kdc_flag;
    k5_mutex_unlock(&mut kg_kdc_flag_mutex);
    if is_kdc != 0 {
        return crate::k5_int_h::krb5int_init_context_kdc(ctxp);
    }
    return crate::krb5_h::krb5_init_context(ctxp);
}
/* naming_exts.c */
/* s4u_gss_glue.c */
/*
 * These take unglued krb5-mech-specific contexts.
 */
#[no_mangle]

pub unsafe extern "C" fn krb5int_gss_use_kdc_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    _desired_mech: crate::gssapi_h::gss_OID,
    _desired_object: crate::gssapi_h::gss_OID,
    mut _value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut err: crate::gssapi_h::OM_uint32 = 0;
    *minor_status = 0u32;
    err = crate::src::krb5::gssapi_krb5::gss_krb5int_initialize_library();
    if err != 0 {
        return err;
    }
    k5_mutex_lock(&mut kg_kdc_flag_mutex);
    kdc_flag = 1i32;
    k5_mutex_unlock(&mut kg_kdc_flag_mutex);
    return 0u32;
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_init_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut claimant_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut mech_type: crate::gssapi_h::gss_OID,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut actual_mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut exts = crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec {
        iakerb: crate::gssapiP_krb5_h::C2RustUnnamed_0 {
            conv: 0 as *mut crate::krb5_h::krb5_data,
            verified: 0,
        },
    };
    crate::stdlib::memset(
        &mut exts as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec>(),
    );
    return krb5_gss_init_sec_context_ext(
        minor_status,
        claimant_cred_handle,
        context_handle,
        target_name,
        mech_type,
        req_flags,
        time_req,
        input_chan_bindings,
        input_token,
        actual_mech_type,
        output_token,
        ret_flags,
        time_rec,
        &mut exts,
    );
}
