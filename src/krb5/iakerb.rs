use ::libc;

pub mod k5_thread_h {

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

    use crate::src::krb5::iakerb::byteswap_h::__bswap_16;

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
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]

    pub unsafe extern "C" fn k5calloc(
        mut nmemb: crate::stddef_h::size_t,
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        let mut ptr = 0 as *mut libc::c_void;
        /* Allocate at least one byte since zero-byte allocs may return NULL. */
        ptr = crate::stdlib::calloc(
            if nmemb != 0 { nmemb } else { 1usize },
            if size != 0 { size } else { 1usize },
        );
        *code = if ptr.is_null() { 12i32 } else { 0i32 };
        return ptr;
    }
    /* Allocate zeroed memory; set *code to 0 on success or ENOMEM on failure. */
    #[inline]

    pub unsafe extern "C" fn k5alloc(
        mut size: crate::stddef_h::size_t,
        mut code: *mut crate::krb5_h::krb5_error_code,
    ) -> *mut libc::c_void {
        return k5calloc(1usize, size, code);
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

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub mod byteswap_h {
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
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::pthread_mutex_t;
pub use crate::stdlib::ssize_t;

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
pub use crate::k5_int_h::_krb5_iakerb_finished;
pub use crate::k5_int_h::_krb5_iakerb_header;
pub use crate::k5_int_h::_krb5_kt;
pub use crate::k5_int_h::_krb5_kt_ops;
pub use crate::k5_int_h::_krb5_os_context;
pub use crate::k5_int_h::ccselect_module_handle;
pub use crate::k5_int_h::decode_krb5_error;
pub use crate::k5_int_h::decode_krb5_iakerb_finished;
pub use crate::k5_int_h::decode_krb5_iakerb_header;
pub use crate::k5_int_h::derived_key;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::encode_krb5_iakerb_finished;
pub use crate::k5_int_h::encode_krb5_iakerb_header;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_authdata_export_authdata;
pub use crate::k5_int_h::krb5_free_iakerb_finished;
pub use crate::k5_int_h::krb5_free_iakerb_header;
pub use crate::k5_int_h::krb5_iakerb_finished;
pub use crate::k5_int_h::krb5_iakerb_header;
pub use crate::k5_int_h::krb5_key_st;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::krb5_sendto_kdc;
pub use crate::k5_int_h::localauth_module_handle;
pub use crate::k5_int_h::plugin_interface;
pub use crate::k5_int_h::plugin_mapping;
pub use crate::k5_int_h::CANONHOST_FALLBACK;
pub use crate::k5_int_h::CANONHOST_FALSE;
pub use crate::k5_int_h::CANONHOST_TRUE;
pub use crate::k5_platform_h::C2RustUnnamed_5;
pub use crate::k5_plugin_h::plugin_dir_handle;
pub use crate::k5_plugin_h::plugin_file_handle;
pub use crate::k5_thread_h::k5_mutex_t;
pub use crate::k5_thread_h::k5_os_mutex;
pub use crate::k5_thread_h::k5_os_mutex_unlock;
pub use crate::krb5_h::_krb5_address;
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
pub use crate::krb5_h::_krb5_get_init_creds_opt;
pub use crate::krb5_h::_krb5_init_creds_context;
pub use crate::krb5_h::_krb5_keyblock;
pub use crate::krb5_h::_krb5_prompt;
pub use crate::krb5_h::_krb5_ticket;
pub use crate::krb5_h::_krb5_ticket_times;
pub use crate::krb5_h::_krb5_tkt_creds_context;
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
pub use crate::krb5_h::krb5_ccache;
pub use crate::krb5_h::krb5_checksum;
pub use crate::krb5_h::krb5_cksumtype;
pub use crate::krb5_h::krb5_clear_error_message;
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
pub use crate::krb5_h::krb5_free_authdata;
pub use crate::krb5_h::krb5_free_checksum_contents;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_creds;
pub use crate::krb5_h::krb5_free_data;
pub use crate::krb5_h::krb5_free_data_contents;
pub use crate::krb5_h::krb5_free_error;
pub use crate::krb5_h::krb5_get_credentials;
pub use crate::krb5_h::krb5_get_init_creds_opt;
pub use crate::krb5_h::krb5_get_init_creds_opt_alloc;
pub use crate::krb5_h::krb5_get_init_creds_opt_free;
pub use crate::krb5_h::krb5_get_init_creds_opt_set_out_ccache;
pub use crate::krb5_h::krb5_get_init_creds_opt_set_tkt_life;
pub use crate::krb5_h::krb5_init_creds_context;
pub use crate::krb5_h::krb5_init_creds_free;
pub use crate::krb5_h::krb5_init_creds_get_times;
pub use crate::krb5_h::krb5_init_creds_init;
pub use crate::krb5_h::krb5_init_creds_set_keytab;
pub use crate::krb5_h::krb5_init_creds_set_password;
pub use crate::krb5_h::krb5_init_creds_step;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_make_checksum;
pub use crate::krb5_h::krb5_k_verify_checksum;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_keyusage;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_mk_error;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_preauthtype;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_prompter_fct;
pub use crate::krb5_h::krb5_rc_st;
pub use crate::krb5_h::krb5_rcache;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timeofday;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_tkt_creds_context;
pub use crate::krb5_h::krb5_tkt_creds_free;
pub use crate::krb5_h::krb5_tkt_creds_get_times;
pub use crate::krb5_h::krb5_tkt_creds_init;
pub use crate::krb5_h::krb5_tkt_creds_step;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::krb5_h::krb5_ui_4;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::iakerb::k5_int_h::empty_data;
pub use crate::src::krb5::iakerb::k5_int_h::k5alloc;
pub use crate::src::krb5::iakerb::k5_int_h::k5calloc;
pub use crate::src::krb5::iakerb::k5_int_h::make_data;
pub use crate::src::krb5::iakerb::k5_int_h::ts_incr;
pub use crate::src::krb5::iakerb::k5_platform_h::store_16_be;
pub use crate::src::krb5::iakerb::k5_thread_h::k5_mutex_unlock;

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
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_name_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapiP_krb5_h::C2RustUnnamed_0;
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_ext_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
pub use crate::gssapi_ext_h::gss_buffer_set_desc_struct;
pub use crate::gssapi_ext_h::gss_buffer_set_t;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc;
pub use crate::gssapi_ext_h::gss_iov_buffer_desc_struct;
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
pub use crate::gssapi_h::gss_cred_id_t;
pub use crate::gssapi_h::gss_cred_usage_t;
pub use crate::gssapi_h::gss_ctx_id_struct;
pub use crate::gssapi_h::gss_ctx_id_t;
pub use crate::gssapi_h::gss_int32;
pub use crate::gssapi_h::gss_name_t;
pub use crate::gssapi_h::gss_qop_t;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::mglueP_h::gss_cred_id_struct;
pub use crate::mglueP_h::gss_name_struct;
pub use crate::src::generic::util_token::gssint_g_make_token_header;
pub use crate::src::generic::util_token::gssint_g_token_size;
pub use crate::src::generic::util_token::gssint_g_verify_token_header;
pub use crate::src::krb5::accept_sec_context::krb5_gss_accept_sec_context_ext;
pub use crate::src::krb5::acquire_cred::iakerb_gss_acquire_cred;
pub use crate::src::krb5::acquire_cred::kg_cred_resolve;
pub use crate::src::krb5::acquire_cred::kg_cred_set_initial_refresh;
pub use crate::src::krb5::acquire_cred::kg_cred_time_to_refresh;
pub use crate::src::krb5::context_time::krb5_gss_context_time;
pub use crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context;
pub use crate::src::krb5::export_sec_context::krb5_gss_export_sec_context;

pub use crate::src::krb5::gssapi_krb5::krb5_gss_inquire_sec_context_by_oid;
pub use crate::src::krb5::gssapi_krb5::krb5_gss_set_sec_context_option;
pub use crate::src::krb5::iakerb::byteswap_h::__bswap_16;
pub use crate::src::krb5::iakerb::gssapi_alloc_h::gssalloc_malloc;
pub use crate::src::krb5::import_sec_context::krb5_gss_import_sec_context;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_sec_context_ext;
pub use crate::src::krb5::inq_context::krb5_gss_inquire_context;
pub use crate::src::krb5::k5seal::krb5_gss_get_mic;
pub use crate::src::krb5::k5seal::krb5_gss_wrap;
pub use crate::src::krb5::k5sealiov::krb5_gss_get_mic_iov;
pub use crate::src::krb5::k5sealiov::krb5_gss_get_mic_iov_length;
pub use crate::src::krb5::k5sealiov::krb5_gss_wrap_iov;
pub use crate::src::krb5::k5sealiov::krb5_gss_wrap_iov_length;
pub use crate::src::krb5::k5unseal::krb5_gss_unwrap;
pub use crate::src::krb5::k5unseal::krb5_gss_verify_mic;
pub use crate::src::krb5::k5unsealiov::krb5_gss_unwrap_iov;
pub use crate::src::krb5::k5unsealiov::krb5_gss_verify_mic_iov;
pub use crate::src::krb5::prf::krb5_gss_pseudo_random;
pub use crate::src::krb5::process_context_token::krb5_gss_process_context_token;
pub use crate::src::krb5::rel_cred::krb5_gss_release_cred;
pub use crate::src::krb5::wrap_size_limit::krb5_gss_wrap_size_limit;
pub use crate::src::mechglue::g_rel_buffer::gss_release_buffer;

extern "C" {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /*
     * Copyright 2009  by the Massachusetts Institute of Technology.
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
    /*
     * IAKERB implementation
     */

}

pub type iakerb_ctx_id_t = *mut iakerb_ctx_id_rec;

pub type iakerb_ctx_id_rec = _iakerb_ctx_id_rec;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _iakerb_ctx_id_rec {
    pub magic: crate::krb5_h::krb5_magic,
    pub k5c: crate::krb5_h::krb5_context,
    pub defcred: crate::gssapi_h::gss_cred_id_t,
    pub state: iakerb_state,
    pub icc: crate::krb5_h::krb5_init_creds_context,
    pub tcc: crate::krb5_h::krb5_tkt_creds_context,
    pub gssc: crate::gssapi_h::gss_ctx_id_t,
    pub conv: crate::krb5_h::krb5_data,
    pub count: u32,
    pub initiate: i32,
    pub established: i32,
    pub gic_opts: *mut crate::krb5_h::krb5_get_init_creds_opt,
}

pub type iakerb_state = u32;
/* hand-off to normal GSS AP-REQ exchange */
/* acquiring ticket with TGT */

pub const IAKERB_AP_REQ: iakerb_state = 2;
/* acquiring ticket with initial creds */

pub const IAKERB_TGS_REQ: iakerb_state = 1;

pub const IAKERB_AS_REQ: iakerb_state = 0;
/*
 * Release an IAKERB context
 */

unsafe extern "C" fn iakerb_release_context(mut ctx: iakerb_ctx_id_t) {
    let mut tmp: crate::gssapi_h::OM_uint32 = 0;
    if ctx.is_null() {
        return;
    }
    crate::src::krb5::rel_cred::krb5_gss_release_cred(&mut tmp, &mut (*ctx).defcred);
    crate::krb5_h::krb5_init_creds_free((*ctx).k5c, (*ctx).icc);
    crate::krb5_h::krb5_tkt_creds_free((*ctx).k5c, (*ctx).tcc);
    crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context(
        &mut tmp,
        &mut (*ctx).gssc,
        0 as crate::gssapi_h::gss_buffer_t,
    );
    crate::krb5_h::krb5_free_data_contents((*ctx).k5c, &mut (*ctx).conv);
    crate::krb5_h::krb5_get_init_creds_opt_free((*ctx).k5c, (*ctx).gic_opts);
    crate::krb5_h::krb5_free_context((*ctx).k5c);
    crate::stdlib::free(ctx as *mut libc::c_void);
}
/*
 * Create a IAKERB-FINISHED structure containing a checksum of
 * the entire IAKERB exchange.
 */
#[no_mangle]

pub unsafe extern "C" fn iakerb_make_finished(
    mut context: crate::krb5_h::krb5_context,
    mut key: crate::krb5_h::krb5_key,
    mut conv: *const crate::krb5_h::krb5_data,
    mut finished: *mut *mut crate::krb5_h::krb5_data,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut iaf = crate::k5_int_h::krb5_iakerb_finished {
        checksum: crate::krb5_h::krb5_checksum {
            magic: 0,
            checksum_type: 0,
            length: 0,
            contents: 0 as *mut crate::krb5_h::krb5_octet,
        },
    };
    *finished = 0 as *mut crate::krb5_h::krb5_data;
    crate::stdlib::memset(
        &mut iaf as *mut crate::k5_int_h::krb5_iakerb_finished as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::k5_int_h::krb5_iakerb_finished>(),
    );
    if key.is_null() {
        return -(1765328375 as isize) as crate::krb5_h::krb5_error_code;
    }
    code = crate::krb5_h::krb5_k_make_checksum(context, 0i32, key, 42i32, conv, &mut iaf.checksum);
    if code != 0i32 {
        return code;
    }
    code = crate::k5_int_h::encode_krb5_iakerb_finished(&mut iaf, finished);
    crate::krb5_h::krb5_free_checksum_contents(context, &mut iaf.checksum);
    return code;
}
/*
 * Verify a IAKERB-FINISHED structure submitted by the initiator
 */
#[no_mangle]

pub unsafe extern "C" fn iakerb_verify_finished(
    mut context: crate::krb5_h::krb5_context,
    mut key: crate::krb5_h::krb5_key,
    mut conv: *const crate::krb5_h::krb5_data,
    mut finished: *const crate::krb5_h::krb5_data,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut iaf = 0 as *mut crate::k5_int_h::krb5_iakerb_finished;
    let mut valid = 0u32;
    if key.is_null() {
        return -(1765328375 as isize) as crate::krb5_h::krb5_error_code;
    }
    code = crate::k5_int_h::decode_krb5_iakerb_finished(finished, &mut iaf);
    if code != 0i32 {
        return code;
    }
    code = crate::krb5_h::krb5_k_verify_checksum(
        context,
        key,
        42i32,
        conv,
        &mut (*iaf).checksum,
        &mut valid,
    );
    if code == 0i32 && valid == 0u32 {
        code = -(1765328353 as isize) as crate::krb5_h::krb5_error_code
    }
    crate::k5_int_h::krb5_free_iakerb_finished(context, iaf);
    return code;
}
/*
 * Save a token for future checksumming.
 */

unsafe extern "C" fn iakerb_save_token(
    mut ctx: iakerb_ctx_id_t,
    token: crate::gssapi_h::gss_buffer_t,
) -> crate::krb5_h::krb5_error_code {
    let mut p = 0 as *mut i8;
    p = crate::stdlib::realloc(
        (*ctx).conv.data as *mut libc::c_void,
        ((*ctx).conv.length as usize).wrapping_add((*token).length),
    ) as *mut i8;
    if p.is_null() {
        return 12i32;
    }
    crate::stdlib::memcpy(
        p.offset((*ctx).conv.length as isize) as *mut libc::c_void,
        (*token).value,
        (*token).length,
    );
    (*ctx).conv.data = p;
    (*ctx).conv.length = ((*ctx).conv.length as usize).wrapping_add((*token).length) as u32;
    return 0i32;
}
/*
 * Parse a token into IAKERB-HEADER and KRB-KDC-REQ/REP
 */

unsafe extern "C" fn iakerb_parse_token(
    mut ctx: iakerb_ctx_id_t,
    mut initialContextToken: i32,
    token: crate::gssapi_h::gss_buffer_t,
    mut realm: *mut crate::krb5_h::krb5_data,
    mut cookie: *mut *mut crate::krb5_h::krb5_data,
    mut request: *mut crate::krb5_h::krb5_data,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut iah = 0 as *mut crate::k5_int_h::krb5_iakerb_header;
    let mut bodysize: u32 = 0;
    let mut lenlen: u32 = 0;
    let mut length: i32 = 0;
    let mut ptr = 0 as *mut u8;
    let mut flags = 0i32;
    let mut data = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    if token.is_null() || (*token).length == 0usize {
        code = -(1765328194 as isize) as crate::krb5_h::krb5_error_code
    } else {
        if initialContextToken != 0 {
            flags |= 0x1i32
        }
        ptr = (*token).value as *mut u8;
        code = crate::src::generic::util_token::gssint_g_verify_token_header(
            crate::src::krb5::gssapi_krb5::gss_mech_iakerb as *const crate::gssapi_h::gss_OID_desc,
            &mut bodysize,
            &mut ptr,
            0x501i32,
            (*token).length as u32,
            flags,
        );
        if !(code != 0i32) {
            data.data = ptr as *mut i8;
            let fresh0 = bodysize;
            bodysize = bodysize.wrapping_sub(1);
            if fresh0 == 0u32 || {
                let fresh1 = ptr;
                ptr = ptr.offset(1);
                (*fresh1 as i32) != 0x30i32
            } {
                /* SEQUENCE */
                code = 1859794438i32
            } else {
                length = crate::src::mechglue::g_glue::gssint_get_der_length(
                    &mut ptr,
                    bodysize,
                    &mut lenlen,
                );
                if length < 0i32 || bodysize.wrapping_sub(lenlen) < length as u32 {
                    code = -(1765328194 as isize) as crate::krb5_h::krb5_error_code
                } else {
                    data.length = (1u32).wrapping_add(lenlen).wrapping_add(length as u32);
                    ptr = ptr.offset(length as isize);
                    bodysize = bodysize.wrapping_sub(lenlen.wrapping_add(length as u32));
                    code = crate::k5_int_h::decode_krb5_iakerb_header(&mut data, &mut iah);
                    if !(code != 0i32) {
                        if !realm.is_null() {
                            *realm = (*iah).target_realm;
                            (*iah).target_realm.data = 0 as *mut i8
                        }
                        if !cookie.is_null() {
                            *cookie = (*iah).cookie;
                            (*iah).cookie = 0 as *mut crate::krb5_h::krb5_data
                        }
                        (*request).data = ptr as *mut i8;
                        (*request).length = bodysize;
                        if (*request).data.offset((*request).length as isize)
                            == ((*token).value as *mut i8).offset((*token).length as isize)
                        {
                        } else {
                            crate::stdlib::__assert_fail(b"request->data + request->length == (char *)token->value + token->length\x00"
                                              as *const u8 as
                                              *const i8,
                                          b"iakerb.c\x00" as *const u8 as
                                              *const i8,
                                          233u32,
                                          (*::std::mem::transmute::<&[u8; 117],
                                                                    &[i8; 117]>(b"krb5_error_code iakerb_parse_token(iakerb_ctx_id_t, int, const gss_buffer_t, krb5_data *, krb5_data **, krb5_data *)\x00")).as_ptr());
                        }
                    }
                }
            }
        }
    }
    crate::k5_int_h::krb5_free_iakerb_header((*ctx).k5c, iah);
    return code;
}
/*
 * Create a token from IAKERB-HEADER and KRB-KDC-REQ/REP
 */

unsafe extern "C" fn iakerb_make_token(
    mut ctx: iakerb_ctx_id_t,
    mut realm: *mut crate::krb5_h::krb5_data,
    mut cookie: *mut crate::krb5_h::krb5_data,
    mut request: *mut crate::krb5_h::krb5_data,
    mut initialContextToken: i32,
    mut token: crate::gssapi_h::gss_buffer_t,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut iah = crate::k5_int_h::krb5_iakerb_header {
        target_realm: crate::krb5_h::krb5_data {
            magic: 0,
            length: 0,
            data: 0 as *mut i8,
        },
        cookie: 0 as *mut crate::krb5_h::krb5_data,
    };
    let mut data = 0 as *mut crate::krb5_h::krb5_data;
    let mut p = 0 as *mut i8;
    let mut tokenSize: u32 = 0;
    let mut q = 0 as *mut u8;
    (*token).value = 0 as *mut libc::c_void;
    (*token).length = 0usize;
    /*
     * Assemble the IAKERB-HEADER from the realm and cookie
     */
    iah.target_realm = *realm;
    iah.cookie = cookie;
    code = crate::k5_int_h::encode_krb5_iakerb_header(&mut iah, &mut data);
    if !(code != 0i32) {
        /*
         * Concatenate Kerberos request.
         */
        p = crate::stdlib::realloc(
            (*data).data as *mut libc::c_void,
            (*data).length.wrapping_add((*request).length) as usize,
        ) as *mut i8;
        if p.is_null() {
            code = 12i32
        } else {
            (*data).data = p;
            if (*request).length > 0u32 {
                crate::stdlib::memcpy(
                    (*data).data.offset((*data).length as isize) as *mut libc::c_void,
                    (*request).data as *const libc::c_void,
                    (*request).length as usize,
                );
            }
            (*data).length = (*data).length.wrapping_add((*request).length);
            if initialContextToken != 0 {
                tokenSize = crate::src::generic::util_token::gssint_g_token_size(
                    crate::src::krb5::gssapi_krb5::gss_mech_iakerb
                        as *const crate::gssapi_h::gss_OID_desc,
                    (*data).length,
                )
            } else {
                tokenSize = (2u32).wrapping_add((*data).length)
            }
            q = gssalloc_malloc(tokenSize as crate::stddef_h::size_t) as *mut u8;
            (*token).value = q as *mut libc::c_void;
            if q.is_null() {
                code = 12i32
            } else {
                (*token).length = tokenSize as crate::stddef_h::size_t;
                if initialContextToken != 0 {
                    crate::src::generic::util_token::gssint_g_make_token_header(
                        crate::src::krb5::gssapi_krb5::gss_mech_iakerb
                            as *const crate::gssapi_h::gss_OID_desc,
                        (*data).length,
                        &mut q,
                        0x501i32,
                    );
                } else {
                    store_16_be(0x501u32, q as *mut libc::c_void);
                    q = q.offset(2isize)
                }
                crate::stdlib::memcpy(
                    q as *mut libc::c_void,
                    (*data).data as *const libc::c_void,
                    (*data).length as usize,
                );
                q = q.offset((*data).length as isize);
                if q == ((*token).value as *mut u8).offset((*token).length as isize) {
                } else {
                    crate::stdlib::__assert_fail(b"q == (unsigned char *)token->value + token->length\x00"
                                      as *const u8 as *const i8,
                                  b"iakerb.c\x00" as *const u8 as
                                      *const i8,
                                  308u32,
                                  (*::std::mem::transmute::<&[u8; 109],
                                                            &[i8; 109]>(b"krb5_error_code iakerb_make_token(iakerb_ctx_id_t, krb5_data *, krb5_data *, krb5_data *, int, gss_buffer_t)\x00")).as_ptr());
                }
            }
        }
    }
    crate::krb5_h::krb5_free_data((*ctx).k5c, data);
    return code;
}
/*
 * Parse the IAKERB token in input_token and send the contained KDC
 * request to the KDC for the realm.
 *
 * Wrap the KDC reply in output_token.
 */

unsafe extern "C" fn iakerb_acceptor_step(
    mut ctx: iakerb_ctx_id_t,
    mut initialContextToken: i32,
    input_token: crate::gssapi_h::gss_buffer_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut request = empty_data();
    let mut reply = empty_data();
    let mut realm = empty_data();
    let mut tmp: crate::gssapi_h::OM_uint32 = 0;
    let mut tcp_only: i32 = 0;
    let mut use_master: i32 = 0;
    let mut kdc_code: crate::krb5_h::krb5_ui_4 = 0;
    (*output_token).length = 0usize;
    (*output_token).value = 0 as *mut libc::c_void;
    if (*ctx).count >= (16i32 + 10i32) as u32 {
        code = -(1765328228 as isize) as crate::krb5_h::krb5_error_code
    } else {
        code = iakerb_parse_token(
            ctx,
            initialContextToken,
            input_token,
            &mut realm,
            0 as *mut *mut crate::krb5_h::krb5_data,
            &mut request,
        );
        if !(code != 0i32) {
            if realm.length == 0u32 || request.length == 0u32 {
                code = -(1765328194 as isize) as crate::krb5_h::krb5_error_code
            } else {
                code = iakerb_save_token(ctx, input_token);
                if !(code != 0i32) {
                    tcp_only = 0i32;
                    loop {
                        if !(tcp_only <= 1i32) {
                            current_block = 15345278821338558188;
                            break;
                        }
                        use_master = 0i32;
                        code = crate::k5_int_h::krb5_sendto_kdc(
                            (*ctx).k5c,
                            &mut request,
                            &mut realm,
                            &mut reply,
                            &mut use_master,
                            tcp_only,
                        );
                        if !(code == 0i32
                            && (!(&mut reply as *mut crate::krb5_h::krb5_data).is_null()
                                && reply.length != 0
                                && *reply.data.offset(0isize) as i32 & !(0x20i32)
                                    == 30i32 | 0x40i32))
                        {
                            current_block = 15345278821338558188;
                            break;
                        }
                        let mut error = 0 as *mut crate::krb5_h::krb5_error;
                        code = crate::k5_int_h::decode_krb5_error(&mut reply, &mut error);
                        if code != 0i32 {
                            current_block = 16875317906461707385;
                            break;
                        }
                        kdc_code = (*error).error;
                        crate::krb5_h::krb5_free_error((*ctx).k5c, error);
                        if !(kdc_code == 52u32) {
                            current_block = 15345278821338558188;
                            break;
                        }
                        crate::krb5_h::krb5_free_data_contents((*ctx).k5c, &mut reply);
                        reply = empty_data();
                        tcp_only += 1
                    }
                    match current_block {
                        16875317906461707385 => {}
                        _ => {
                            if code as isize == -(1765328228 as isize)
                                || code as isize == -(1765328230 as isize)
                            {
                                let mut error_0 = crate::krb5_h::krb5_error {
                                    magic: 0,
                                    ctime: 0,
                                    cusec: 0,
                                    susec: 0,
                                    stime: 0,
                                    error: 0,
                                    client: 0 as *mut crate::krb5_h::krb5_principal_data,
                                    server: 0 as *mut crate::krb5_h::krb5_principal_data,
                                    text: crate::krb5_h::krb5_data {
                                        magic: 0,
                                        length: 0,
                                        data: 0 as *mut i8,
                                    },
                                    e_data: crate::krb5_h::krb5_data {
                                        magic: 0,
                                        length: 0,
                                        data: 0 as *mut i8,
                                    },
                                };
                                crate::stdlib::memset(
                                    &mut error_0 as *mut crate::krb5_h::krb5_error
                                        as *mut libc::c_void,
                                    0i32,
                                    ::std::mem::size_of::<crate::krb5_h::krb5_error>(),
                                );
                                if code as isize == -(1765328228 as isize) {
                                    error_0.error = 86u32
                                } else if code as isize == -(1765328230 as isize) {
                                    error_0.error = 85u32
                                }
                                code = crate::krb5_h::krb5_mk_error(
                                    (*ctx).k5c,
                                    &mut error_0,
                                    &mut reply,
                                );
                                if code != 0i32 {
                                    current_block = 16875317906461707385;
                                } else {
                                    current_block = 14832935472441733737;
                                }
                            } else if code != 0i32 {
                                current_block = 16875317906461707385;
                            } else {
                                current_block = 14832935472441733737;
                            }
                            match current_block {
                                16875317906461707385 => {}
                                _ => {
                                    code = iakerb_make_token(
                                        ctx,
                                        &mut realm,
                                        0 as *mut crate::krb5_h::krb5_data,
                                        &mut reply,
                                        0i32,
                                        output_token,
                                    );
                                    if !(code != 0i32) {
                                        code = iakerb_save_token(ctx, output_token);
                                        if !(code != 0i32) {
                                            (*ctx).count = (*ctx).count.wrapping_add(1)
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
    if code != 0i32 {
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmp, output_token);
    }
    /* request is a pointer into input_token, no need to free */
    crate::krb5_h::krb5_free_data_contents((*ctx).k5c, &mut realm);
    crate::krb5_h::krb5_free_data_contents((*ctx).k5c, &mut reply);
    return code;
}
/*
 * Initialise the krb5_init_creds context for the IAKERB context
 */

unsafe extern "C" fn iakerb_init_creds_ctx(
    mut ctx: iakerb_ctx_id_t,
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut time_req: crate::gssapi_h::OM_uint32,
) -> crate::krb5_h::krb5_error_code {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    if (*cred).iakerb_mech() as i32 == 0i32 {
        code = 22i32
    } else {
        if !(*cred).name.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"cred->name != NULL\x00" as *const u8 as
                              *const i8,
                          b"iakerb.c\x00" as *const u8 as *const i8,
                          428u32,
                          (*::std::mem::transmute::<&[u8; 86],
                                                    &[i8; 86]>(b"krb5_error_code iakerb_init_creds_ctx(iakerb_ctx_id_t, krb5_gss_cred_id_t, OM_uint32)\x00")).as_ptr());
        }
        if !(*(*cred).name).princ.is_null() {
        } else {
            crate::stdlib::__assert_fail(b"cred->name->princ != NULL\x00" as *const u8 as
                              *const i8,
                          b"iakerb.c\x00" as *const u8 as *const i8,
                          429u32,
                          (*::std::mem::transmute::<&[u8; 86],
                                                    &[i8; 86]>(b"krb5_error_code iakerb_init_creds_ctx(iakerb_ctx_id_t, krb5_gss_cred_id_t, OM_uint32)\x00")).as_ptr());
        }
        code = crate::krb5_h::krb5_get_init_creds_opt_alloc((*ctx).k5c, &mut (*ctx).gic_opts);
        if !(code != 0i32) {
            if time_req != 0u32 && time_req != 0xffffffffu32 {
                crate::krb5_h::krb5_get_init_creds_opt_set_tkt_life(
                    (*ctx).gic_opts,
                    time_req as crate::krb5_h::krb5_deltat,
                );
            }
            code = crate::krb5_h::krb5_get_init_creds_opt_set_out_ccache(
                (*ctx).k5c,
                (*ctx).gic_opts,
                (*cred).ccache,
            );
            if !(code != 0i32) {
                code = crate::krb5_h::krb5_init_creds_init(
                    (*ctx).k5c,
                    (*(*cred).name).princ,
                    None,
                    0 as *mut libc::c_void,
                    0i32,
                    (*ctx).gic_opts,
                    &mut (*ctx).icc,
                );
                if !(code != 0i32) {
                    if !(*cred).password.is_null() {
                        code = crate::krb5_h::krb5_init_creds_set_password(
                            (*ctx).k5c,
                            (*ctx).icc,
                            (*cred).password,
                        )
                    } else if !(*cred).client_keytab.is_null() {
                        code = crate::krb5_h::krb5_init_creds_set_keytab(
                            (*ctx).k5c,
                            (*ctx).icc,
                            (*cred).client_keytab,
                        )
                    } else {
                        code = -(1765328203 as isize) as crate::krb5_h::krb5_error_code
                    }
                    (code) != 0i32;
                }
            }
        }
    }
    return code;
}
/*
 * Initialise the krb5_tkt_creds context for the IAKERB context
 */

unsafe extern "C" fn iakerb_tkt_creds_ctx(
    mut ctx: iakerb_ctx_id_t,
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut name: crate::gssapiP_krb5_h::krb5_gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut creds = crate::krb5_h::krb5_creds {
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
    let mut now: crate::krb5_h::krb5_timestamp = 0;
    if !(*cred).name.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"cred->name != NULL\x00" as *const u8 as
                          *const i8,
                      b"iakerb.c\x00" as *const u8 as *const i8,
                      483u32,
                      (*::std::mem::transmute::<&[u8; 102],
                                                &[i8; 102]>(b"krb5_error_code iakerb_tkt_creds_ctx(iakerb_ctx_id_t, krb5_gss_cred_id_t, krb5_gss_name_t, OM_uint32)\x00")).as_ptr());
    }
    if !(*(*cred).name).princ.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"cred->name->princ != NULL\x00" as *const u8 as
                          *const i8,
                      b"iakerb.c\x00" as *const u8 as *const i8,
                      484u32,
                      (*::std::mem::transmute::<&[u8; 102],
                                                &[i8; 102]>(b"krb5_error_code iakerb_tkt_creds_ctx(iakerb_ctx_id_t, krb5_gss_cred_id_t, krb5_gss_name_t, OM_uint32)\x00")).as_ptr());
    }
    crate::stdlib::memset(
        &mut creds as *mut crate::krb5_h::krb5_creds as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::krb5_h::krb5_creds>(),
    );
    creds.client = (*(*cred).name).princ;
    creds.server = (*name).princ;
    if time_req != 0u32 && time_req != 0xffffffffu32 {
        code = crate::krb5_h::krb5_timeofday((*ctx).k5c, &mut now);
        if code != 0i32 {
            current_block = 14846341633757719488;
        } else {
            creds.times.endtime = ts_incr(now, time_req as crate::krb5_h::krb5_deltat);
            current_block = 11812396948646013369;
        }
    } else {
        current_block = 11812396948646013369;
    }
    match current_block {
        11812396948646013369 => {
            if !(*(*cred).name).ad_context.is_null() {
                code = crate::k5_int_h::krb5_authdata_export_authdata(
                    (*ctx).k5c,
                    (*(*cred).name).ad_context,
                    0x2i32,
                    &mut creds.authdata,
                );
                if code != 0i32 {
                    current_block = 14846341633757719488;
                } else {
                    current_block = 10599921512955367680;
                }
            } else {
                current_block = 10599921512955367680;
            }
            match current_block {
                14846341633757719488 => {}
                _ => {
                    code = crate::krb5_h::krb5_tkt_creds_init(
                        (*ctx).k5c,
                        (*cred).ccache,
                        &mut creds,
                        0i32,
                        &mut (*ctx).tcc,
                    );
                    (code) != 0i32;
                }
            }
        }
        _ => {}
    }
    crate::krb5_h::krb5_free_authdata((*ctx).k5c, creds.authdata);
    return code;
}
/*
 * Parse the IAKERB token in input_token and process the KDC
 * response.
 *
 * Emit the next KDC request, if any, in output_token.
 */

unsafe extern "C" fn iakerb_initiator_step(
    mut ctx: iakerb_ctx_id_t,
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut name: crate::gssapiP_krb5_h::krb5_gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    input_token: crate::gssapi_h::gss_buffer_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut code = 0i32;
    let mut in_0 = empty_data();
    let mut out = empty_data();
    let mut realm = empty_data();
    let mut cookie = 0 as *mut crate::krb5_h::krb5_data;
    let mut tmp: crate::gssapi_h::OM_uint32 = 0;
    let mut flags = 0u32;
    let mut times = crate::krb5_h::krb5_ticket_times {
        authtime: 0,
        starttime: 0,
        endtime: 0,
        renew_till: 0,
    };
    (*output_token).length = 0usize;
    (*output_token).value = 0 as *mut libc::c_void;
    if !input_token.is_null() {
        code = iakerb_parse_token(
            ctx,
            0i32,
            input_token,
            0 as *mut crate::krb5_h::krb5_data,
            &mut cookie,
            &mut in_0,
        );
        if code != 0i32 {
            current_block = 3793315583556022455;
        } else {
            code = iakerb_save_token(ctx, input_token);
            if code != 0i32 {
                current_block = 3793315583556022455;
            } else {
                current_block = 2868539653012386629;
            }
        }
    } else {
        current_block = 2868539653012386629;
    }
    match current_block {
        2868539653012386629 => {
            match (*ctx).state {
                0 => {
                    if (*ctx).icc.is_null() {
                        code = iakerb_init_creds_ctx(ctx, cred, time_req);
                        if code != 0i32 {
                            current_block = 3793315583556022455;
                        } else {
                            current_block = 9606288038608642794;
                        }
                    } else {
                        current_block = 9606288038608642794;
                    }
                    match current_block {
                        3793315583556022455 => {}
                        _ => {
                            code = crate::krb5_h::krb5_init_creds_step(
                                (*ctx).k5c,
                                (*ctx).icc,
                                &mut in_0,
                                &mut out,
                                &mut realm,
                                &mut flags,
                            );
                            if code != 0i32 {
                                if (*cred).have_tgt != 0 {
                                    /* We were trying to refresh; keep going with current creds. */
                                    (*ctx).state = IAKERB_TGS_REQ;
                                    crate::krb5_h::krb5_clear_error_message((*ctx).k5c);
                                    current_block = 4775909272756257391;
                                } else {
                                    current_block = 3793315583556022455;
                                }
                            } else if flags & 0x1u32 == 0 {
                                crate::krb5_h::krb5_init_creds_get_times(
                                    (*ctx).k5c,
                                    (*ctx).icc,
                                    &mut times,
                                );
                                crate::src::krb5::acquire_cred::kg_cred_set_initial_refresh(
                                    (*ctx).k5c,
                                    cred,
                                    &mut times,
                                );
                                (*cred).expire = times.endtime;
                                crate::krb5_h::krb5_init_creds_free((*ctx).k5c, (*ctx).icc);
                                (*ctx).icc = 0 as crate::krb5_h::krb5_init_creds_context;
                                (*ctx).state = IAKERB_TGS_REQ;
                                current_block = 4775909272756257391;
                            } else {
                                current_block = 12997042908615822766;
                            }
                            match current_block {
                                3793315583556022455 => {}
                                12997042908615822766 => {}
                                _ => {
                                    in_0 = empty_data();
                                    current_block = 14728543630564123267;
                                }
                            }
                        }
                    }
                }
                1 => {
                    current_block = 14728543630564123267;
                }
                2 | _ => {
                    current_block = 12997042908615822766;
                }
            }
            match current_block {
                3793315583556022455 => {}
                _ => {
                    match current_block {
                        14728543630564123267 =>
                        /* Done with AS request; fall through to TGS request. */
                        {
                            if (*ctx).tcc.is_null() {
                                code = iakerb_tkt_creds_ctx(ctx, cred, name, time_req);
                                if code != 0i32 {
                                    current_block = 3793315583556022455;
                                } else {
                                    current_block = 14434620278749266018;
                                }
                            } else {
                                current_block = 14434620278749266018;
                            }
                            match current_block {
                                3793315583556022455 => {}
                                _ => {
                                    code = crate::krb5_h::krb5_tkt_creds_step(
                                        (*ctx).k5c,
                                        (*ctx).tcc,
                                        &mut in_0,
                                        &mut out,
                                        &mut realm,
                                        &mut flags,
                                    );
                                    if code != 0i32 {
                                        current_block = 3793315583556022455;
                                    } else if flags & 0x1u32 == 0 {
                                        crate::krb5_h::krb5_tkt_creds_get_times(
                                            (*ctx).k5c,
                                            (*ctx).tcc,
                                            &mut times,
                                        );
                                        (*cred).expire = times.endtime;
                                        crate::krb5_h::krb5_tkt_creds_free((*ctx).k5c, (*ctx).tcc);
                                        (*ctx).tcc = 0 as crate::krb5_h::krb5_tkt_creds_context;
                                        (*ctx).state = IAKERB_AP_REQ;
                                        current_block = 12997042908615822766;
                                    } else {
                                        current_block = 12997042908615822766;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        3793315583556022455 => {}
                        _ =>
                        /* Done with TGS request; fall through to AP request. */
                        {
                            if out.length != 0u32 {
                                if (*ctx).state != IAKERB_AP_REQ {
                                } else {
                                    crate::stdlib::__assert_fail(b"ctx->state != IAKERB_AP_REQ\x00"
                                                      as *const u8 as
                                                      *const i8,
                                                  b"iakerb.c\x00" as *const u8
                                                      as *const i8,
                                                  610u32,
                                                  (*::std::mem::transmute::<&[u8; 137],
                                                                            &[i8; 137]>(b"krb5_error_code iakerb_initiator_step(iakerb_ctx_id_t, krb5_gss_cred_id_t, krb5_gss_name_t, OM_uint32, const gss_buffer_t, gss_buffer_t)\x00")).as_ptr());
                                }
                                code = iakerb_make_token(
                                    ctx,
                                    &mut realm,
                                    cookie,
                                    &mut out,
                                    (input_token == 0 as crate::gssapi_h::gss_buffer_t) as i32,
                                    output_token,
                                );
                                if !(code != 0i32) {
                                    /* Save the token for generating a future checksum */
                                    code = iakerb_save_token(ctx, output_token);
                                    if !(code != 0i32) {
                                        (*ctx).count = (*ctx).count.wrapping_add(1)
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
        crate::src::mechglue::g_rel_buffer::gss_release_buffer(&mut tmp, output_token);
    }
    crate::krb5_h::krb5_free_data((*ctx).k5c, cookie);
    crate::krb5_h::krb5_free_data_contents((*ctx).k5c, &mut out);
    crate::krb5_h::krb5_free_data_contents((*ctx).k5c, &mut realm);
    return code;
}
/*
 * Determine the starting IAKERB state for a context. If we already
 * have a ticket, we may not need to do IAKERB at all.
 */

unsafe extern "C" fn iakerb_get_initial_state(
    mut ctx: iakerb_ctx_id_t,
    mut cred: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut target: crate::gssapiP_krb5_h::krb5_gss_name_t,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut state: *mut iakerb_state,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
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
    let mut out_creds = 0 as *mut crate::krb5_h::krb5_creds;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    crate::stdlib::memset(
        &mut in_creds as *mut crate::krb5_h::krb5_creds as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::krb5_h::krb5_creds>(),
    );
    in_creds.client = (*(*cred).name).princ;
    in_creds.server = (*target).princ;
    if !(*(*cred).name).ad_context.is_null() {
        code = crate::k5_int_h::krb5_authdata_export_authdata(
            (*ctx).k5c,
            (*(*cred).name).ad_context,
            0x2i32,
            &mut in_creds.authdata,
        );
        if code != 0i32 {
            current_block = 11666045669424323853;
        } else {
            current_block = 10879442775620481940;
        }
    } else {
        current_block = 10879442775620481940;
    }
    match current_block {
        10879442775620481940 => {
            if time_req != 0u32 && time_req != 0xffffffffu32 {
                let mut now: crate::krb5_h::krb5_timestamp = 0;
                code = crate::krb5_h::krb5_timeofday((*ctx).k5c, &mut now);
                if code != 0i32 {
                    current_block = 11666045669424323853;
                } else {
                    in_creds.times.endtime = ts_incr(now, time_req as crate::krb5_h::krb5_deltat);
                    current_block = 7651349459974463963;
                }
            } else {
                current_block = 7651349459974463963;
            }
            match current_block {
                11666045669424323853 => {}
                _ =>
                /* Make an AS request if we have no creds or it's time to refresh them. */
                {
                    if (*cred).expire == 0i32
                        || crate::src::krb5::acquire_cred::kg_cred_time_to_refresh((*ctx).k5c, cred)
                            != 0
                    {
                        *state = IAKERB_AS_REQ;
                        code = 0i32
                    } else {
                        code = crate::krb5_h::krb5_get_credentials(
                            (*ctx).k5c,
                            2i32,
                            (*cred).ccache,
                            &mut in_creds,
                            &mut out_creds,
                        );
                        if code as isize == -(1765328243 as isize)
                            || code as isize == -(1765328184 as isize)
                        {
                            *state = if (*cred).have_tgt != 0 {
                                IAKERB_TGS_REQ as i32
                            } else {
                                IAKERB_AS_REQ as i32
                            } as iakerb_state;
                            code = 0i32
                        } else if code == 0i32 {
                            *state = IAKERB_AP_REQ;
                            crate::krb5_h::krb5_free_creds((*ctx).k5c, out_creds);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    crate::krb5_h::krb5_free_authdata((*ctx).k5c, in_creds.authdata);
    return code;
}
/*
 * Allocate and initialise an IAKERB context
 */

unsafe extern "C" fn iakerb_alloc_context(
    mut pctx: *mut iakerb_ctx_id_t,
    mut initiate: i32,
) -> crate::krb5_h::krb5_error_code {
    let mut ctx = 0 as *mut iakerb_ctx_id_rec;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    *pctx = 0 as iakerb_ctx_id_t;
    ctx = k5alloc(::std::mem::size_of::<iakerb_ctx_id_rec>(), &mut code) as iakerb_ctx_id_t;
    if !ctx.is_null() {
        (*ctx).defcred = 0 as crate::gssapi_h::gss_cred_id_t;
        (*ctx).magic = 39756048i32;
        (*ctx).state = IAKERB_AS_REQ;
        (*ctx).count = 0u32;
        (*ctx).initiate = initiate;
        (*ctx).established = 0i32;
        code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut (*ctx).k5c);
        if !(code != 0i32) {
            *pctx = ctx
        }
    }
    if code != 0i32 {
        iakerb_release_context(ctx);
    }
    return code;
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_delete_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut output_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut iakerb_ctx = *context_handle as iakerb_ctx_id_t;
    if !output_token.is_null() {
        (*output_token).length = 0usize;
        (*output_token).value = 0 as *mut libc::c_void
    }
    *minor_status = 0u32;
    *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t;
    iakerb_release_context(iakerb_ctx);
    return 0u32;
}

unsafe extern "C" fn iakerb_is_iakerb_token(
    token: crate::gssapi_h::gss_buffer_t,
) -> crate::krb5_h::krb5_boolean {
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut bodysize = (*token).length as u32;
    let mut ptr = (*token).value as *mut u8;
    code = crate::src::generic::util_token::gssint_g_verify_token_header(
        crate::src::krb5::gssapi_krb5::gss_mech_iakerb as *const crate::gssapi_h::gss_OID_desc,
        &mut bodysize,
        &mut ptr,
        0x501i32,
        (*token).length as u32,
        0i32,
    );
    return (code == 0i32) as crate::krb5_h::krb5_boolean;
}

unsafe extern "C" fn iakerb_make_exts(
    mut ctx: iakerb_ctx_id_t,
    mut exts: *mut crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec,
) {
    crate::stdlib::memset(
        exts as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec>(),
    );
    if (*ctx).conv.length != 0u32 {
        (*exts).iakerb.conv = &mut (*ctx).conv
    };
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_accept_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut verifier_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut src_name: *mut crate::gssapi_h::gss_name_t,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut delegated_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major_status = (13u32) << 16i32;
    let mut code: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = 0 as *mut iakerb_ctx_id_rec;
    let mut initialContextToken = (*context_handle == 0 as crate::gssapi_h::gss_ctx_id_t) as i32;
    if initialContextToken != 0 {
        code = iakerb_alloc_context(&mut ctx, 0i32) as crate::gssapi_h::OM_uint32;
        if code != 0u32 {
            current_block = 10280779239138868270;
        } else {
            current_block = 2473556513754201174;
        }
    } else {
        ctx = *context_handle as iakerb_ctx_id_t;
        current_block = 2473556513754201174;
    }
    match current_block {
        2473556513754201174 => {
            if iakerb_is_iakerb_token(input_token) != 0 {
                if !(*ctx).gssc.is_null() {
                    /* We shouldn't get an IAKERB token now. */
                    code = -(2045022960 as isize) as crate::gssapi_h::OM_uint32;
                    major_status = (9u32) << 16i32
                } else {
                    code = iakerb_acceptor_step(ctx, initialContextToken, input_token, output_token)
                        as crate::gssapi_h::OM_uint32;
                    if code == -(1765328194 as isize) as crate::gssapi_h::OM_uint32 {
                        major_status = (9u32) << 16i32
                    }
                    if !(code != 0u32) {
                        if initialContextToken != 0 {
                            *context_handle = ctx as crate::gssapi_h::gss_ctx_id_t;
                            ctx = 0 as iakerb_ctx_id_t
                        }
                        if !src_name.is_null() {
                            *src_name = 0 as crate::gssapi_h::gss_name_t
                        }
                        if !mech_type.is_null() {
                            *mech_type = crate::src::krb5::gssapi_krb5::gss_mech_iakerb
                        }
                        if !ret_flags.is_null() {
                            *ret_flags = 0u32
                        }
                        if !time_rec.is_null() {
                            *time_rec = 0u32
                        }
                        if !delegated_cred_handle.is_null() {
                            *delegated_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t
                        }
                        major_status = ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32
                    }
                }
            } else {
                let mut exts = crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec {
                    iakerb: crate::gssapiP_krb5_h::C2RustUnnamed_0 {
                        conv: 0 as *mut crate::krb5_h::krb5_data,
                        verified: 0,
                    },
                };
                iakerb_make_exts(ctx, &mut exts);
                major_status =
                    crate::src::krb5::accept_sec_context::krb5_gss_accept_sec_context_ext(
                        &mut code,
                        &mut (*ctx).gssc,
                        verifier_cred_handle,
                        input_token,
                        input_chan_bindings,
                        src_name,
                        0 as *mut crate::gssapi_h::gss_OID,
                        output_token,
                        ret_flags,
                        time_rec,
                        delegated_cred_handle,
                        &mut exts,
                    );
                if major_status == 0u32 {
                    (*ctx).established = 1i32
                }
                if !mech_type.is_null() {
                    *mech_type = crate::src::krb5::gssapi_krb5::gss_mech_krb5
                }
            }
        }
        _ => {}
    }
    if initialContextToken != 0 && major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        iakerb_release_context(ctx);
        *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t
    }
    *minor_status = code;
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_init_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut claimant_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut target_name: crate::gssapi_h::gss_name_t,
    mut _mech_type: crate::gssapi_h::gss_OID,
    mut req_flags: crate::gssapi_h::OM_uint32,
    mut time_req: crate::gssapi_h::OM_uint32,
    mut input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut actual_mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut major_status = (13u32) << 16i32;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut ctx = 0 as *mut iakerb_ctx_id_rec;
    let mut kcred = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_cred_id_rec;
    let mut kname = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_name_rec;
    let mut cred_locked = 0u32;
    let mut initialContextToken = (*context_handle == 0 as crate::gssapi_h::gss_ctx_id_t) as i32;
    if initialContextToken != 0 {
        code = iakerb_alloc_context(&mut ctx, 1i32);
        if code != 0i32 {
            *minor_status = code as crate::gssapi_h::OM_uint32;
            current_block = 12791752257761306173;
        } else if claimant_cred_handle.is_null() {
            major_status = crate::src::krb5::acquire_cred::iakerb_gss_acquire_cred(
                minor_status,
                0 as crate::gssapi_h::gss_name_t,
                0xffffffffu32,
                0 as crate::gssapi_h::gss_OID_set,
                1i32,
                &mut (*ctx).defcred,
                0 as *mut crate::gssapi_h::gss_OID_set,
                0 as *mut crate::gssapi_h::OM_uint32,
            );
            if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                current_block = 12791752257761306173;
            } else {
                claimant_cred_handle = (*ctx).defcred;
                current_block = 15976848397966268834;
            }
        } else {
            current_block = 15976848397966268834;
        }
    } else {
        ctx = *context_handle as iakerb_ctx_id_t;
        if claimant_cred_handle.is_null() {
            claimant_cred_handle = (*ctx).defcred
        }
        current_block = 15976848397966268834;
    }
    match current_block {
        15976848397966268834 => {
            kname = target_name as crate::gssapiP_krb5_h::krb5_gss_name_t;
            major_status = crate::src::krb5::acquire_cred::kg_cred_resolve(
                minor_status,
                (*ctx).k5c,
                claimant_cred_handle,
                target_name,
            );
            if !(major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0) {
                cred_locked = 1u32;
                kcred = claimant_cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
                major_status = (13u32) << 16i32;
                if initialContextToken != 0 {
                    code = iakerb_get_initial_state(ctx, kcred, kname, time_req, &mut (*ctx).state);
                    if code != 0i32 {
                        *minor_status = code as crate::gssapi_h::OM_uint32;
                        current_block = 12791752257761306173;
                    } else {
                        *context_handle = ctx as crate::gssapi_h::gss_ctx_id_t;
                        current_block = 15345278821338558188;
                    }
                } else {
                    current_block = 15345278821338558188;
                }
                match current_block {
                    12791752257761306173 => {}
                    _ => {
                        if (*ctx).state != IAKERB_AP_REQ {
                            /* We need to do IAKERB. */
                            code = iakerb_initiator_step(
                                ctx,
                                kcred,
                                kname,
                                time_req,
                                input_token,
                                output_token,
                            );
                            if code as isize == -(1765328194 as isize) {
                                major_status = (9u32) << 16i32
                            }
                            if code != 0i32 {
                                *minor_status = code as crate::gssapi_h::OM_uint32;
                                current_block = 12791752257761306173;
                            } else {
                                current_block = 9520865839495247062;
                            }
                        } else {
                            current_block = 9520865839495247062;
                        }
                        match current_block {
                            12791752257761306173 => {}
                            _ => {
                                if (*ctx).state == IAKERB_AP_REQ {
                                    let mut exts = crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec {
                                        iakerb: crate::gssapiP_krb5_h::C2RustUnnamed_0 {
                                            conv: 0 as *mut crate::krb5_h::krb5_data,
                                            verified: 0,
                                        },
                                    };
                                    if cred_locked != 0 {
                                        k5_mutex_unlock(&mut (*kcred).lock);
                                        cred_locked = 0u32
                                    }
                                    iakerb_make_exts(ctx, &mut exts);
                                    if (*ctx).gssc.is_null() {
                                        input_token = 0 as crate::gssapi_h::gss_buffer_t
                                    }
                                    /* IAKERB is finished, or we skipped to Kerberos directly. */
                                    major_status =
                                        crate::src::krb5::init_sec_context::krb5_gss_init_sec_context_ext(minor_status,
                                                                      kcred as
                                                                          crate::gssapi_h::gss_cred_id_t,
                                                                      &mut (*ctx).gssc,
                                                                      target_name,
                                                                      crate::src::krb5::gssapi_krb5::gss_mech_iakerb,
                                                                      req_flags,
                                                                      time_req,
                                                                      input_chan_bindings,
                                                                      input_token,
                                                                      0 as
                                                                          *mut crate::gssapi_h::gss_OID,
                                                                      output_token,
                                                                      ret_flags,
                                                                      time_rec,
                                                                      &mut exts);
                                    if major_status == 0u32 {
                                        (*ctx).established = 1i32
                                    }
                                    if !actual_mech_type.is_null() {
                                        *actual_mech_type =
                                            crate::src::krb5::gssapi_krb5::gss_mech_krb5
                                    }
                                } else {
                                    if !actual_mech_type.is_null() {
                                        *actual_mech_type =
                                            crate::src::krb5::gssapi_krb5::gss_mech_iakerb
                                    }
                                    if !ret_flags.is_null() {
                                        *ret_flags = 0u32
                                    }
                                    if !time_rec.is_null() {
                                        *time_rec = 0u32
                                    }
                                    major_status =
                                        ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if cred_locked != 0 {
        k5_mutex_unlock(&mut (*kcred).lock);
    }
    if initialContextToken != 0 && major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
        iakerb_release_context(ctx);
        *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t
    }
    return major_status;
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_unwrap(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5unseal::krb5_gss_unwrap(
        minor_status,
        (*ctx).gssc,
        input_message_buffer,
        output_message_buffer,
        conf_state,
        qop_state,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_wrap(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut input_message_buffer: crate::gssapi_h::gss_buffer_t,
    mut conf_state: *mut i32,
    mut output_message_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5seal::krb5_gss_wrap(
        minor_status,
        (*ctx).gssc,
        conf_req_flag,
        qop_req,
        input_message_buffer,
        conf_state,
        output_message_buffer,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_process_context_token(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    token_buffer: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (9u32) << 16i32;
    }
    return crate::src::krb5::process_context_token::krb5_gss_process_context_token(
        minor_status,
        (*ctx).gssc,
        token_buffer,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_context_time(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::context_time::krb5_gss_context_time(
        minor_status,
        (*ctx).gssc,
        time_rec,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_export_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut interprocess_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut maj: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = *context_handle as iakerb_ctx_id_t;
    /* We don't currently support exporting partially established contexts. */
    if (*ctx).established == 0 {
        return (16u32) << 16i32;
    }
    maj = crate::src::krb5::export_sec_context::krb5_gss_export_sec_context(
        minor_status,
        &mut (*ctx).gssc,
        interprocess_token,
    );
    if (*ctx).gssc.is_null() {
        iakerb_release_context(ctx);
        *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t
    }
    return maj;
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_import_sec_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut interprocess_token: crate::gssapi_h::gss_buffer_t,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut maj: crate::gssapi_h::OM_uint32 = 0;
    let mut tmpmin: crate::gssapi_h::OM_uint32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut gssc = 0 as *mut crate::gssapi_h::gss_ctx_id_struct;
    let mut kctx = 0 as *mut crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
    let mut ctx = 0 as *mut iakerb_ctx_id_rec;
    maj = crate::src::krb5::import_sec_context::krb5_gss_import_sec_context(
        minor_status,
        interprocess_token,
        &mut gssc,
    );
    if maj != 0u32 {
        return maj;
    }
    kctx = gssc as crate::gssapiP_krb5_h::krb5_gss_ctx_id_t;
    if (*kctx).established() == 0 {
        /* We don't currently support importing partially established
         * contexts. */
        crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context(
            &mut tmpmin,
            &mut gssc,
            0 as crate::gssapi_h::gss_buffer_t,
        );
        return (13u32) << 16i32;
    }
    code = iakerb_alloc_context(&mut ctx, (*kctx).initiate() as i32);
    if code != 0i32 {
        crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context(
            &mut tmpmin,
            &mut gssc,
            0 as crate::gssapi_h::gss_buffer_t,
        );
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    (*ctx).gssc = gssc;
    (*ctx).established = 1i32;
    *context_handle = ctx as crate::gssapi_h::gss_ctx_id_t;
    return 0u32;
}
/* LEAN_CLIENT */
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_inquire_context(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut src_name: *mut crate::gssapi_h::gss_name_t,
    mut targ_name: *mut crate::gssapi_h::gss_name_t,
    mut lifetime_rec: *mut crate::gssapi_h::OM_uint32,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut ctx_flags: *mut crate::gssapi_h::OM_uint32,
    mut initiate: *mut i32,
    mut opened: *mut i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ret: crate::gssapi_h::OM_uint32 = 0;
    let mut ctx = context_handle as iakerb_ctx_id_t;
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
        *mech_type = crate::src::krb5::gssapi_krb5::gss_mech_iakerb
    }
    if !ctx_flags.is_null() {
        *ctx_flags = 0u32
    }
    if !initiate.is_null() {
        *initiate = (*ctx).initiate
    }
    if !opened.is_null() {
        *opened = (*ctx).established
    }
    if (*ctx).gssc.is_null() {
        return 0u32;
    }
    ret = crate::src::krb5::inq_context::krb5_gss_inquire_context(
        minor_status,
        (*ctx).gssc,
        src_name,
        targ_name,
        lifetime_rec,
        mech_type,
        ctx_flags,
        initiate,
        opened,
    );
    if (*ctx).established == 0 {
        /* Report IAKERB as the mech OID until the context is established. */
        if !mech_type.is_null() {
            *mech_type = crate::src::krb5::gssapi_krb5::gss_mech_iakerb
        }
        /* We don't support exporting partially-established contexts. */
        if !ctx_flags.is_null() {
            *ctx_flags &= !(256i32) as u32
        }
    }
    return ret;
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_wrap_size_limit(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut req_output_size: crate::gssapi_h::OM_uint32,
    mut max_input_size: *mut crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::wrap_size_limit::krb5_gss_wrap_size_limit(
        minor_status,
        (*ctx).gssc,
        conf_req_flag,
        qop_req,
        req_output_size,
        max_input_size,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_get_mic(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut message_buffer: crate::gssapi_h::gss_buffer_t,
    mut message_token: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5seal::krb5_gss_get_mic(
        minor_status,
        (*ctx).gssc,
        qop_req,
        message_buffer,
        message_token,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_verify_mic(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut msg_buffer: crate::gssapi_h::gss_buffer_t,
    mut token_buffer: crate::gssapi_h::gss_buffer_t,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5unseal::krb5_gss_verify_mic(
        minor_status,
        (*ctx).gssc,
        msg_buffer,
        token_buffer,
        qop_state,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_inquire_sec_context_by_oid(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    context_handle: crate::gssapi_h::gss_ctx_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    mut data_set: *mut crate::gssapi_ext_h::gss_buffer_set_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (16u32) << 16i32;
    }
    return crate::src::krb5::gssapi_krb5::krb5_gss_inquire_sec_context_by_oid(
        minor_status,
        (*ctx).gssc,
        desired_object,
        data_set,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_set_sec_context_option(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    desired_object: crate::gssapi_h::gss_OID,
    value: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = *context_handle as iakerb_ctx_id_t;
    if ctx.is_null() || (*ctx).gssc.is_null() {
        return (16u32) << 16i32;
    }
    return crate::src::krb5::gssapi_krb5::krb5_gss_set_sec_context_option(
        minor_status,
        &mut (*ctx).gssc,
        desired_object,
        value,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_wrap_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5sealiov::krb5_gss_wrap_iov(
        minor_status,
        (*ctx).gssc,
        conf_req_flag,
        qop_req,
        conf_state,
        iov,
        iov_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_unwrap_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_state: *mut i32,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5unsealiov::krb5_gss_unwrap_iov(
        minor_status,
        (*ctx).gssc,
        conf_state,
        qop_state,
        iov,
        iov_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_wrap_iov_length(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut conf_req_flag: i32,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut conf_state: *mut i32,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5sealiov::krb5_gss_wrap_iov_length(
        minor_status,
        (*ctx).gssc,
        conf_req_flag,
        qop_req,
        conf_state,
        iov,
        iov_count,
    );
}
/* LEAN_CLIENT */
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_pseudo_random(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut prf_key: i32,
    prf_in: crate::gssapi_h::gss_buffer_t,
    mut desired_output_len: crate::stdlib::ssize_t,
    mut prf_out: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::prf::krb5_gss_pseudo_random(
        minor_status,
        (*ctx).gssc,
        prf_key,
        prf_in,
        desired_output_len,
        prf_out,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_get_mic_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5sealiov::krb5_gss_get_mic_iov(
        minor_status,
        (*ctx).gssc,
        qop_req,
        iov,
        iov_count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_verify_mic_iov(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_state: *mut crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5unsealiov::krb5_gss_verify_mic_iov(
        minor_status,
        (*ctx).gssc,
        qop_state,
        iov,
        iov_count,
    );
}
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
/* s4u_gss_glue.c */
/*
 * These take unglued krb5-mech-specific contexts.
 */
/* _GSS_STATIC_LINK */
/* For error message handling.  */
/* Returns a shared string, not a private copy!  */
/* Prefix concatenated with Kerberos encryption type */
/* IAKERB */
/*
 * Transfer contents of a krb5_data to a gss_buffer and invalidate the source
 * On unix, this is a simple pointer copy
 * On windows, memory is reallocated and copied.
 */
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
/* minor_status */
/* input_cred_handle */
/* input_usage */
/* desired_mech */
/* overwrite_cred */
/* default_cred */
/* cred_store */
/* elements_stored */
/* cred_usage_stored */
#[no_mangle]

pub unsafe extern "C" fn iakerb_gss_get_mic_iov_length(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: crate::gssapi_h::gss_ctx_id_t,
    mut qop_req: crate::gssapi_h::gss_qop_t,
    mut iov: *mut crate::gssapi_ext_h::gss_iov_buffer_desc,
    mut iov_count: i32,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = context_handle as iakerb_ctx_id_t;
    if (*ctx).gssc.is_null() {
        return (8u32) << 16i32;
    }
    return crate::src::krb5::k5sealiov::krb5_gss_get_mic_iov_length(
        minor_status,
        (*ctx).gssc,
        qop_req,
        iov,
        iov_count,
    );
}
