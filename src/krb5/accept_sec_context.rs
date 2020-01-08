use ::libc;

pub mod k5_thread_h {

    #[inline]

    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut crate::k5_thread_h::k5_mutex_t) -> i32 {
        return crate::k5_thread_h::k5_os_mutex_init(m);
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

    pub unsafe extern "C" fn load_16_le(mut cvp: *const libc::c_void) -> u16 {
        let mut p = cvp as *const u8;
        return (*(p as *const crate::k5_platform_h::C2RustUnnamed_5)).i;
    }
    #[inline]

    pub unsafe extern "C" fn load_32_le(mut cvp: *const libc::c_void) -> u32 {
        let mut p = cvp as *const u8;
        return (*(p as *const crate::k5_platform_h::C2RustUnnamed_6)).i;
    }

    use crate::src::krb5::accept_sec_context::byteswap_h::__bswap_16;
    use crate::src::krb5::accept_sec_context::byteswap_h::__bswap_32;
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

    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}

pub mod gssapiP_krb5_h {

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

    use crate::src::krb5::accept_sec_context::k5_int_h::empty_data;

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
pub use crate::k5_int_h::decode_krb5_ap_req;
pub use crate::k5_int_h::derived_key;
pub use crate::k5_int_h::dns_canonhost;
pub use crate::k5_int_h::encode_krb5_ticket;
pub use crate::k5_int_h::hostrealm_module_handle;
pub use crate::k5_int_h::k5_tls_vtable_st;
pub use crate::k5_int_h::kdb5_dal_handle;
pub use crate::k5_int_h::krb5_auth_con_get_authdata_context;
pub use crate::k5_int_h::krb5_auth_con_set_authdata_context;
pub use crate::k5_int_h::krb5_auth_con_setpermetypes;
pub use crate::k5_int_h::krb5_authdata_context;
pub use crate::k5_int_h::krb5_free_ap_req;
pub use crate::k5_int_h::krb5_key_st;
pub use crate::k5_int_h::krb5_preauth_context;
pub use crate::k5_int_h::krb5_preauth_context_st;
pub use crate::k5_int_h::krb5_rd_req_decoded;
pub use crate::k5_int_h::krb5int_access;
pub use crate::k5_int_h::krb5int_accessor;
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
pub use crate::k5_thread_h::k5_os_mutex_destroy;
pub use crate::k5_thread_h::k5_os_mutex_init;
pub use crate::k5_thread_h::k5_os_mutex_unlock;
pub use crate::krb5_h::_krb5_address;
pub use crate::krb5_h::_krb5_ap_req;
pub use crate::krb5_h::_krb5_auth_context;
pub use crate::krb5_h::_krb5_authdata;
pub use crate::krb5_h::_krb5_authenticator;
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
pub use crate::krb5_h::krb5_ap_req;
pub use crate::krb5_h::krb5_auth_con_free;
pub use crate::krb5_h::krb5_auth_con_getauthenticator;
pub use crate::krb5_h::krb5_auth_con_getflags;
pub use crate::krb5_h::krb5_auth_con_getkey_k;
pub use crate::krb5_h::krb5_auth_con_getlocalseqnumber;
pub use crate::krb5_h::krb5_auth_con_getrecvsubkey_k;
pub use crate::krb5_h::krb5_auth_con_getremoteseqnumber;
pub use crate::krb5_h::krb5_auth_con_getsendsubkey_k;
pub use crate::krb5_h::krb5_auth_con_init;
pub use crate::krb5_h::krb5_auth_con_setaddrs;
pub use crate::krb5_h::krb5_auth_con_setflags;
pub use crate::krb5_h::krb5_auth_con_setrcache;
pub use crate::krb5_h::krb5_auth_context;
pub use crate::krb5_h::krb5_authdata;
pub use crate::krb5_h::krb5_authdatatype;
pub use crate::krb5_h::krb5_authenticator;
pub use crate::krb5_h::krb5_boolean;
pub use crate::krb5_h::krb5_c_checksum_length;
pub use crate::krb5_h::krb5_cc_close;
pub use crate::krb5_h::krb5_cc_destroy;
pub use crate::krb5_h::krb5_cc_initialize;
pub use crate::krb5_h::krb5_cc_new_unique;
pub use crate::krb5_h::krb5_cc_store_cred;
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
pub use crate::krb5_h::krb5_free_authenticator;
pub use crate::krb5_h::krb5_free_context;
pub use crate::krb5_h::krb5_free_data;
pub use crate::krb5_h::krb5_free_data_contents;
pub use crate::krb5_h::krb5_free_principal;
pub use crate::krb5_h::krb5_free_tgt_creds;
pub use crate::krb5_h::krb5_int16;
pub use crate::krb5_h::krb5_int32;
pub use crate::krb5_h::krb5_k_free_key;
pub use crate::krb5_h::krb5_k_verify_checksum;
pub use crate::krb5_h::krb5_kdc_req;
pub use crate::krb5_h::krb5_key;
pub use crate::krb5_h::krb5_keyblock;
pub use crate::krb5_h::krb5_keytab;
pub use crate::krb5_h::krb5_keytab_entry;
pub use crate::krb5_h::krb5_keytab_entry_st;
pub use crate::krb5_h::krb5_keyusage;
pub use crate::krb5_h::krb5_kt_cursor;
pub use crate::krb5_h::krb5_kvno;
pub use crate::krb5_h::krb5_magic;
pub use crate::krb5_h::krb5_merge_authdata;
pub use crate::krb5_h::krb5_mk_error;
pub use crate::krb5_h::krb5_mk_rep;
pub use crate::krb5_h::krb5_msgtype;
pub use crate::krb5_h::krb5_octet;
pub use crate::krb5_h::krb5_pa_data;
pub use crate::krb5_h::krb5_pointer;
pub use crate::krb5_h::krb5_post_recv_fn;
pub use crate::krb5_h::krb5_pre_send_fn;
pub use crate::krb5_h::krb5_preauthtype;
pub use crate::krb5_h::krb5_principal;
pub use crate::krb5_h::krb5_principal_data;
pub use crate::krb5_h::krb5_prompt_type;
pub use crate::krb5_h::krb5_rc_st;
pub use crate::krb5_h::krb5_rcache;
pub use crate::krb5_h::krb5_rd_cred;
pub use crate::krb5_h::krb5_rd_rep_dce;
pub use crate::krb5_h::krb5_replay_data;
pub use crate::krb5_h::krb5_ticket;
pub use crate::krb5_h::krb5_ticket_times;
pub use crate::krb5_h::krb5_timeofday;
pub use crate::krb5_h::krb5_timestamp;
pub use crate::krb5_h::krb5_trace_callback;
pub use crate::krb5_h::krb5_trace_info;
pub use crate::krb5_h::krb5_transited;
pub use crate::krb5_h::krb5_ui_2;
pub use crate::krb5_h::krb5_ui_4;
pub use crate::krb5_h::krb5_us_timeofday;
pub use crate::profile_h::profile_t;
pub use crate::src::krb5::accept_sec_context::k5_int_h::empty_data;
pub use crate::src::krb5::accept_sec_context::k5_int_h::make_data;
pub use crate::src::krb5::accept_sec_context::k5_int_h::ts_delta;
pub use crate::src::krb5::accept_sec_context::k5_platform_h::load_16_be;
pub use crate::src::krb5::accept_sec_context::k5_platform_h::load_16_le;
pub use crate::src::krb5::accept_sec_context::k5_platform_h::load_32_be;
pub use crate::src::krb5::accept_sec_context::k5_platform_h::load_32_le;
pub use crate::src::krb5::accept_sec_context::k5_thread_h::k5_mutex_init;
pub use crate::src::krb5::accept_sec_context::k5_thread_h::k5_mutex_unlock;

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
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
pub use crate::gssapiP_krb5_h::krb5_gss_name_t;
pub use crate::gssapiP_krb5_h::C2RustUnnamed_0;
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_ext_rec;
pub use crate::gssapiP_krb5_h::_krb5_gss_ctx_id_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_ext_rec;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t;
pub use crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
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
pub use crate::src::generic::util_seqstate::gssint_g_seqstate_init;
pub use crate::src::generic::util_token::gssint_g_make_token_header;
pub use crate::src::generic::util_token::gssint_g_token_size;
pub use crate::src::generic::util_token::gssint_g_verify_token_header;
pub use crate::src::krb5::accept_sec_context::gssapiP_krb5_h::data_to_gss;
pub use crate::src::krb5::acquire_cred::kg_cred_resolve;
pub use crate::src::krb5::acquire_cred::krb5_gss_acquire_cred;
pub use crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_info;
pub use crate::src::krb5::disp_status::krb5_gss_save_error_string;
pub use crate::src::krb5::iakerb::iakerb_verify_finished;
pub use crate::src::krb5::init_sec_context::krb5_gss_init_context;
pub use crate::src::krb5::naming_exts::kg_acceptor_princ;
pub use crate::src::krb5::naming_exts::kg_duplicate_name;
pub use crate::src::krb5::naming_exts::kg_init_name;
pub use crate::src::krb5::naming_exts::kg_release_name;
pub use crate::src::krb5::rel_cred::krb5_gss_release_cred;
pub use crate::src::krb5::s4u_gss_glue::kg_compose_deleg_cred;
pub use crate::src::krb5::util_cksum::kg_checksum_channel_bindings;
pub use crate::src::krb5::util_crypt::kg_setup_keys;

pub use crate::src::krb5::accept_sec_context::byteswap_h::__bswap_16;
pub use crate::src::krb5::accept_sec_context::byteswap_h::__bswap_32;
pub use crate::src::krb5::accept_sec_context::gssapi_alloc_h::gssalloc_malloc;

unsafe extern "C" fn create_constrained_deleg_creds(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut verifier_cred_handle: crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut ticket: *mut crate::krb5_h::krb5_ticket,
    mut out_cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
    mut context: crate::krb5_h::krb5_context,
) -> crate::gssapi_h::OM_uint32 {
    let mut major_status: crate::gssapi_h::OM_uint32 = 0;
    let mut krb_creds = crate::krb5_h::krb5_creds {
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
    let mut data = 0 as *mut crate::krb5_h::krb5_data;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    if !out_cred.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"out_cred != NULL\x00" as *const u8 as
                          *const i8,
                      b"accept_sec_context.c\x00" as *const u8 as
                          *const i8,
                      127u32,
                      (*::std::mem::transmute::<&[u8; 125],
                                                &[i8; 125]>(b"OM_uint32 create_constrained_deleg_creds(OM_uint32 *, krb5_gss_cred_id_t, krb5_ticket *, krb5_gss_cred_id_t *, krb5_context)\x00")).as_ptr());
    }
    if (*verifier_cred_handle).usage == 0i32 {
    } else {
        crate::stdlib::__assert_fail(b"verifier_cred_handle->usage == GSS_C_BOTH\x00" as
                          *const u8 as *const i8,
                      b"accept_sec_context.c\x00" as *const u8 as
                          *const i8,
                      128u32,
                      (*::std::mem::transmute::<&[u8; 125],
                                                &[i8; 125]>(b"OM_uint32 create_constrained_deleg_creds(OM_uint32 *, krb5_gss_cred_id_t, krb5_ticket *, krb5_gss_cred_id_t *, krb5_context)\x00")).as_ptr());
    }
    crate::stdlib::memset(
        &mut krb_creds as *mut crate::krb5_h::krb5_creds as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<crate::krb5_h::krb5_creds>(),
    );
    krb_creds.client = (*(*ticket).enc_part2).client;
    krb_creds.server = (*ticket).server;
    krb_creds.keyblock = *(*(*ticket).enc_part2).session;
    krb_creds.ticket_flags = (*(*ticket).enc_part2).flags;
    krb_creds.times = (*(*ticket).enc_part2).times;
    krb_creds.magic = -(1760647408 as isize) as crate::krb5_h::krb5_magic;
    krb_creds.authdata = 0 as *mut *mut crate::krb5_h::krb5_authdata;
    code = crate::k5_int_h::encode_krb5_ticket(ticket, &mut data);
    if code != 0 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    krb_creds.ticket = *data;
    major_status = crate::src::krb5::s4u_gss_glue::kg_compose_deleg_cred(
        minor_status,
        verifier_cred_handle,
        &mut krb_creds,
        0xffffffffu32,
        out_cred,
        0 as *mut crate::gssapi_h::OM_uint32,
        context,
    );
    crate::krb5_h::krb5_free_data(context, data);
    return major_status;
}
/* Decode, decrypt and store the forwarded creds in the local ccache. */

unsafe extern "C" fn rd_and_store_for_creds(
    mut context: crate::krb5_h::krb5_context,
    mut auth_context: crate::krb5_h::krb5_auth_context,
    mut inbuf: *mut crate::krb5_h::krb5_data,
    mut out_cred: *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_t,
) -> crate::krb5_h::krb5_error_code {
    let mut current_block: u64;
    let mut creds = 0 as *mut *mut crate::krb5_h::krb5_creds;
    let mut retval: crate::krb5_h::krb5_error_code = 0;
    let mut ccache = 0 as crate::krb5_h::krb5_ccache;
    let mut cred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    let mut new_auth_ctx = 0 as crate::krb5_h::krb5_auth_context;
    let mut flags_org: crate::krb5_h::krb5_int32 = 0;
    retval = crate::krb5_h::krb5_auth_con_getflags(context, auth_context, &mut flags_org);
    if retval != 0 {
        return retval;
    }
    crate::krb5_h::krb5_auth_con_setflags(context, auth_context, 0i32);
    /*
     * By the time krb5_rd_cred is called here (after krb5_rd_req has been
     * called in krb5_gss_accept_sec_context), the "keyblock" field of
     * auth_context contains a pointer to the session key, and the
     * "recv_subkey" field might contain a session subkey.  Either of
     * these (the "recv_subkey" if it isn't NULL, otherwise the
     * "keyblock") might have been used to encrypt the encrypted part of
     * the KRB_CRED message that contains the forwarded credentials.  (The
     * Java Crypto and Security Implementation from the DSTC in Australia
     * always uses the session key.  But apparently it never negotiates a
     * subkey, so this code works fine against a JCSI client.)  Up to the
     * present, though, GSSAPI clients linked against the MIT code (which
     * is almost all GSSAPI clients) don't encrypt the KRB_CRED message at
     * all -- at this level.  So if the first call to krb5_rd_cred fails,
     * we should call it a second time with another auth context freshly
     * created by krb5_auth_con_init.  All of its keyblock fields will be
     * NULL, so krb5_rd_cred will assume that the KRB_CRED message is
     * unencrypted.  (The MIT code doesn't actually send the KRB_CRED
     * message in the clear -- the "authenticator" whose "checksum" ends up
     * containing the KRB_CRED message does get encrypted.)
     */
    if crate::krb5_h::krb5_rd_cred(
        context,
        auth_context,
        inbuf,
        &mut creds,
        0 as *mut crate::krb5_h::krb5_replay_data,
    ) != 0
    {
        retval = crate::krb5_h::krb5_auth_con_init(context, &mut new_auth_ctx);
        if retval != 0 {
            current_block = 4448131064135155392;
        } else {
            crate::krb5_h::krb5_auth_con_setflags(context, new_auth_ctx, 0i32);
            retval = crate::krb5_h::krb5_rd_cred(
                context,
                new_auth_ctx,
                inbuf,
                &mut creds,
                0 as *mut crate::krb5_h::krb5_replay_data,
            );
            if retval != 0 {
                current_block = 4448131064135155392;
            } else {
                current_block = 2868539653012386629;
            }
        }
    } else {
        current_block = 2868539653012386629;
    }
    match current_block {
        2868539653012386629 => {
            retval = crate::krb5_h::krb5_cc_new_unique(
                context,
                b"MEMORY\x00" as *const u8 as *const i8,
                0 as *const i8,
                &mut ccache,
            );
            if retval != 0 {
                ccache = 0 as crate::krb5_h::krb5_ccache
            } else {
                retval = crate::krb5_h::krb5_cc_initialize(
                    context,
                    ccache,
                    (**creds.offset(0isize)).client,
                );
                if !(retval != 0) {
                    retval =
                        crate::krb5_h::krb5_cc_store_cred(context, ccache, *creds.offset(0isize));
                    if !(retval != 0) {
                        /* generate a delegated credential handle */
                        if !out_cred.is_null() {
                            /* allocate memory for a cred_t... */
                            cred = crate::stdlib::malloc(::std::mem::size_of::<
                                crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
                            >())
                                as crate::gssapiP_krb5_h::krb5_gss_cred_id_t; /* out of memory? */
                            if cred.is_null() {
                                retval = 12i32
                            } else {
                                /* zero it out... */
                                crate::stdlib::memset(
                                    cred as *mut libc::c_void,
                                    0i32,
                                    ::std::mem::size_of::<
                                        crate::gssapiP_krb5_h::krb5_gss_cred_id_rec,
                                    >(),
                                );
                                retval = k5_mutex_init(&mut (*cred).lock);
                                if retval != 0 {
                                    crate::stdlib::free(cred as *mut libc::c_void);
                                    cred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t
                                } else {
                                    /* copy the client principle into it... */
                                    retval = crate::src::krb5::naming_exts::kg_init_name(
                                        context,
                                        (**creds.offset(0isize)).client,
                                        0 as *mut i8,
                                        0 as *mut i8,
                                        0 as crate::k5_int_h::krb5_authdata_context,
                                        0i32,
                                        &mut (*cred).name,
                                    ); /* out of memory? */
                                    if retval != 0 {
                                        crate::k5_thread_h::k5_os_mutex_destroy(&mut (*cred).lock); /* clean up memory on failure */
                                        retval = 12i32; /* we can't accept with this */
                                        crate::stdlib::free(cred as *mut libc::c_void);
                                        cred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t
                                    } else {
                                        (*cred).usage = 1i32;
                                        /* cred->name already set */
                                        (*cred).keytab = 0 as crate::krb5_h::krb5_keytab; /* no keytab associated with this... */
                                        (*cred).expire = (**creds.offset(0isize)).times.endtime; /* store the end time */
                                        (*cred).ccache = ccache; /* the ccache containing the credential */
                                        (*cred).set_destroy_ccache(1u32);
                                        ccache = 0 as crate::krb5_h::krb5_ccache
                                    }
                                }
                            }
                            /* cred takes ownership so don't destroy */
                        }
                    }
                }
            }
        }
        _ => {}
    }
    /* If there were errors, there might have been a memory leak
       if (!cred)
       if ((retval = krb5_cc_close(context, ccache)))
       goto cleanup;
    */
    if !creds.is_null() {
        crate::krb5_h::krb5_free_tgt_creds(context, creds); /* return credential */
    }
    if !ccache.is_null() {
        crate::krb5_h::krb5_cc_destroy(context, ccache);
    }
    if !out_cred.is_null() {
        *out_cred = cred
    }
    if !new_auth_ctx.is_null() {
        crate::krb5_h::krb5_auth_con_free(context, new_auth_ctx);
    }
    crate::krb5_h::krb5_auth_con_setflags(context, auth_context, flags_org);
    return retval;
}
/*
 * Performs third leg of DCE authentication
 */

unsafe extern "C" fn kg_accept_dce(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut context_handle: *mut crate::gssapi_h::gss_ctx_id_t,
    mut _verifier_cred_handle: crate::gssapi_h::gss_cred_id_t,
    mut input_token: crate::gssapi_h::gss_buffer_t,
    mut _input_chan_bindings: crate::gssapi_h::gss_channel_bindings_t,
    mut src_name: *mut crate::gssapi_h::gss_name_t,
    mut mech_type: *mut crate::gssapi_h::gss_OID,
    mut output_token: crate::gssapi_h::gss_buffer_t,
    mut ret_flags: *mut crate::gssapi_h::OM_uint32,
    mut time_rec: *mut crate::gssapi_h::OM_uint32,
    mut delegated_cred_handle: *mut crate::gssapi_h::gss_cred_id_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut now: crate::krb5_h::krb5_timestamp = 0;
    let mut name = 0 as crate::gssapiP_krb5_h::krb5_gss_name_t;
    let mut nonce = 0u32;
    let mut ap_rep = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut major_status = (13u32) << 16i32;
    (*output_token).length = 0usize;
    (*output_token).value = 0 as *mut libc::c_void;
    if !mech_type.is_null() {
        *mech_type = 0 as crate::gssapi_h::gss_OID
    }
    /* return a bogus cred handle */
    if !delegated_cred_handle.is_null() {
        *delegated_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t
    }
    ctx = *context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    code = crate::krb5_h::krb5_timeofday((*ctx).k5_context, &mut now);
    if code != 0i32 {
        major_status = (13u32) << 16i32
    } else {
        ap_rep.data = (*input_token).value as *mut i8;
        ap_rep.length = (*input_token).length as u32;
        code = crate::krb5_h::krb5_rd_rep_dce(
            (*ctx).k5_context,
            (*ctx).auth_context,
            &mut ap_rep,
            &mut nonce,
        );
        if code != 0i32 {
            major_status = (13u32) << 16i32
        } else {
            (*ctx).set_established(1u32);
            if !src_name.is_null() {
                code = crate::src::krb5::naming_exts::kg_duplicate_name(
                    (*ctx).k5_context,
                    (*ctx).there,
                    &mut name,
                );
                if code != 0 {
                    major_status = (13u32) << 16i32;
                    current_block = 9595826404651570249;
                } else {
                    *src_name = name as crate::gssapi_h::gss_name_t;
                    current_block = 15768484401365413375;
                }
            } else {
                current_block = 15768484401365413375;
            }
            match current_block {
                9595826404651570249 => {}
                _ => {
                    if !mech_type.is_null() {
                        *mech_type = (*ctx).mech_used
                    }
                    if !time_rec.is_null() {
                        *time_rec = (ts_delta((*ctx).krb_times.endtime, now)
                            + (*(*ctx).k5_context).clockskew)
                            as crate::gssapi_h::OM_uint32
                    }
                    /* Never return GSS_C_DELEG_FLAG since we don't support DCE credential
                     * delegation yet. */
                    if !ret_flags.is_null() {
                        *ret_flags = (*ctx).gss_flags & !(1i32) as u32
                    }
                    *minor_status = 0u32;
                    return 0u32;
                }
            }
        }
    }
    /* real failure code follows */
    crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context(
        minor_status,
        &mut ctx as *mut *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec
            as *mut crate::gssapi_h::gss_ctx_id_t,
        0 as crate::gssapi_h::gss_buffer_t,
    );
    *context_handle = 0 as crate::gssapi_h::gss_ctx_id_t;
    *minor_status = code as crate::gssapi_h::OM_uint32;
    return major_status;
}

unsafe extern "C" fn kg_process_extension(
    mut context: crate::krb5_h::krb5_context,
    mut auth_context: crate::krb5_h::krb5_auth_context,
    mut ext_type: i32,
    mut ext_data: *mut crate::krb5_h::krb5_data,
    mut exts: crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t,
) -> crate::krb5_h::krb5_error_code {
    let mut code = 0i32;
    if !exts.is_null() {
    } else {
        crate::stdlib::__assert_fail(b"exts != NULL\x00" as *const u8 as *const i8,
                      b"accept_sec_context.c\x00" as *const u8 as
                          *const i8,
                      388u32,
                      (*::std::mem::transmute::<&[u8; 108],
                                                &[i8; 108]>(b"krb5_error_code kg_process_extension(krb5_context, krb5_auth_context, int, krb5_data *, krb5_gss_ctx_ext_t)\x00")).as_ptr());
    }
    match ext_type {
        1 => {
            if (*exts).iakerb.conv.is_null() {
                code = -(1765328344 as isize) as crate::krb5_h::krb5_error_code
            /* XXX */
            } else {
                let mut key = 0 as *mut crate::k5_int_h::krb5_key_st;
                code =
                    crate::krb5_h::krb5_auth_con_getrecvsubkey_k(context, auth_context, &mut key);
                if !(code != 0i32) {
                    code = crate::src::krb5::iakerb::iakerb_verify_finished(
                        context,
                        key,
                        (*exts).iakerb.conv,
                        ext_data,
                    );
                    if code == 0i32 {
                        (*exts).iakerb.verified = 1i32
                    }
                    crate::krb5_h::krb5_k_free_key(context, key);
                }
            }
        }
        _ => {}
    }
    return code;
}

unsafe extern "C" fn kg_accept_krb5(
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
    mut exts: crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut current_block: u64;
    let mut context = 0 as *mut crate::k5_int_h::_krb5_context;
    let mut ptr = 0 as *mut u8;
    let mut ptr2 = 0 as *mut u8;
    let mut sptr = 0 as *mut i8;
    let mut tmp: crate::gssapi_h::OM_uint32 = 0;
    let mut md5len: crate::stddef_h::size_t = 0;
    let mut cred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
    let mut ap_rep = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut ap_req = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut i: u32 = 0;
    let mut code: crate::krb5_h::krb5_error_code = 0;
    let mut addr = crate::krb5_h::krb5_address {
        magic: 0,
        addrtype: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut paddr = 0 as *mut crate::krb5_h::krb5_address;
    let mut authdat = 0 as *mut crate::krb5_h::krb5_authenticator;
    let mut reqcksum = crate::krb5_h::krb5_checksum {
        magic: 0,
        checksum_type: 0,
        length: 0,
        contents: 0 as *mut crate::krb5_h::krb5_octet,
    };
    let mut name = 0 as crate::gssapiP_krb5_h::krb5_gss_name_t;
    let mut gss_flags = 0u32;
    let mut ctx = 0 as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    let mut now: crate::krb5_h::krb5_timestamp = 0;
    let mut token = crate::gssapi_h::gss_buffer_desc {
        length: 0,
        value: 0 as *mut libc::c_void,
    };
    let mut auth_context = 0 as crate::krb5_h::krb5_auth_context;
    let mut ticket = 0 as *mut crate::krb5_h::krb5_ticket;
    let mut option_id: i32 = 0;
    let mut option = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut mech_used = 0 as *const crate::gssapi_h::gss_OID_desc;
    let mut major_status = (13u32) << 16i32;
    let mut tmp_minor_status: crate::gssapi_h::OM_uint32 = 0;
    let mut krb_error_data = crate::krb5_h::krb5_error {
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
    let mut scratch = crate::krb5_h::krb5_data {
        magic: 0,
        length: 0,
        data: 0 as *mut i8,
    };
    let mut defcred = 0 as crate::gssapi_h::gss_cred_id_t;
    let mut deleg_cred = 0 as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
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
    let mut cred_rcache = 0i32;
    let mut no_encap = 0i32;
    let mut token_deleg_flag = 0i32;
    let mut ap_req_options = 0i32;
    let mut negotiated_etype: crate::krb5_h::krb5_enctype = 0;
    let mut ad_context = 0 as crate::k5_int_h::krb5_authdata_context;
    let mut accprinc = 0 as crate::krb5_h::krb5_principal;
    let mut request = 0 as *mut crate::krb5_h::krb5_ap_req;
    code = crate::k5_int_h::krb5int_accessor(
        &mut kaccess,
        ((::std::mem::size_of::<crate::k5_int_h::krb5int_access>() & 0xffffusize
            | ((23i32) << 16i32) as usize) as u32
            & 0xffffffffu32) as crate::krb5_h::krb5_int32,
    );
    if code != 0 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    code = crate::src::krb5::init_sec_context::krb5_gss_init_context(&mut context);
    if code != 0 {
        *minor_status = code as crate::gssapi_h::OM_uint32;
        return (13u32) << 16i32;
    }
    /* set up returns to be freeable */
    if !src_name.is_null() {
        *src_name = 0 as crate::gssapi_h::gss_name_t
    }
    (*output_token).length = 0usize;
    (*output_token).value = 0 as *mut libc::c_void;
    token.value = 0 as *mut libc::c_void;
    reqcksum.contents = 0 as *mut crate::krb5_h::krb5_octet;
    ap_req.data = 0 as *mut i8;
    ap_rep.data = 0 as *mut i8;
    if !mech_type.is_null() {
        *mech_type = 0 as crate::gssapi_h::gss_OID
    }
    /* return a bogus cred handle */
    if !delegated_cred_handle.is_null() {
        *delegated_cred_handle = 0 as crate::gssapi_h::gss_cred_id_t
    }
    /* handle default cred handle */
    if verifier_cred_handle.is_null() {
        major_status = crate::src::krb5::acquire_cred::krb5_gss_acquire_cred(
            minor_status,
            0 as crate::gssapi_h::gss_name_t,
            0xffffffffu32,
            0 as crate::gssapi_h::gss_OID_set,
            2i32,
            &mut defcred,
            0 as *mut crate::gssapi_h::gss_OID_set,
            0 as *mut crate::gssapi_h::OM_uint32,
        );
        if major_status != 0u32 {
            code = *minor_status as crate::krb5_h::krb5_error_code;
            current_block = 6451473480150109090;
        } else {
            verifier_cred_handle = defcred;
            current_block = 12381812505308290051;
        }
    } else {
        current_block = 12381812505308290051;
    }
    match current_block {
        12381812505308290051 => {
            /* Resolve any initiator state in the verifier cred and lock it. */
            major_status = crate::src::krb5::acquire_cred::kg_cred_resolve(
                minor_status,
                context,
                verifier_cred_handle,
                0 as crate::gssapi_h::gss_name_t,
            );
            if major_status & ((0o377u32) << 24i32 | (0o377u32) << 16i32) != 0 {
                code = *minor_status as crate::krb5_h::krb5_error_code;
                current_block = 6451473480150109090;
            } else {
                cred = verifier_cred_handle as crate::gssapiP_krb5_h::krb5_gss_cred_id_t;
                /* make sure the supplied credentials are valid for accept */
                if (*cred).usage != 2i32 && (*cred).usage != 0i32 {
                    code = 0i32;
                    major_status = (7u32) << 16i32;
                    current_block = 6451473480150109090;
                } else {
                    /* verify the token's integrity, and leave the token in ap_req.
                    figure out which mech oid was used, and save it */
                    ptr = (*input_token).value as *mut u8;
                    code = crate::src::generic::util_token::gssint_g_verify_token_header(
                        crate::src::krb5::gssapi_krb5::gss_mech_krb5
                            as *const crate::gssapi_h::gss_OID_desc,
                        &mut ap_req.length,
                        &mut ptr,
                        0x100i32,
                        (*input_token).length as u32,
                        1i32,
                    );
                    if code == 0 {
                        mech_used = crate::src::krb5::gssapi_krb5::gss_mech_krb5
                            as *const crate::gssapi_h::gss_OID_desc;
                        current_block = 919954187481050311;
                    } else if code as isize == -(2045022965 as isize) && {
                        code = crate::src::generic::util_token::gssint_g_verify_token_header(
                            crate::src::krb5::gssapi_krb5::gss_mech_iakerb
                                as *const crate::gssapi_h::gss_OID_desc,
                            &mut ap_req.length,
                            &mut ptr,
                            0x100i32,
                            (*input_token).length as u32,
                            1i32,
                        );
                        (code) == 0
                    } {
                        mech_used = crate::src::krb5::gssapi_krb5::gss_mech_iakerb
                            as *const crate::gssapi_h::gss_OID_desc;
                        current_block = 919954187481050311;
                    } else if code as isize == -(2045022965 as isize) && {
                        code = crate::src::generic::util_token::gssint_g_verify_token_header(
                            crate::src::krb5::gssapi_krb5::gss_mech_krb5_wrong
                                as *const crate::gssapi_h::gss_OID_desc,
                            &mut ap_req.length,
                            &mut ptr,
                            0x100i32,
                            (*input_token).length as u32,
                            1i32,
                        );
                        (code) == 0
                    } {
                        mech_used = crate::src::krb5::gssapi_krb5::gss_mech_krb5_wrong
                            as *const crate::gssapi_h::gss_OID_desc;
                        current_block = 919954187481050311;
                    } else if code as isize == -(2045022965 as isize) && {
                        code = crate::src::generic::util_token::gssint_g_verify_token_header(
                            crate::src::krb5::gssapi_krb5::gss_mech_krb5_old
                                as *const crate::gssapi_h::gss_OID_desc,
                            &mut ap_req.length,
                            &mut ptr,
                            0x100i32,
                            (*input_token).length as u32,
                            1i32,
                        );
                        (code) == 0
                    } {
                        /*
                         * Previous versions of this library used the old mech_id
                         * and some broken behavior (wrong IV on checksum
                         * encryption).  We support the old mech_id for
                         * compatibility, and use it to decide when to use the
                         * old behavior.
                         */
                        mech_used = crate::src::krb5::gssapi_krb5::gss_mech_krb5_old
                            as *const crate::gssapi_h::gss_OID_desc;
                        current_block = 919954187481050311;
                    } else if code as isize == -(2045022960 as isize) {
                        major_status = ((1i32) << 0i32 + 0i32) as crate::gssapi_h::OM_uint32;
                        code = -(1765328344 as isize) as crate::krb5_h::krb5_error_code;
                        mech_used = crate::src::krb5::gssapi_krb5::gss_mech_krb5
                            as *const crate::gssapi_h::gss_OID_desc;
                        current_block = 6451473480150109090;
                    } else if code as isize == -(2045022964 as isize) {
                        /* DCE style not encapsulated */
                        ap_req.length = (*input_token).length as u32;
                        ap_req.data = (*input_token).value as *mut i8;
                        mech_used = crate::src::krb5::gssapi_krb5::gss_mech_krb5
                            as *const crate::gssapi_h::gss_OID_desc;
                        no_encap = 1i32;
                        current_block = 919954187481050311;
                    } else {
                        major_status = (9u32) << 16i32;
                        current_block = 6451473480150109090;
                    }
                    match current_block {
                        6451473480150109090 => {}
                        _ => {
                            sptr = ptr as *mut i8;
                            ap_req.data = sptr;
                            sptr = sptr.offset(ap_req.length as isize);
                            /* construct the sender_addr */
                            if !input_chan_bindings.is_null()
                                && (*input_chan_bindings).initiator_addrtype == 2u32
                            {
                                /* XXX is this right? */
                                addr.addrtype = 0x2i32;
                                addr.length =
                                    (*input_chan_bindings).initiator_address.length as u32;
                                addr.contents = (*input_chan_bindings).initiator_address.value
                                    as *mut crate::krb5_h::krb5_octet;
                                paddr = &mut addr
                            } else {
                                paddr = 0 as *mut crate::krb5_h::krb5_address
                            }
                            /* decode the AP_REQ message */
                            code = crate::k5_int_h::decode_krb5_ap_req(&mut ap_req, &mut request);
                            if code != 0 {
                                major_status = (13u32) << 16i32;
                                current_block = 3928475635904994795;
                            } else {
                                ticket = (*request).ticket;
                                /* decode the message */
                                code =
                                    crate::krb5_h::krb5_auth_con_init(context, &mut auth_context);
                                if code != 0 {
                                    major_status = (13u32) << 16i32;
                                    crate::src::krb5::disp_status::krb5_gss_save_error_info(
                                        code as crate::gssapi_h::OM_uint32,
                                        context,
                                    );
                                } else {
                                    if !(*cred).rcache.is_null() {
                                        cred_rcache = 1i32;
                                        code = crate::krb5_h::krb5_auth_con_setrcache(
                                            context,
                                            auth_context,
                                            (*cred).rcache,
                                        );
                                        if code != 0 {
                                            major_status = (13u32) << 16i32;
                                            current_block = 6451473480150109090;
                                        } else {
                                            current_block = 6040267449472925966;
                                        }
                                    } else {
                                        current_block = 6040267449472925966;
                                    }
                                    match current_block {
                                        6451473480150109090 => {}
                                        _ => {
                                            code = crate::krb5_h::krb5_auth_con_setaddrs(
                                                context,
                                                auth_context,
                                                0 as *mut crate::krb5_h::krb5_address,
                                                paddr,
                                            );
                                            if code != 0 {
                                                major_status = (13u32) << 16i32
                                            } else {
                                                /* Limit the encryption types negotiated (if requested). */
                                                if !(*cred).req_enctypes.is_null() {
                                                    code =
                                                        crate::k5_int_h::krb5_auth_con_setpermetypes(context,
                                                                                    auth_context,
                                                                                    (*cred).req_enctypes);
                                                    if code != 0 {
                                                        major_status = (13u32) << 16i32;
                                                        current_block = 6451473480150109090;
                                                    } else {
                                                        current_block = 2904036176499606090;
                                                    }
                                                } else {
                                                    current_block = 2904036176499606090;
                                                }
                                                match current_block {
                                                    6451473480150109090 => {}
                                                    _ => {
                                                        if (*cred).default_identity() == 0 {
                                                            code =
                                                                crate::src::krb5::naming_exts::kg_acceptor_princ(context,
                                                                                  (*cred).name,
                                                                                  &mut accprinc)
                                                                    as
                                                                    crate::krb5_h::krb5_error_code;
                                                            if code != 0 {
                                                                major_status = (13u32) << 16i32;
                                                                current_block = 6451473480150109090;
                                                            } else {
                                                                current_block = 7639320476250304355;
                                                            }
                                                        } else {
                                                            current_block = 7639320476250304355;
                                                        }
                                                        match current_block {
                                                            6451473480150109090 => {}
                                                            _ => {
                                                                code =
                                                                    crate::k5_int_h::krb5_rd_req_decoded(context,
                                                                                        &mut auth_context,
                                                                                        request,
                                                                                        accprinc
                                                                                            as
                                                                                            crate::krb5_h::krb5_const_principal,
                                                                                        (*cred).keytab,
                                                                                        &mut ap_req_options,
                                                                                        0
                                                                                            as
                                                                                            *mut *mut crate::krb5_h::krb5_ticket);
                                                                crate::krb5_h::krb5_free_principal(
                                                                    context, accprinc,
                                                                );
                                                                if code != 0 {
                                                                    major_status = (13u32) << 16i32
                                                                } else {
                                                                    crate::krb5_h::krb5_auth_con_setflags(context,
                                                                                           auth_context,
                                                                                           0x4i32);
                                                                    crate::krb5_h::krb5_auth_con_getauthenticator(context,
                                                                                                   auth_context,
                                                                                                   &mut authdat);
                                                                    if (*authdat).checksum.is_null()
                                                                    {
                                                                        /*
                                                                         * Some SMB client implementations use handcrafted GSSAPI code that
                                                                         * does not provide a checksum.  MS-KILE documents that the Microsoft
                                                                         * implementation considers a missing checksum acceptable; the server
                                                                         * assumes all flags are unset in this case, and does not check channel
                                                                         * bindings.
                                                                         */
                                                                        gss_flags = 0u32;
                                                                        current_block =
                                                                            12788783625999190409;
                                                                    } else if (*(*authdat).checksum)
                                                                        .checksum_type
                                                                        != 0x8003i32
                                                                    {
                                                                        /* Samba does not send 0x8003 GSS-API checksums */
                                                                        let mut valid:
                                                                                crate::krb5_h::krb5_boolean =
                                                                            0;
                                                                        let mut subkey =
                                                                            0
                                                                                as
                                                                                *mut crate::k5_int_h::krb5_key_st;
                                                                        let mut zero =
                                                                            crate::krb5_h::krb5_data{magic: 0, length: 0, data: 0 as *mut i8,};
                                                                        code =
                                                                            crate::krb5_h::krb5_auth_con_getkey_k(context,
                                                                                                   auth_context,
                                                                                                   &mut subkey);
                                                                        if code != 0 {
                                                                            major_status =
                                                                                (13u32) << 16i32;
                                                                            current_block =
                                                                                6451473480150109090;
                                                                        } else {
                                                                            zero.length = 0u32;
                                                                            zero.data = b"\x00"
                                                                                as *const u8
                                                                                as *mut i8;
                                                                            code
                                                                                =
                                                                                crate::krb5_h::krb5_k_verify_checksum(context,
                                                                                                       subkey,
                                                                                                       10i32,
                                                                                                       &mut zero,
                                                                                                       (*authdat).checksum,
                                                                                                       &mut valid);
                                                                            crate::krb5_h::krb5_k_free_key(context,
                                                                                            subkey);
                                                                            if code != 0
                                                                                || valid == 0
                                                                            {
                                                                                major_status =
                                                                                    (6u32) << 16i32;
                                                                                current_block
                                                                                    =
                                                                                    6451473480150109090;
                                                                            } else {
                                                                                /* Use ap_options from the request to guess the mutual flag. */
                                                                                gss_flags
                                                                                    =
                                                                                    (4i32
                                                                                         |
                                                                                         8i32)
                                                                                        as
                                                                                        crate::krb5_h::krb5_ui_4;
                                                                                if ap_req_options
                                                                                    & 0x20000000i32
                                                                                    != 0
                                                                                {
                                                                                    gss_flags |=
                                                                                        2u32
                                                                                }
                                                                                current_block
                                                                                    =
                                                                                    12788783625999190409;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        /* gss krb5 v1 */
                                                                        /* stash this now, for later. */
                                                                        code =
                                                                            crate::krb5_h::krb5_c_checksum_length(context,
                                                                                                   0x7i32,
                                                                                                   &mut md5len);
                                                                        if code != 0 {
                                                                            major_status =
                                                                                (13u32) << 16i32;
                                                                            current_block =
                                                                                6451473480150109090;
                                                                        } else if (*(*authdat)
                                                                            .checksum)
                                                                            .checksum_type
                                                                            != 0x8003i32
                                                                            || (*(*authdat)
                                                                                .checksum)
                                                                                .length
                                                                                < 24u32
                                                                        {
                                                                            code = 0i32;
                                                                            major_status =
                                                                                (4u32) << 16i32;
                                                                            current_block =
                                                                                6451473480150109090;
                                                                        } else {
                                                                            ptr = (*(*authdat)
                                                                                .checksum)
                                                                                .contents;
                                                                            tmp = if 0i32 != 0 {
                                                                                load_32_be(ptr
                                                                                                   as
                                                                                                   *const libc::c_void)
                                                                            } else {
                                                                                load_32_le(ptr
                                                                                                   as
                                                                                                   *const libc::c_void)
                                                                            };
                                                                            ptr =
                                                                                ptr.offset(4isize);
                                                                            if tmp as usize
                                                                                != md5len
                                                                            {
                                                                                code = 39756038i32;
                                                                                major_status =
                                                                                    (13u32)
                                                                                        << 16i32;
                                                                                current_block
                                                                                    =
                                                                                    6451473480150109090;
                                                                            } else {
                                                                                /* verify that the checksum is correct */
                                                                                /*
                                                                                  The checksum may be either exactly 24 bytes, in which case
                                                                                  no options are specified, or greater than 24 bytes, in which case
                                                                                  one or more options are specified. Currently, the only valid
                                                                                  option is KRB5_GSS_FOR_CREDS_OPTION ( = 1 ).
                                                                                */
                                                                                /*
                                                                                The following section of code attempts to implement the
                                                                                optional channel binding facility as described in RFC2743.

                                                                                Since this facility is optional channel binding may or may
                                                                                not have been provided by either the client or the server.

                                                                                If the server has specified input_chan_bindings equal to
                                                                                GSS_C_NO_CHANNEL_BINDINGS then we skip the check.  If
                                                                                the server does provide channel bindings then we compute
                                                                                a checksum and compare against those provided by the
                                                                                client.         */
                                                                                code
                                                                                    =
                                                                                    crate::src::krb5::util_cksum::kg_checksum_channel_bindings(context,
                                                                                                                 input_chan_bindings,
                                                                                                                 &mut reqcksum);
                                                                                if code != 0 {
                                                                                    major_status
                                                                                        =
                                                                                        (4u32)
                                                                                            <<
                                                                                            16i32;
                                                                                    current_block
                                                                                        =
                                                                                        6451473480150109090;
                                                                                } else {
                                                                                    /* Always read the clients bindings - eventhough we might ignore them */
                                                                                    ptr2 = ptr;
                                                                                    ptr
                                                                                        =
                                                                                        ptr.offset(reqcksum.length
                                                                                                       as
                                                                                                       isize);
                                                                                    if !input_chan_bindings.is_null()
                                                                                       {
                                                                                        if crate::stdlib::memcmp(ptr2
                                                                                                      as
                                                                                                      *const libc::c_void,
                                                                                                  reqcksum.contents
                                                                                                      as
                                                                                                      *const libc::c_void,
                                                                                                  reqcksum.length
                                                                                                      as
                                                                                                      usize)
                                                                                               !=
                                                                                               0i32
                                                                                           {
                                                                                            crate::stdlib::free(reqcksum.contents
                                                                                                     as
                                                                                                     *mut libc::c_void);
                                                                                            reqcksum.contents
                                                                                                =
                                                                                                0
                                                                                                    as
                                                                                                    *mut crate::krb5_h::krb5_octet;
                                                                                            code
                                                                                                =
                                                                                                0i32;
                                                                                            major_status
                                                                                                =
                                                                                                (4u32)
                                                                                                    <<
                                                                                                    16i32;
                                                                                            current_block
                                                                                                =
                                                                                                6451473480150109090;
                                                                                        } else {
                                                                                            current_block
                                                                                                =
                                                                                                12129449210080749085;
                                                                                        }
                                                                                    } else {
                                                                                        current_block
                                                                                            =
                                                                                            12129449210080749085;
                                                                                    }
                                                                                    match current_block
                                                                                        {
                                                                                        6451473480150109090
                                                                                        =>
                                                                                        {
                                                                                        }
                                                                                        _
                                                                                        =>
                                                                                        {
                                                                                            crate::stdlib::free(reqcksum.contents
                                                                                                     as
                                                                                                     *mut libc::c_void);
                                                                                            reqcksum.contents
                                                                                                =
                                                                                                0
                                                                                                    as
                                                                                                    *mut crate::krb5_h::krb5_octet;
                                                                                            /* Read the token flags.  Remember if GSS_C_DELEG_FLAG was set, but
         * mask it out until we actually read a delegated credential. */
                                                                                            gss_flags
                                                                                                =
                                                                                                if 0i32
                                                                                                       !=
                                                                                                       0
                                                                                                   {
                                                                                                    load_32_be(ptr
                                                                                                                   as
                                                                                                                   *const libc::c_void)
                                                                                                } else {
                                                                                                    load_32_le(ptr
                                                                                                                   as
                                                                                                                   *const libc::c_void)
                                                                                                };
                                                                                            ptr
                                                                                                =
                                                                                                ptr.offset(4isize);
                                                                                            token_deleg_flag
                                                                                                =
                                                                                                (gss_flags
                                                                                                     &
                                                                                                     1u32)
                                                                                                    as
                                                                                                    i32;
                                                                                            gss_flags
                                                                                                &=
                                                                                                !(1i32)
                                                                                                    as
                                                                                                    u32;
                                                                                            /* if the checksum length > 24, there are options to process */
                                                                                            i
                                                                                                =
                                                                                                (*(*authdat).checksum).length.wrapping_sub(24u32);
                                                                                            if i
                                                                                                   !=
                                                                                                   0
                                                                                                   &&
                                                                                                   token_deleg_flag
                                                                                                       !=
                                                                                                       0
                                                                                               {
                                                                                                if i
                                                                                                       >=
                                                                                                       4u32
                                                                                                   {
                                                                                                    option_id
                                                                                                        =
                                                                                                        if 0i32
                                                                                                               !=
                                                                                                               0
                                                                                                           {
                                                                                                            (((load_16_be(ptr
                                                                                                                             as
                                                                                                                             *const libc::c_void)
                                                                                                                  as
                                                                                                                  i32)))
                                                                                                                <<
                                                                                                                16i32
                                                                                                        } else {
                                                                                                            load_16_le(ptr
                                                                                                                           as
                                                                                                                           *const libc::c_void)
                                                                                                                as
                                                                                                                i32
                                                                                                        };
                                                                                                    ptr
                                                                                                        =
                                                                                                        ptr.offset(2isize);
                                                                                                    option.length
                                                                                                        =
                                                                                                        if 0i32
                                                                                                               !=
                                                                                                               0
                                                                                                           {
                                                                                                            (((load_16_be(ptr
                                                                                                                             as
                                                                                                                             *const libc::c_void)
                                                                                                                  as
                                                                                                                  i32)))
                                                                                                                <<
                                                                                                                16i32
                                                                                                        } else {
                                                                                                            load_16_le(ptr
                                                                                                                           as
                                                                                                                           *const libc::c_void)
                                                                                                                as
                                                                                                                i32
                                                                                                        }
                                                                                                            as
                                                                                                            u32;
                                                                                                    ptr
                                                                                                        =
                                                                                                        ptr.offset(2isize);
                                                                                                    i
                                                                                                        =
                                                                                                        i.wrapping_sub(4u32);
                                                                                                    if i
                                                                                                           <
                                                                                                           option.length
                                                                                                       {
                                                                                                        code = 39756038i32;
                                                                                                        major_status = (13u32) << 16i32;
                                                                                                        current_block = 6451473480150109090;
                                                                                                    } else {
                                                                                                        /* have to use ptr2, since option.data is wrong type and
                   macro uses ptr as both lvalue and rvalue */
                                                                                                        ptr2
                                                                                                            =
                                                                                                            ptr;
                                                                                                        ptr
                                                                                                            =
                                                                                                            ptr.offset(option.length
                                                                                                                           as
                                                                                                                           isize);
                                                                                                        option.data
                                                                                                            =
                                                                                                            ptr2
                                                                                                                as
                                                                                                                *mut i8;
                                                                                                        i
                                                                                                            =
                                                                                                            i.wrapping_sub(option.length);
                                                                                                        if option_id
                                                                                                               !=
                                                                                                               1i32
                                                                                                           {
                                                                                                            major_status
                                                                                                                =
                                                                                                                (13u32)
                                                                                                                    <<
                                                                                                                    16i32;
                                                                                                            current_block
                                                                                                                =
                                                                                                                6451473480150109090;
                                                                                                        } else {
                                                                                                            /* store the delegated credential */
                                                                                                            code
                                                                                                                =
                                                                                                                rd_and_store_for_creds(context,
                                                                                                                                       auth_context,
                                                                                                                                       &mut option,
                                                                                                                                       if !delegated_cred_handle.is_null()
                                                                                                                                          {
                                                                                                                                           &mut deleg_cred
                                                                                                                                       } else {
                                                                                                                                           0
                                                                                                                                               as
                                                                                                                                               *mut crate::gssapiP_krb5_h::krb5_gss_cred_id_t
                                                                                                                                       });
                                                                                                            if code
                                                                                                                   !=
                                                                                                                   0
                                                                                                               {
                                                                                                                major_status
                                                                                                                    =
                                                                                                                    (13u32)
                                                                                                                        <<
                                                                                                                        16i32;
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    6451473480150109090;
                                                                                                            } else {
                                                                                                                gss_flags
                                                                                                                    |=
                                                                                                                    1u32;
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    8551376836414271792;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    current_block
                                                                                                        =
                                                                                                        8551376836414271792;
                                                                                                }
                                                                                                /* if i >= 4 */
                                                                                                /* ignore any additional trailing data, for now */
                                                                                            } else {
                                                                                                current_block
                                                                                                    =
                                                                                                    8551376836414271792;
                                                                                            }
                                                                                            match current_block
                                                                                                {
                                                                                                6451473480150109090
                                                                                                =>
                                                                                                {
                                                                                                }
                                                                                                _
                                                                                                =>
                                                                                                {
                                                                                                    loop {
                                                                                                        if !(i
                                                                                                                 >
                                                                                                                 0u32)
                                                                                                           {
                                                                                                            current_block
                                                                                                                =
                                                                                                                12788783625999190409;
                                                                                                            break
                                                                                                                ;
                                                                                                        }
                                                                                                        /* Process Type-Length-Data options */
                                                                                                        if i
                                                                                                               <
                                                                                                               8u32
                                                                                                           {
                                                                                                            code = 39756038i32;
                                                                                                            major_status
                                                                                                                =
                                                                                                                (13u32)
                                                                                                                    <<
                                                                                                                    16i32;
                                                                                                            current_block
                                                                                                                =
                                                                                                                6451473480150109090;
                                                                                                            break
                                                                                                                ;
                                                                                                        } else {
                                                                                                            option_id
                                                                                                                =
                                                                                                                if 1i32
                                                                                                                       !=
                                                                                                                       0
                                                                                                                   {
                                                                                                                    load_32_be(ptr
                                                                                                                                   as
                                                                                                                                   *const libc::c_void)
                                                                                                                } else {
                                                                                                                    load_32_le(ptr
                                                                                                                                   as
                                                                                                                                   *const libc::c_void)
                                                                                                                }
                                                                                                                    as
                                                                                                                    i32;
                                                                                                            ptr
                                                                                                                =
                                                                                                                ptr.offset(4isize);
                                                                                                            option.length
                                                                                                                =
                                                                                                                if 1i32
                                                                                                                       !=
                                                                                                                       0
                                                                                                                   {
                                                                                                                    load_32_be(ptr
                                                                                                                                   as
                                                                                                                                   *const libc::c_void)
                                                                                                                } else {
                                                                                                                    load_32_le(ptr
                                                                                                                                   as
                                                                                                                                   *const libc::c_void)
                                                                                                                };
                                                                                                            ptr
                                                                                                                =
                                                                                                                ptr.offset(4isize);
                                                                                                            i
                                                                                                                =
                                                                                                                i.wrapping_sub(8u32);
                                                                                                            if i
                                                                                                                   <
                                                                                                                   option.length
                                                                                                               {
                                                                                                                code = 39756038i32;
                                                                                                                major_status
                                                                                                                    =
                                                                                                                    (13u32)
                                                                                                                        <<
                                                                                                                        16i32;
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    6451473480150109090;
                                                                                                                break
                                                                                                                    ;
                                                                                                            } else {
                                                                                                                ptr2
                                                                                                                    =
                                                                                                                    ptr;
                                                                                                                ptr
                                                                                                                    =
                                                                                                                    ptr.offset(option.length
                                                                                                                                   as
                                                                                                                                   isize);
                                                                                                                option.data
                                                                                                                    =
                                                                                                                    ptr2
                                                                                                                        as
                                                                                                                        *mut i8;
                                                                                                                i
                                                                                                                    =
                                                                                                                    i.wrapping_sub(option.length);
                                                                                                                code
                                                                                                                    =
                                                                                                                    kg_process_extension(context,
                                                                                                                                         auth_context,
                                                                                                                                         option_id,
                                                                                                                                         &mut option,
                                                                                                                                         exts);
                                                                                                                if !(code
                                                                                                                         !=
                                                                                                                         0i32)
                                                                                                                   {
                                                                                                                    continue
                                                                                                                        ;
                                                                                                                }
                                                                                                                major_status
                                                                                                                    =
                                                                                                                    (13u32)
                                                                                                                        <<
                                                                                                                        16i32;
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    6451473480150109090;
                                                                                                                break
                                                                                                                    ;
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
                                                                    match current_block {
                                                                        6451473480150109090 => {}
                                                                        _ => {
                                                                            if !(*exts)
                                                                                .iakerb
                                                                                .conv
                                                                                .is_null()
                                                                                && (*exts)
                                                                                    .iakerb
                                                                                    .verified
                                                                                    == 0
                                                                            {
                                                                                major_status =
                                                                                    (6u32) << 16i32
                                                                            } else if no_encap
                                                                                != (gss_flags
                                                                                    & 0x1000u32
                                                                                    != 0u32)
                                                                                    as i32
                                                                            {
                                                                                major_status =
                                                                                    (9u32) << 16i32
                                                                            } else {
                                                                                /* only DCE_STYLE clients are allowed to send raw AP-REQs */
                                                                                /* create the ctx struct and start filling it in */
                                                                                ctx
                                                                                    =
                                                                                    crate::stdlib::malloc(::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec>())
                                                                                        as
                                                                                        *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
                                                                                if ctx.is_null() {
                                                                                    code = 12i32;
                                                                                    major_status =
                                                                                        (13u32)
                                                                                            << 16i32
                                                                                } else {
                                                                                    crate::stdlib::memset(ctx
                                                                                               as
                                                                                               *mut libc::c_void,
                                                                                           0i32, ::std::mem::size_of::<crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec>());
                                                                                    (*ctx).magic =
                                                                                        39756040i32;
                                                                                    (*ctx).mech_used
                                                                                        =
                                                                                        mech_used
                                                                                            as
                                                                                            crate::gssapi_h::gss_OID;
                                                                                    (*ctx).auth_context
                                                                                        =
                                                                                        auth_context;
                                                                                    (*ctx).set_initiate(0u32);
                                                                                    (*ctx).gss_flags
                                                                                        =
                                                                                        256u32
                                                                                            |
                                                                                            gss_flags
                                                                                                &
                                                                                                (32i32
                                                                                                     |
                                                                                                     16i32
                                                                                                     |
                                                                                                     2i32
                                                                                                     |
                                                                                                     4i32
                                                                                                     |
                                                                                                     8i32
                                                                                                     |
                                                                                                     1i32
                                                                                                     |
                                                                                                     0x1000i32
                                                                                                     |
                                                                                                     0x2000i32
                                                                                                     |
                                                                                                     0x4000i32)
                                                                                                    as
                                                                                                    u32;
                                                                                    (*ctx).set_seed_init(0u32);
                                                                                    (*ctx).cred_rcache
                                                                                        =
                                                                                        cred_rcache;
                                                                                    /* XXX move this into gss_name_t */
                                                                                    code
                                                                                        =
                                                                                        crate::krb5_h::krb5_merge_authdata(context,
                                                                                                            (*(*ticket).enc_part2).authorization_data,
                                                                                                            (*authdat).authorization_data,
                                                                                                            &mut (*ctx).authdata);
                                                                                    if code != 0 {
                                                                                        major_status
                                                                                            =
                                                                                            (13u32)
                                                                                                <<
                                                                                                16i32
                                                                                    } else {
                                                                                        code
                                                                                            =
                                                                                            crate::src::krb5::naming_exts::kg_init_name(context,
                                                                                                         (*ticket).server,
                                                                                                         0
                                                                                                             as
                                                                                                             *mut i8,
                                                                                                         0
                                                                                                             as
                                                                                                             *mut i8,
                                                                                                         0
                                                                                                             as
                                                                                                             crate::k5_int_h::krb5_authdata_context,
                                                                                                         0i32,
                                                                                                         &mut (*ctx).here);
                                                                                        if code != 0
                                                                                        {
                                                                                            major_status
                                                                                                =
                                                                                                (13u32)
                                                                                                    <<
                                                                                                    16i32
                                                                                        } else {
                                                                                            code
                                                                                                =
                                                                                                crate::k5_int_h::krb5_auth_con_get_authdata_context(context,
                                                                                                                                   auth_context,
                                                                                                                                   &mut ad_context);
                                                                                            if code
                                                                                                != 0
                                                                                            {
                                                                                                major_status
                                                                                                    =
                                                                                                    (13u32)
                                                                                                        <<
                                                                                                        16i32
                                                                                            } else {
                                                                                                code
                                                                                                    =
                                                                                                    crate::src::krb5::naming_exts::kg_init_name(context,
                                                                                                                 (*authdat).client,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     *mut i8,
                                                                                                                 0
                                                                                                                     as
                                                                                                                     *mut i8,
                                                                                                                 ad_context,
                                                                                                                 0x1i32,
                                                                                                                 &mut (*ctx).there);
                                                                                                if code
                                                                                                       !=
                                                                                                       0
                                                                                                   {
                                                                                                    major_status
                                                                                                        =
                                                                                                        (13u32)
                                                                                                            <<
                                                                                                            16i32
                                                                                                } else {
                                                                                                    /* Now owned by ctx->there */
                                                                                                    (*authdat).client
                                                                                                        =
                                                                                                        0
                                                                                                            as
                                                                                                            crate::krb5_h::krb5_principal;
                                                                                                    crate::k5_int_h::krb5_auth_con_set_authdata_context(context,
                                                                                                                                       auth_context,
                                                                                                                                       0
                                                                                                                                           as
                                                                                                                                           crate::k5_int_h::krb5_authdata_context);
                                                                                                    code
                                                                                                        =
                                                                                                        crate::krb5_h::krb5_auth_con_getrecvsubkey_k(context,
                                                                                                                                      auth_context,
                                                                                                                                      &mut (*ctx).subkey);
                                                                                                    if code
                                                                                                           !=
                                                                                                           0
                                                                                                       {
                                                                                                        major_status
                                                                                                            =
                                                                                                            (13u32)
                                                                                                                <<
                                                                                                                16i32
                                                                                                    } else {
                                                                                                        /* use the session key if the subkey isn't present */
                                                                                                        if (*ctx).subkey.is_null()
                                                                                                           {
                                                                                                            code
                                                                                                                =
                                                                                                                crate::krb5_h::krb5_auth_con_getkey_k(context,
                                                                                                                                       auth_context,
                                                                                                                                       &mut (*ctx).subkey);
                                                                                                            if code
                                                                                                                   !=
                                                                                                                   0
                                                                                                               {
                                                                                                                major_status
                                                                                                                    =
                                                                                                                    (13u32)
                                                                                                                        <<
                                                                                                                        16i32;
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    6451473480150109090;
                                                                                                            } else {
                                                                                                                current_block
                                                                                                                    =
                                                                                                                    4540002298510446694;
                                                                                                            }
                                                                                                        } else {
                                                                                                            current_block
                                                                                                                =
                                                                                                                4540002298510446694;
                                                                                                        }
                                                                                                        match current_block
                                                                                                            {
                                                                                                            6451473480150109090
                                                                                                            =>
                                                                                                            {
                                                                                                            }
                                                                                                            _
                                                                                                            =>
                                                                                                            {
                                                                                                                if (*ctx).subkey.is_null()
                                                                                                                   {
                                                                                                                    /* this isn't a very good error, but it's not clear to me this
           can actually happen */
                                                                                                                    major_status
                                                                                                                        =
                                                                                                                        (13u32)
                                                                                                                            <<
                                                                                                                            16i32;
                                                                                                                    code
                                                                                                                        =
                                                                                                                        -(1765328375
                                                                                                                              as
                                                                                                                              isize)
                                                                                                                            as
                                                                                                                            crate::krb5_h::krb5_error_code
                                                                                                                } else {
                                                                                                                    (*ctx).enc
                                                                                                                        =
                                                                                                                        0
                                                                                                                            as
                                                                                                                            crate::krb5_h::krb5_key;
                                                                                                                    (*ctx).seq
                                                                                                                        =
                                                                                                                        0
                                                                                                                            as
                                                                                                                            crate::krb5_h::krb5_key;
                                                                                                                    (*ctx).set_have_acceptor_subkey(0u32);
                                                                                                                    /* DCE_STYLE implies acceptor_subkey */
                                                                                                                    if (*ctx).gss_flags
                                                                                                                           &
                                                                                                                           0x1000u32
                                                                                                                           ==
                                                                                                                           0u32
                                                                                                                       {
                                                                                                                        code
                                                                                                                            =
                                                                                                                            crate::src::krb5::util_crypt::kg_setup_keys(context,
                                                                                                                                          ctx,
                                                                                                                                          (*ctx).subkey,
                                                                                                                                          &mut (*ctx).cksumtype); /* struct copy */
                                                                                                                        if code
                                                                                                                               !=
                                                                                                                               0
                                                                                                                           {
                                                                                                                            major_status
                                                                                                                                =
                                                                                                                                (13u32)
                                                                                                                                    <<
                                                                                                                                    16i32;
                                                                                                                            current_block
                                                                                                                                =
                                                                                                                                6451473480150109090;
                                                                                                                        } else {
                                                                                                                            current_block
                                                                                                                                =
                                                                                                                                6936584767197543976;
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        current_block
                                                                                                                            =
                                                                                                                            6936584767197543976;
                                                                                                                    }
                                                                                                                    match current_block
                                                                                                                        {
                                                                                                                        6451473480150109090
                                                                                                                        =>
                                                                                                                        {
                                                                                                                        }
                                                                                                                        _
                                                                                                                        =>
                                                                                                                        {
                                                                                                                            (*ctx).krb_times
                                                                                                                                =
                                                                                                                                (*(*ticket).enc_part2).times;
                                                                                                                            (*ctx).krb_flags
                                                                                                                                =
                                                                                                                                (*(*ticket).enc_part2).flags;
                                                                                                                            if !delegated_cred_handle.is_null()
                                                                                                                                   &&
                                                                                                                                   deleg_cred.is_null()
                                                                                                                                   &&
                                                                                                                                   (*cred).usage
                                                                                                                                       ==
                                                                                                                                       0i32
                                                                                                                               {
                                                                                                                                /*
         * Now, we always fabricate a delegated credentials handle
         * containing the service ticket to ourselves, which can be
         * used for S4U2Proxy.
         */
                                                                                                                                major_status
                                                                                                                                    =
                                                                                                                                    create_constrained_deleg_creds(minor_status,
                                                                                                                                                                   cred,
                                                                                                                                                                   ticket,
                                                                                                                                                                   &mut deleg_cred,
                                                                                                                                                                   context);
                                                                                                                                if major_status
                                                                                                                                       &
                                                                                                                                       ((0o377u32)
                                                                                                                                            <<
                                                                                                                                            24i32
                                                                                                                                            |
                                                                                                                                            (0o377u32)
                                                                                                                                                <<
                                                                                                                                                16i32)
                                                                                                                                       !=
                                                                                                                                       0
                                                                                                                                   {
                                                                                                                                    current_block
                                                                                                                                        =
                                                                                                                                        6451473480150109090;
                                                                                                                                } else {
                                                                                                                                    (*ctx).gss_flags
                                                                                                                                        |=
                                                                                                                                        1u32;
                                                                                                                                    current_block
                                                                                                                                        =
                                                                                                                                        15055213890147597004;
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                current_block
                                                                                                                                    =
                                                                                                                                    15055213890147597004;
                                                                                                                            }
                                                                                                                            match current_block
                                                                                                                                {
                                                                                                                                6451473480150109090
                                                                                                                                =>
                                                                                                                                {
                                                                                                                                }
                                                                                                                                _
                                                                                                                                =>
                                                                                                                                {
                                                                                                                                    let mut seq_temp:
                                                                                                                                            crate::krb5_h::krb5_int32 =
                                                                                                                                        0;
                                                                                                                                    crate::krb5_h::krb5_auth_con_getremoteseqnumber(context,
                                                                                                                                                                     auth_context,
                                                                                                                                                                     &mut seq_temp);
                                                                                                                                    (*ctx).seq_recv
                                                                                                                                        =
                                                                                                                                        seq_temp
                                                                                                                                            as
                                                                                                                                            crate::stdlib::uint64_t;
                                                                                                                                    code
                                                                                                                                        =
                                                                                                                                        crate::krb5_h::krb5_timeofday(context,
                                                                                                                                                       &mut now);
                                                                                                                                    if code
                                                                                                                                           !=
                                                                                                                                           0
                                                                                                                                       {
                                                                                                                                        major_status
                                                                                                                                            =
                                                                                                                                            (13u32)
                                                                                                                                                <<
                                                                                                                                                16i32
                                                                                                                                    } else {
                                                                                                                                        code
                                                                                                                                            =
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
                                                                                                                                        if code
                                                                                                                                               !=
                                                                                                                                               0
                                                                                                                                           {
                                                                                                                                            major_status
                                                                                                                                                =
                                                                                                                                                (13u32)
                                                                                                                                                    <<
                                                                                                                                                    16i32
                                                                                                                                        } else {
                                                                                                                                            /* DCE_STYLE implies mutual authentication */
                                                                                                                                            if (*ctx).gss_flags
                                                                                                                                                   &
                                                                                                                                                   0x1000u32
                                                                                                                                                   !=
                                                                                                                                                   0
                                                                                                                                               {
                                                                                                                                                (*ctx).gss_flags
                                                                                                                                                    |=
                                                                                                                                                    2u32
                                                                                                                                            }
                                                                                                                                            /* at this point, the entire context structure is filled in,
       so it can be released.  */
                                                                                                                                            /* generate an AP_REP if necessary */
                                                                                                                                            if (*ctx).gss_flags
                                                                                                                                                   &
                                                                                                                                                   2u32
                                                                                                                                                   !=
                                                                                                                                                   0
                                                                                                                                               {
                                                                                                                                                let mut ptr3 =
                                                                                                                                                    0
                                                                                                                                                        as
                                                                                                                                                        *mut u8;
                                                                                                                                                let mut seq_temp_0:
                                                                                                                                                        crate::krb5_h::krb5_int32 =
                                                                                                                                                    0;
                                                                                                                                                let mut cfx_generate_subkey:
                                                                                                                                                        i32 =
                                                                                                                                                    0;
                                                                                                                                                /*
         * Do not generate a subkey per RFC 4537 unless we are upgrading to CFX,
         * because pre-CFX tokens do not indicate which key to use. (Note that
         * DCE_STYLE implies that we will use a subkey.)
         */
                                                                                                                                                if (*ctx).proto
                                                                                                                                                       ==
                                                                                                                                                       0i32
                                                                                                                                                       &&
                                                                                                                                                       (*ctx).gss_flags
                                                                                                                                                           &
                                                                                                                                                           0x1000u32
                                                                                                                                                           ==
                                                                                                                                                           0u32
                                                                                                                                                       &&
                                                                                                                                                       ap_req_options
                                                                                                                                                           &
                                                                                                                                                           0x1i32
                                                                                                                                                           !=
                                                                                                                                                           0
                                                                                                                                                   {
                                                                                                                                                    code
                                                                                                                                                        =
                                                                                                                                                        Some(kaccess.auth_con_get_subkey_enctype.expect("non-null function pointer")).expect("non-null function pointer")(context,
                                                                                                                                                                                                                                                                          auth_context,
                                                                                                                                                                                                                                                                          &mut negotiated_etype);
                                                                                                                                                    if code
                                                                                                                                                           !=
                                                                                                                                                           0i32
                                                                                                                                                       {
                                                                                                                                                        major_status
                                                                                                                                                            =
                                                                                                                                                            (13u32)
                                                                                                                                                                <<
                                                                                                                                                                16i32;
                                                                                                                                                        current_block
                                                                                                                                                            =
                                                                                                                                                            6451473480150109090;
                                                                                                                                                    } else {
                                                                                                                                                        match negotiated_etype
                                                                                                                                                            {
                                                                                                                                                            16
                                                                                                                                                            |
                                                                                                                                                            23
                                                                                                                                                            |
                                                                                                                                                            24
                                                                                                                                                            =>
                                                                                                                                                            {
                                                                                                                                                                /* RFC 4121 accidentally omits RC4-HMAC-EXP as a "not-newer"
                 * enctype, even though RFC 4757 treats it as one. */
                                                                                                                                                                ap_req_options
                                                                                                                                                                    &=
                                                                                                                                                                    !(0x1i32)
                                                                                                                                                            }
                                                                                                                                                            _
                                                                                                                                                            =>
                                                                                                                                                            {
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                        current_block
                                                                                                                                                            =
                                                                                                                                                            11353886201549099807;
                                                                                                                                                    }
                                                                                                                                                } else {
                                                                                                                                                    current_block
                                                                                                                                                        =
                                                                                                                                                        11353886201549099807;
                                                                                                                                                }
                                                                                                                                                match current_block
                                                                                                                                                    {
                                                                                                                                                    6451473480150109090
                                                                                                                                                    =>
                                                                                                                                                    {
                                                                                                                                                    }
                                                                                                                                                    _
                                                                                                                                                    =>
                                                                                                                                                    {
                                                                                                                                                        if (*ctx).proto
                                                                                                                                                               ==
                                                                                                                                                               1i32
                                                                                                                                                               ||
                                                                                                                                                               (*ctx).gss_flags
                                                                                                                                                                   &
                                                                                                                                                                   0x1000u32
                                                                                                                                                                   !=
                                                                                                                                                                   0
                                                                                                                                                               ||
                                                                                                                                                               ap_req_options
                                                                                                                                                                   &
                                                                                                                                                                   0x1i32
                                                                                                                                                                   !=
                                                                                                                                                                   0
                                                                                                                                                           {
                                                                                                                                                            cfx_generate_subkey
                                                                                                                                                                =
                                                                                                                                                                1i32
                                                                                                                                                        } else {
                                                                                                                                                            cfx_generate_subkey
                                                                                                                                                                =
                                                                                                                                                                0i32
                                                                                                                                                        }
                                                                                                                                                        if cfx_generate_subkey
                                                                                                                                                               !=
                                                                                                                                                               0
                                                                                                                                                           {
                                                                                                                                                            let mut acflags:
                                                                                                                                                                    crate::krb5_h::krb5_int32 =
                                                                                                                                                                0;
                                                                                                                                                            code
                                                                                                                                                                =
                                                                                                                                                                crate::krb5_h::krb5_auth_con_getflags(context,
                                                                                                                                                                                       auth_context,
                                                                                                                                                                                       &mut acflags);
                                                                                                                                                            if code
                                                                                                                                                                   ==
                                                                                                                                                                   0i32
                                                                                                                                                               {
                                                                                                                                                                acflags
                                                                                                                                                                    |=
                                                                                                                                                                    0x20i32;
                                                                                                                                                                code
                                                                                                                                                                    =
                                                                                                                                                                    crate::krb5_h::krb5_auth_con_setflags(context,
                                                                                                                                                                                           auth_context,
                                                                                                                                                                                           acflags)
                                                                                                                                                            }
                                                                                                                                                            if code
                                                                                                                                                                   !=
                                                                                                                                                                   0
                                                                                                                                                               {
                                                                                                                                                                major_status
                                                                                                                                                                    =
                                                                                                                                                                    (13u32)
                                                                                                                                                                        <<
                                                                                                                                                                        16i32;
                                                                                                                                                                current_block
                                                                                                                                                                    =
                                                                                                                                                                    6451473480150109090;
                                                                                                                                                            } else {
                                                                                                                                                                current_block
                                                                                                                                                                    =
                                                                                                                                                                    316278526493857137;
                                                                                                                                                            }
                                                                                                                                                        } else {
                                                                                                                                                            current_block
                                                                                                                                                                =
                                                                                                                                                                316278526493857137;
                                                                                                                                                        }
                                                                                                                                                        match current_block
                                                                                                                                                            {
                                                                                                                                                            6451473480150109090
                                                                                                                                                            =>
                                                                                                                                                            {
                                                                                                                                                            }
                                                                                                                                                            _
                                                                                                                                                            =>
                                                                                                                                                            {
                                                                                                                                                                code
                                                                                                                                                                    =
                                                                                                                                                                    crate::krb5_h::krb5_mk_rep(context,
                                                                                                                                                                                auth_context,
                                                                                                                                                                                &mut ap_rep);
                                                                                                                                                                if code
                                                                                                                                                                       !=
                                                                                                                                                                       0
                                                                                                                                                                   {
                                                                                                                                                                    major_status
                                                                                                                                                                        =
                                                                                                                                                                        (13u32)
                                                                                                                                                                            <<
                                                                                                                                                                            16i32;
                                                                                                                                                                    current_block
                                                                                                                                                                        =
                                                                                                                                                                        6451473480150109090;
                                                                                                                                                                } else {
                                                                                                                                                                    crate::krb5_h::krb5_auth_con_getlocalseqnumber(context,
                                                                                                                                                                                                    auth_context,
                                                                                                                                                                                                    &mut seq_temp_0);
                                                                                                                                                                    (*ctx).seq_send
                                                                                                                                                                        =
                                                                                                                                                                        (seq_temp_0
                                                                                                                                                                             as
                                                                                                                                                                             isize
                                                                                                                                                                             &
                                                                                                                                                                             0xffffffff
                                                                                                                                                                                 as
                                                                                                                                                                                 isize)
                                                                                                                                                                            as
                                                                                                                                                                            crate::stdlib::uint64_t;
                                                                                                                                                                    if cfx_generate_subkey
                                                                                                                                                                           !=
                                                                                                                                                                           0
                                                                                                                                                                       {
                                                                                                                                                                        /* Get the new acceptor subkey.  With the code above, there
               should always be one if we make it to this point.  */
                                                                                                                                                                        code
                                                                                                                                                                            =
                                                                                                                                                                            crate::krb5_h::krb5_auth_con_getsendsubkey_k(context,
                                                                                                                                                                                                          auth_context,
                                                                                                                                                                                                          &mut (*ctx).acceptor_subkey);
                                                                                                                                                                        if code
                                                                                                                                                                               !=
                                                                                                                                                                               0i32
                                                                                                                                                                           {
                                                                                                                                                                            major_status
                                                                                                                                                                                =
                                                                                                                                                                                (13u32)
                                                                                                                                                                                    <<
                                                                                                                                                                                    16i32;
                                                                                                                                                                            current_block
                                                                                                                                                                                =
                                                                                                                                                                                6451473480150109090;
                                                                                                                                                                        } else {
                                                                                                                                                                            (*ctx).set_have_acceptor_subkey(1u32);
                                                                                                                                                                            code
                                                                                                                                                                                =
                                                                                                                                                                                crate::src::krb5::util_crypt::kg_setup_keys(context,
                                                                                                                                                                                              ctx,
                                                                                                                                                                                              (*ctx).acceptor_subkey,
                                                                                                                                                                                              &mut (*ctx).acceptor_subkey_cksumtype);
                                                                                                                                                                            if code
                                                                                                                                                                                   !=
                                                                                                                                                                                   0
                                                                                                                                                                               {
                                                                                                                                                                                major_status
                                                                                                                                                                                    =
                                                                                                                                                                                    (13u32)
                                                                                                                                                                                        <<
                                                                                                                                                                                        16i32;
                                                                                                                                                                                current_block
                                                                                                                                                                                    =
                                                                                                                                                                                    6451473480150109090;
                                                                                                                                                                            } else {
                                                                                                                                                                                current_block
                                                                                                                                                                                    =
                                                                                                                                                                                    11165373645405170214;
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    } else {
                                                                                                                                                                        current_block
                                                                                                                                                                            =
                                                                                                                                                                            11165373645405170214;
                                                                                                                                                                    }
                                                                                                                                                                    match current_block
                                                                                                                                                                        {
                                                                                                                                                                        6451473480150109090
                                                                                                                                                                        =>
                                                                                                                                                                        {
                                                                                                                                                                        }
                                                                                                                                                                        _
                                                                                                                                                                        =>

                                                                                                                                                                        /* the reply token hasn't been sent yet, but that's ok. */
                                                                                                                                                                        {
                                                                                                                                                                            if (*ctx).gss_flags
                                                                                                                                                                                   &
                                                                                                                                                                                   0x1000u32
                                                                                                                                                                                   !=
                                                                                                                                                                                   0
                                                                                                                                                                               {
                                                                                                                                                                                if (*ctx).have_acceptor_subkey()
                                                                                                                                                                                       !=
                                                                                                                                                                                       0
                                                                                                                                                                                   {
                                                                                                                                                                                } else {
                                                                                                                                                                                    crate::stdlib::__assert_fail(b"ctx->have_acceptor_subkey\x00"
                                                                                                                                                                                                      as
                                                                                                                                                                                                      *const u8
                                                                                                                                                                                                      as
                                                                                                                                                                                                      *const i8,
                                                                                                                                                                                                  b"accept_sec_context.c\x00"
                                                                                                                                                                                                      as
                                                                                                                                                                                                      *const u8
                                                                                                                                                                                                      as
                                                                                                                                                                                                      *const i8,
                                                                                                                                                                                                  1071u32,
                                                                                                                                                                                                  (*::std::mem::transmute::<&[u8; 27],
                                                                                                                                                                                                                            &[i8; 27]>(b"OM_uint32 kg_accept_krb5()\x00")).as_ptr());
                                                                                                                                                                                }
                                                                                                                                                                                /* in order to force acceptor subkey to be used, don't set PROT_READY */
                                                                                                                                                                                /* Raw AP-REP is returned */
                                                                                                                                                                                code
                                                                                                                                                                                    =
                                                                                                                                                                                    data_to_gss(&mut ap_rep,
                                                                                                                                                                                                output_token);
                                                                                                                                                                                if code
                                                                                                                                                                                       !=
                                                                                                                                                                                       0
                                                                                                                                                                                   {
                                                                                                                                                                                    major_status
                                                                                                                                                                                        =
                                                                                                                                                                                        (13u32)
                                                                                                                                                                                            <<
                                                                                                                                                                                            16i32
                                                                                                                                                                                } else {
                                                                                                                                                                                    (*ctx).set_established(0u32);
                                                                                                                                                                                    *context_handle
                                                                                                                                                                                        =
                                                                                                                                                                                        ctx
                                                                                                                                                                                            as
                                                                                                                                                                                            crate::gssapi_h::gss_ctx_id_t;
                                                                                                                                                                                    *minor_status
                                                                                                                                                                                        =
                                                                                                                                                                                        0u32;
                                                                                                                                                                                    major_status
                                                                                                                                                                                        =
                                                                                                                                                                                        ((1i32)
                                                                                                                                                                                             <<
                                                                                                                                                                                             0i32
                                                                                                                                                                                                 +
                                                                                                                                                                                                 0i32)
                                                                                                                                                                                            as
                                                                                                                                                                                            crate::gssapi_h::OM_uint32
                                                                                                                                                                                }
                                                                                                                                                                                current_block
                                                                                                                                                                                    =
                                                                                                                                                                                    6451473480150109090;
                                                                                                                                                                            } else {
                                                                                                                                                                                (*ctx).gss_flags
                                                                                                                                                                                    |=
                                                                                                                                                                                    128u32;
                                                                                                                                                                                (*ctx).set_established(1u32);
                                                                                                                                                                                token.length
                                                                                                                                                                                    =
                                                                                                                                                                                    crate::src::generic::util_token::gssint_g_token_size(mech_used,
                                                                                                                                                                                                        ap_rep.length)
                                                                                                                                                                                        as
                                                                                                                                                                                        crate::stddef_h::size_t;
                                                                                                                                                                                token.value = gssalloc_malloc(token.length);
                                                                                                                                                                                if token.value.is_null()
                                                                                                                                                                                   {
                                                                                                                                                                                    major_status = (13u32) << 16i32;
                                                                                                                                                                                    code = 12i32;
                                                                                                                                                                                    current_block = 6451473480150109090;
                                                                                                                                                                                } else {
                                                                                                                                                                                    ptr3
                                                                                                                                                                                        =
                                                                                                                                                                                        token.value
                                                                                                                                                                                            as
                                                                                                                                                                                            *mut u8;
                                                                                                                                                                                    crate::src::generic::util_token::gssint_g_make_token_header(mech_used,
                                                                                                                                                                                                               ap_rep.length,
                                                                                                                                                                                                               &mut ptr3,
                                                                                                                                                                                                               0x200i32);
                                                                                                                                                                                    crate::stdlib::memcpy(ptr3
                                                                                                                                                                                               as
                                                                                                                                                                                               *mut libc::c_void,
                                                                                                                                                                                           ap_rep.data
                                                                                                                                                                                               as
                                                                                                                                                                                               *const libc::c_void,
                                                                                                                                                                                           ap_rep.length
                                                                                                                                                                                               as
                                                                                                                                                                                               usize);
                                                                                                                                                                                    ptr3
                                                                                                                                                                                        =
                                                                                                                                                                                        ptr3.offset(ap_rep.length
                                                                                                                                                                                                        as
                                                                                                                                                                                                        isize);
                                                                                                                                                                                    (*ctx).set_established(1u32);
                                                                                                                                                                                    current_block
                                                                                                                                                                                        =
                                                                                                                                                                                        11729964750550375795;
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                token.length
                                                                                                                                                    =
                                                                                                                                                    0usize;
                                                                                                                                                token.value
                                                                                                                                                    =
                                                                                                                                                    0
                                                                                                                                                        as
                                                                                                                                                        *mut libc::c_void;
                                                                                                                                                (*ctx).seq_send
                                                                                                                                                    =
                                                                                                                                                    (*ctx).seq_recv;
                                                                                                                                                (*ctx).set_established(1u32);
                                                                                                                                                current_block
                                                                                                                                                    =
                                                                                                                                                    11729964750550375795;
                                                                                                                                            }
                                                                                                                                            match current_block
                                                                                                                                                {
                                                                                                                                                6451473480150109090
                                                                                                                                                =>
                                                                                                                                                {
                                                                                                                                                }
                                                                                                                                                _
                                                                                                                                                =>

                                                                                                                                                /* set the return arguments */
                                                                                                                                                {
                                                                                                                                                    if !src_name.is_null()
                                                                                                                                                       {
                                                                                                                                                        code
                                                                                                                                                            =
                                                                                                                                                            crate::src::krb5::naming_exts::kg_duplicate_name(context,
                                                                                                                                                                              (*ctx).there,
                                                                                                                                                                              &mut name);
                                                                                                                                                        if code
                                                                                                                                                               !=
                                                                                                                                                               0
                                                                                                                                                           {
                                                                                                                                                            major_status
                                                                                                                                                                =
                                                                                                                                                                (13u32)
                                                                                                                                                                    <<
                                                                                                                                                                    16i32;
                                                                                                                                                            current_block
                                                                                                                                                                =
                                                                                                                                                                6451473480150109090;
                                                                                                                                                        } else {
                                                                                                                                                            current_block
                                                                                                                                                                =
                                                                                                                                                                10648164479545198704;
                                                                                                                                                        }
                                                                                                                                                    } else {
                                                                                                                                                        current_block
                                                                                                                                                            =
                                                                                                                                                            10648164479545198704;
                                                                                                                                                    }
                                                                                                                                                    match current_block
                                                                                                                                                        {
                                                                                                                                                        6451473480150109090
                                                                                                                                                        =>
                                                                                                                                                        {
                                                                                                                                                        }
                                                                                                                                                        _
                                                                                                                                                        =>
                                                                                                                                                        {
                                                                                                                                                            if !mech_type.is_null()
                                                                                                                                                               {
                                                                                                                                                                *mech_type
                                                                                                                                                                    =
                                                                                                                                                                    mech_used
                                                                                                                                                                        as
                                                                                                                                                                        crate::gssapi_h::gss_OID
                                                                                                                                                            }
                                                                                                                                                            /* Add the maximum allowable clock skew as a grace period for context
     * expiration, just as we do for the ticket. */
                                                                                                                                                            if !time_rec.is_null()
                                                                                                                                                               {
                                                                                                                                                                *time_rec
                                                                                                                                                                    =
                                                                                                                                                                    (ts_delta((*ctx).krb_times.endtime,
                                                                                                                                                                              now)
                                                                                                                                                                         +
                                                                                                                                                                         (*context).clockskew)
                                                                                                                                                                        as
                                                                                                                                                                        crate::gssapi_h::OM_uint32
                                                                                                                                                            }
                                                                                                                                                            if !ret_flags.is_null()
                                                                                                                                                               {
                                                                                                                                                                *ret_flags
                                                                                                                                                                    =
                                                                                                                                                                    (*ctx).gss_flags
                                                                                                                                                            }
                                                                                                                                                            *context_handle
                                                                                                                                                                =
                                                                                                                                                                ctx
                                                                                                                                                                    as
                                                                                                                                                                    crate::gssapi_h::gss_ctx_id_t;
                                                                                                                                                            *output_token
                                                                                                                                                                =
                                                                                                                                                                token;
                                                                                                                                                            if !src_name.is_null()
                                                                                                                                                               {
                                                                                                                                                                *src_name
                                                                                                                                                                    =
                                                                                                                                                                    name
                                                                                                                                                                        as
                                                                                                                                                                        crate::gssapi_h::gss_name_t
                                                                                                                                                            }
                                                                                                                                                            if !delegated_cred_handle.is_null()
                                                                                                                                                               {
                                                                                                                                                                *delegated_cred_handle
                                                                                                                                                                    =
                                                                                                                                                                    deleg_cred
                                                                                                                                                                        as
                                                                                                                                                                        crate::gssapi_h::gss_cred_id_t
                                                                                                                                                            }
                                                                                                                                                            /* finally! */
                                                                                                                                                            *minor_status
                                                                                                                                                                =
                                                                                                                                                                0u32;
                                                                                                                                                            major_status
                                                                                                                                                                =
                                                                                                                                                                0u32
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
                                current_block = 6451473480150109090;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        6451473480150109090 =>
        /* Only last leg should set return arguments */
        {
            if !authdat.is_null() {
                crate::krb5_h::krb5_free_authenticator(context, authdat);
            }
            /* The ctx structure has the handle of the auth_context */
            if !auth_context.is_null() && ctx.is_null() {
                if cred_rcache != 0 {
                    crate::krb5_h::krb5_auth_con_setrcache(
                        context,
                        auth_context,
                        0 as crate::krb5_h::krb5_rcache,
                    );
                }
                crate::krb5_h::krb5_auth_con_free(context, auth_context);
            }
            if !reqcksum.contents.is_null() {
                crate::stdlib::free(reqcksum.contents as *mut libc::c_void);
            }
            if !ap_rep.data.is_null() {
                crate::krb5_h::krb5_free_data_contents(context, &mut ap_rep);
            }
            if major_status == 0u32
                || major_status == ((1i32) << 0i32 + 0i32) as u32
                    && code as isize != -(1765328344 as isize)
            {
                (*ctx).k5_context = context;
                context = 0 as crate::krb5_h::krb5_context
            } else {
                /* from here on is the real "fail" code */
                if !ctx.is_null() {
                    crate::src::krb5::delete_sec_context::krb5_gss_delete_sec_context(
                        &mut tmp_minor_status,
                        &mut ctx as *mut *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec
                            as *mut crate::gssapi_h::gss_ctx_id_t,
                        0 as crate::gssapi_h::gss_buffer_t,
                    );
                }
                if !deleg_cred.is_null() {
                    /* free memory associated with the deleg credential */
                    if !(*deleg_cred).ccache.is_null() {
                        crate::krb5_h::krb5_cc_close(context, (*deleg_cred).ccache);
                    }
                    if !(*deleg_cred).name.is_null() {
                        crate::src::krb5::naming_exts::kg_release_name(
                            context,
                            &mut (*deleg_cred).name,
                        );
                    }
                    crate::stdlib::free(deleg_cred as *mut libc::c_void);
                }
                if !token.value.is_null() {
                    crate::stdlib::free(token.value);
                }
                if !name.is_null() {
                    crate::src::krb5::naming_exts::kg_release_name(context, &mut name);
                }
                *minor_status = code as crate::gssapi_h::OM_uint32;
                /* We may have failed before being able to read the GSS flags from the
                 * authenticator, so also check the request AP options. */
                if !cred.is_null()
                    && !request.is_null()
                    && (gss_flags & 2u32 != 0
                        || (*request).ap_options & 0x20000000i32 != 0
                        || major_status == ((1i32) << 0i32 + 0i32) as u32)
                {
                    let mut tmsglen: u32 = 0;
                    let mut toktype: i32 = 0;
                    /*
                     * The client is expecting a response, so we can send an
                     * error token back
                     */
                    crate::stdlib::memset(
                        &mut krb_error_data as *mut crate::krb5_h::krb5_error as *mut libc::c_void,
                        0i32,
                        ::std::mem::size_of::<crate::krb5_h::krb5_error>(),
                    ); /* KRB_ERR_GENERIC */
                    code =
                        (code as isize - -(1765328384 as isize)) as crate::krb5_h::krb5_error_code;
                    if code < 0i32 || code > 127i32 {
                        code = 60i32
                    }
                    krb_error_data.error = code as crate::krb5_h::krb5_ui_4;
                    crate::krb5_h::krb5_us_timeofday(
                        context,
                        &mut krb_error_data.stime,
                        &mut krb_error_data.susec,
                    );
                    krb_error_data.server = (*ticket).server;
                    code = crate::krb5_h::krb5_mk_error(context, &mut krb_error_data, &mut scratch);
                    if !(code != 0) {
                        tmsglen = scratch.length;
                        toktype = 0x300i32;
                        token.length = crate::src::generic::util_token::gssint_g_token_size(
                            mech_used, tmsglen,
                        ) as crate::stddef_h::size_t;
                        token.value = gssalloc_malloc(token.length);
                        if !token.value.is_null() {
                            ptr = token.value as *mut u8;
                            crate::src::generic::util_token::gssint_g_make_token_header(
                                mech_used, tmsglen, &mut ptr, toktype,
                            );
                            crate::stdlib::memcpy(
                                ptr as *mut libc::c_void,
                                scratch.data as *const libc::c_void,
                                scratch.length as usize,
                            );
                            ptr = ptr.offset(scratch.length as isize);
                            crate::krb5_h::krb5_free_data_contents(context, &mut scratch);
                            *output_token = token
                        }
                    }
                }
            }
        }
        _ => {}
    }
    crate::k5_int_h::krb5_free_ap_req(context, request);
    if !cred.is_null() {
        k5_mutex_unlock(&mut (*cred).lock);
    }
    if !defcred.is_null() {
        crate::src::krb5::rel_cred::krb5_gss_release_cred(&mut tmp_minor_status, &mut defcred);
    }
    if !context.is_null() {
        if major_status != 0 && *minor_status != 0 {
            crate::src::krb5::disp_status::krb5_gss_save_error_info(*minor_status, context);
        }
        crate::krb5_h::krb5_free_context(context);
    }
    return major_status;
}
/* LEAN_CLIENT */
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_accept_sec_context_ext(
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
    mut exts: crate::gssapiP_krb5_h::krb5_gss_ctx_ext_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut ctx = *context_handle as *mut crate::gssapiP_krb5_h::krb5_gss_ctx_id_rec;
    /*
     * Context handle must be unspecified.  Actually, it must be
     * non-established, but currently, accept_sec_context never returns
     * a non-established context handle.
     */
    /*SUPPRESS 29*/
    if !ctx.is_null() {
        if (*ctx).established() as i32 == 0i32 && (*ctx).gss_flags & 0x1000u32 != 0 {
            return kg_accept_dce(
                minor_status,
                context_handle,
                verifier_cred_handle,
                input_token,
                input_chan_bindings,
                src_name,
                mech_type,
                output_token,
                ret_flags,
                time_rec,
                delegated_cred_handle,
            );
        } else {
            *minor_status = 22u32;
            crate::src::krb5::disp_status::krb5_gss_save_error_string(
                22u32,
                b"accept_sec_context called with existing context handle\x00" as *const u8
                    as *mut i8,
            );
            return (13u32) << 16i32;
        }
    }
    return kg_accept_krb5(
        minor_status,
        context_handle,
        verifier_cred_handle,
        input_token,
        input_chan_bindings,
        src_name,
        mech_type,
        output_token,
        ret_flags,
        time_rec,
        delegated_cred_handle,
        exts,
    );
}
#[no_mangle]

pub unsafe extern "C" fn krb5_gss_accept_sec_context(
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
    return krb5_gss_accept_sec_context_ext(
        minor_status,
        context_handle,
        verifier_cred_handle,
        input_token,
        input_chan_bindings,
        src_name,
        mech_type,
        output_token,
        ret_flags,
        time_rec,
        delegated_cred_handle,
        &mut exts,
    );
}
