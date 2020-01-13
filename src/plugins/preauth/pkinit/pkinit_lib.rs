use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    use super::types_h::__uint8_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:32"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:32"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:32"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "180:1"]
    pub type krb5_cksumtype = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "391:16"]
    pub struct _krb5_checksum {
        pub magic: krb5_magic,
        pub checksum_type: krb5_cksumtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "391:1"]
    pub type krb5_checksum = _krb5_checksum;
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        /* per Kerberos v5 protocol spec */
        /* not yet in the spec... */
        /* macros to determine if a type is a local type */
        /*
 * end "hostaddr.h"
 */
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        /* *
 * Free a krb5_data structure.
 *
 * @param [in] context          Library context
 * @param [in] val              Data structure to be freed
 *
 * This function frees the contents of @a val and the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4743:1"]
        pub fn krb5_free_data(context: krb5_context, val: *mut krb5_data);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int-pkinit.h:32"]
pub mod k5_int_pkinit_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2006
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
    /*
 * pkinit structures
 */
    /* PKAuthenticator */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:16"]
    pub struct _krb5_pk_authenticator {
        pub cusec: krb5_int32,
        pub ctime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub paChecksum: krb5_checksum,
        pub freshnessToken: *mut krb5_data,
    }
    #[c2rust::src_loc = "40:1"]
    pub type krb5_pk_authenticator = _krb5_pk_authenticator;
    /* AlgorithmIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:16"]
    pub struct _krb5_algorithm_identifier {
        pub algorithm: krb5_data,
        pub parameters: krb5_data,
    }
    #[c2rust::src_loc = "49:1"]
    pub type krb5_algorithm_identifier = _krb5_algorithm_identifier;
    /* Optional */
    /* SubjectPublicKeyInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:16"]
    pub struct _krb5_subject_pk_info {
        pub algorithm: krb5_algorithm_identifier,
        pub subjectPublicKey: krb5_data,
    }
    #[c2rust::src_loc = "55:1"]
    pub type krb5_subject_pk_info = _krb5_subject_pk_info;
    /* BIT STRING */
    /* * AuthPack from RFC 4556*/
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:16"]
    pub struct _krb5_auth_pack {
        pub pkAuthenticator: krb5_pk_authenticator,
        pub clientPublicValue: *mut krb5_subject_pk_info,
        pub supportedCMSTypes: *mut *mut krb5_algorithm_identifier,
        pub clientDHNonce: krb5_data,
        pub supportedKDFs: *mut *mut krb5_data,
    }
    #[c2rust::src_loc = "61:1"]
    pub type krb5_auth_pack = _krb5_auth_pack;
    /* OIDs of KDFs; OPTIONAL */
    /* ExternalPrincipalIdentifier */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "70:16"]
    pub struct _krb5_external_principal_identifier {
        pub subjectName: krb5_data,
        pub issuerAndSerialNumber: krb5_data,
        pub subjectKeyIdentifier: krb5_data,
    }
    #[c2rust::src_loc = "70:1"]
    pub type krb5_external_principal_identifier
        =
        _krb5_external_principal_identifier;
    /* Optional */
    /* PA-PK-AS-REQ (rfc4556 -- PA TYPE 16) */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:16"]
    pub struct _krb5_pa_pk_as_req {
        pub signedAuthPack: krb5_data,
        pub trustedCertifiers: *mut *mut krb5_external_principal_identifier,
        pub kdcPkId: krb5_data,
    }
    #[c2rust::src_loc = "77:1"]
    pub type krb5_pa_pk_as_req = _krb5_pa_pk_as_req;
    /* Optional */
    /* * Pkinit DHRepInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:16"]
    pub struct _krb5_dh_rep_info {
        pub dhSignedData: krb5_data,
        pub serverDHNonce: krb5_data,
        pub kdfID: *mut krb5_data,
    }
    #[c2rust::src_loc = "84:1"]
    pub type krb5_dh_rep_info = _krb5_dh_rep_info;
    /* OID of selected KDF OPTIONAL */
    /* KDCDHKeyInfo */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:16"]
    pub struct _krb5_kdc_dh_key_info {
        pub subjectPublicKey: krb5_data,
        pub nonce: krb5_int32,
        pub dhKeyExpiration: krb5_timestamp,
    }
    #[c2rust::src_loc = "91:1"]
    pub type krb5_kdc_dh_key_info = _krb5_kdc_dh_key_info;
    /* Optional */
    /* ReplyKeyPack */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:16"]
    pub struct _krb5_reply_key_pack {
        pub replyKey: krb5_keyblock,
        pub asChecksum: krb5_checksum,
    }
    #[c2rust::src_loc = "98:1"]
    pub type krb5_reply_key_pack = _krb5_reply_key_pack;
    /* PA-PK-AS-REP (rfc4556 -- PA TYPE 17) */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "104:16"]
    pub struct _krb5_pa_pk_as_rep {
        pub choice: krb5_pa_pk_as_rep_selection,
        pub u: krb5_pa_pk_as_rep_choices,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:11"]
    pub union krb5_pa_pk_as_rep_choices {
        pub dh_Info: krb5_dh_rep_info,
        pub encKeyPack: krb5_data,
    }
    #[c2rust::src_loc = "105:5"]
    pub type krb5_pa_pk_as_rep_selection = libc::c_int;
    #[c2rust::src_loc = "108:9"]
    pub const choice_pa_pk_as_rep_encKeyPack: krb5_pa_pk_as_rep_selection = 1;
    #[c2rust::src_loc = "107:9"]
    pub const choice_pa_pk_as_rep_dhInfo: krb5_pa_pk_as_rep_selection = 0;
    #[c2rust::src_loc = "106:9"]
    pub const choice_pa_pk_as_rep_UNKNOWN: krb5_pa_pk_as_rep_selection = -1;
    #[c2rust::src_loc = "104:1"]
    pub type krb5_pa_pk_as_rep = _krb5_pa_pk_as_rep;
    use super::krb5_h::{krb5_int32, krb5_timestamp, krb5_checksum, krb5_data,
                        krb5_keyblock};
    /* _KRB5_INT_PKINIT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/plugins/preauth/pkinit/pkinit.h:32"]
pub mod pkinit_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "146:16"]
    pub struct _pkinit_plg_opts {
        pub require_eku: libc::c_int,
        pub accept_secondary_eku: libc::c_int,
        pub allow_upn: libc::c_int,
        pub dh_or_rsa: libc::c_int,
        pub require_crl_checking: libc::c_int,
        pub require_freshness: libc::c_int,
        pub disable_freshness: libc::c_int,
        pub dh_min_bits: libc::c_int,
    }
    #[c2rust::src_loc = "146:1"]
    pub type pkinit_plg_opts = _pkinit_plg_opts;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "160:16"]
    pub struct _pkinit_req_opts {
        pub require_eku: libc::c_int,
        pub accept_secondary_eku: libc::c_int,
        pub allow_upn: libc::c_int,
        pub dh_or_rsa: libc::c_int,
        pub require_crl_checking: libc::c_int,
        pub dh_size: libc::c_int,
        pub require_hostname_match: libc::c_int,
        pub disable_freshness: libc::c_int,
    }
    #[c2rust::src_loc = "160:1"]
    pub type pkinit_req_opts = _pkinit_req_opts;
    #[inline]
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn pkiDebug(mut fmt: *const libc::c_char,
                                      mut args: ...) {
    }
    /* _PKINIT_H */
    /*
 * Now get crypto function declarations
 */
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:32"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "521:1"]
        pub fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "786:1"]
        pub fn fileno(__stream: *mut FILE) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:32"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __int32_t, __off_t, __off64_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint8_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_enctype, krb5_cksumtype,
                       krb5_timestamp, krb5_error_code, krb5_magic,
                       _krb5_data, krb5_data, krb5_context, _krb5_keyblock,
                       krb5_keyblock, _krb5_checksum, krb5_checksum,
                       _krb5_context, krb5_free_data};
pub use self::k5_int_pkinit_h::{_krb5_pk_authenticator, krb5_pk_authenticator,
                                _krb5_algorithm_identifier,
                                krb5_algorithm_identifier,
                                _krb5_subject_pk_info, krb5_subject_pk_info,
                                _krb5_auth_pack, krb5_auth_pack,
                                _krb5_external_principal_identifier,
                                krb5_external_principal_identifier,
                                _krb5_pa_pk_as_req, krb5_pa_pk_as_req,
                                _krb5_dh_rep_info, krb5_dh_rep_info,
                                _krb5_kdc_dh_key_info, krb5_kdc_dh_key_info,
                                _krb5_reply_key_pack, krb5_reply_key_pack,
                                _krb5_pa_pk_as_rep, krb5_pa_pk_as_rep_choices,
                                krb5_pa_pk_as_rep_selection,
                                choice_pa_pk_as_rep_encKeyPack,
                                choice_pa_pk_as_rep_dhInfo,
                                choice_pa_pk_as_rep_UNKNOWN,
                                krb5_pa_pk_as_rep};
pub use self::pkinit_h::{_pkinit_plg_opts, pkinit_plg_opts, _pkinit_req_opts,
                         pkinit_req_opts, pkiDebug};
use self::stdlib_h::{malloc, calloc, free};
use self::stdio_h::{fclose, fopen, fputc, fileno};
use self::fcntl_h::fcntl;
use self::string_h::memcpy;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * COPYRIGHT (C) 2006,2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
#[no_mangle]
#[c2rust::src_loc = "36:17"]
pub static mut dh_oid: krb5_data =
    {
        let mut init =
            _krb5_data{magic: 0 as libc::c_int,
                       length: 7 as libc::c_int as libc::c_uint,
                       data:
                           b"*\x86H\xce>\x02\x01\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn pkinit_init_req_opts(mut reqopts:
                                                  *mut *mut pkinit_req_opts)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut opts: *mut pkinit_req_opts = 0 as *mut pkinit_req_opts;
    *reqopts = 0 as *mut pkinit_req_opts;
    opts =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<pkinit_req_opts>() as libc::c_ulong) as
            *mut pkinit_req_opts;
    if opts.is_null() { return retval }
    (*opts).require_eku = 1 as libc::c_int;
    (*opts).accept_secondary_eku = 0 as libc::c_int;
    (*opts).allow_upn = 0 as libc::c_int;
    (*opts).dh_or_rsa = 1 as libc::c_int;
    (*opts).require_crl_checking = 0 as libc::c_int;
    (*opts).dh_size = 2048 as libc::c_int;
    *reqopts = opts;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn pkinit_fini_req_opts(mut opts:
                                                  *mut pkinit_req_opts) {
    free(opts as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "69:1"]
pub unsafe extern "C" fn pkinit_init_plg_opts(mut plgopts:
                                                  *mut *mut pkinit_plg_opts)
 -> krb5_error_code {
    let mut retval: krb5_error_code = 12 as libc::c_int;
    let mut opts: *mut pkinit_plg_opts = 0 as *mut pkinit_plg_opts;
    *plgopts = 0 as *mut pkinit_plg_opts;
    opts =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<pkinit_plg_opts>() as libc::c_ulong) as
            *mut pkinit_plg_opts;
    if opts.is_null() { return retval }
    (*opts).require_eku = 1 as libc::c_int;
    (*opts).accept_secondary_eku = 0 as libc::c_int;
    (*opts).dh_or_rsa = 1 as libc::c_int;
    (*opts).allow_upn = 0 as libc::c_int;
    (*opts).require_crl_checking = 0 as libc::c_int;
    (*opts).require_freshness = 0 as libc::c_int;
    (*opts).disable_freshness = 0 as libc::c_int;
    (*opts).dh_min_bits = 2048 as libc::c_int;
    *plgopts = opts;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "95:1"]
pub unsafe extern "C" fn pkinit_fini_plg_opts(mut opts:
                                                  *mut pkinit_plg_opts) {
    free(opts as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "102:1"]
pub unsafe extern "C" fn free_krb5_pa_pk_as_req(mut in_0:
                                                    *mut *mut krb5_pa_pk_as_req) {
    if (*in_0).is_null() { return }
    free((**in_0).signedAuthPack.data as *mut libc::c_void);
    if !(**in_0).trustedCertifiers.is_null() {
        free_krb5_external_principal_identifier(&mut (**in_0).trustedCertifiers);
    }
    free((**in_0).kdcPkId.data as *mut libc::c_void);
    free(*in_0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn free_krb5_reply_key_pack(mut in_0:
                                                      *mut *mut krb5_reply_key_pack) {
    if (*in_0).is_null() { return }
    free((**in_0).replyKey.contents as *mut libc::c_void);
    free((**in_0).asChecksum.contents as *mut libc::c_void);
    free(*in_0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "122:1"]
pub unsafe extern "C" fn free_krb5_auth_pack(mut in_0:
                                                 *mut *mut krb5_auth_pack) {
    if (*in_0).is_null() { return }
    if !(**in_0).clientPublicValue.is_null() {
        free((*(**in_0).clientPublicValue).algorithm.algorithm.data as
                 *mut libc::c_void);
        free((*(**in_0).clientPublicValue).algorithm.parameters.data as
                 *mut libc::c_void);
        free((*(**in_0).clientPublicValue).subjectPublicKey.data as
                 *mut libc::c_void);
        free((**in_0).clientPublicValue as *mut libc::c_void);
    }
    free((**in_0).pkAuthenticator.paChecksum.contents as *mut libc::c_void);
    krb5_free_data(0 as krb5_context,
                   (**in_0).pkAuthenticator.freshnessToken);
    if !(**in_0).supportedCMSTypes.is_null() {
        free_krb5_algorithm_identifiers(&mut (**in_0).supportedCMSTypes);
    }
    if !(**in_0).supportedKDFs.is_null() {
        let mut supportedKDFs: *mut *mut krb5_data = (**in_0).supportedKDFs;
        let mut i: libc::c_uint = 0;
        i = 0 as libc::c_int as libc::c_uint;
        while !(*supportedKDFs.offset(i as isize)).is_null() {
            krb5_free_data(0 as krb5_context,
                           *supportedKDFs.offset(i as isize));
            i = i.wrapping_add(1)
        }
        free(supportedKDFs as *mut libc::c_void);
    }
    free(*in_0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "146:1"]
pub unsafe extern "C" fn free_krb5_pa_pk_as_rep(mut in_0:
                                                    *mut *mut krb5_pa_pk_as_rep) {
    if (*in_0).is_null() { return }
    match (**in_0).choice as libc::c_int {
        0 => {
            krb5_free_data(0 as krb5_context, (**in_0).u.dh_Info.kdfID);
            free((**in_0).u.dh_Info.dhSignedData.data as *mut libc::c_void);
        }
        1 => { free((**in_0).u.encKeyPack.data as *mut libc::c_void); }
        _ => { }
    }
    free(*in_0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "164:1"]
pub unsafe extern "C" fn free_krb5_external_principal_identifier(mut in_0:
                                                                     *mut *mut *mut krb5_external_principal_identifier) {
    let mut i: libc::c_int = 0 as libc::c_int;
    if (*in_0).is_null() { return }
    while !(*(*in_0).offset(i as isize)).is_null() {
        free((**(*in_0).offset(i as isize)).subjectName.data as
                 *mut libc::c_void);
        free((**(*in_0).offset(i as isize)).issuerAndSerialNumber.data as
                 *mut libc::c_void);
        free((**(*in_0).offset(i as isize)).subjectKeyIdentifier.data as
                 *mut libc::c_void);
        free(*(*in_0).offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free(*in_0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "179:1"]
pub unsafe extern "C" fn free_krb5_algorithm_identifier(mut in_0:
                                                            *mut krb5_algorithm_identifier) {
    if in_0.is_null() { return }
    free((*in_0).algorithm.data as *mut libc::c_void);
    free((*in_0).parameters.data as *mut libc::c_void);
    free(in_0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "189:1"]
pub unsafe extern "C" fn free_krb5_algorithm_identifiers(mut in_0:
                                                             *mut *mut *mut krb5_algorithm_identifier) {
    let mut i: libc::c_int = 0;
    if in_0.is_null() || (*in_0).is_null() { return }
    i = 0 as libc::c_int;
    while !(*(*in_0).offset(i as isize)).is_null() {
        free_krb5_algorithm_identifier(*(*in_0).offset(i as isize));
        i += 1
    }
    free(*in_0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "201:1"]
pub unsafe extern "C" fn free_krb5_subject_pk_info(mut in_0:
                                                       *mut *mut krb5_subject_pk_info) {
    if (*in_0).is_null() { return }
    free((**in_0).algorithm.parameters.data as *mut libc::c_void);
    free((**in_0).subjectPublicKey.data as *mut libc::c_void);
    free(*in_0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "210:1"]
pub unsafe extern "C" fn free_krb5_kdc_dh_key_info(mut in_0:
                                                       *mut *mut krb5_kdc_dh_key_info) {
    if (*in_0).is_null() { return }
    free((**in_0).subjectPublicKey.data as *mut libc::c_void);
    free(*in_0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "218:1"]
pub unsafe extern "C" fn init_krb5_pa_pk_as_req(mut in_0:
                                                    *mut *mut krb5_pa_pk_as_req) {
    *in_0 =
        malloc(::std::mem::size_of::<krb5_pa_pk_as_req>() as libc::c_ulong) as
            *mut krb5_pa_pk_as_req;
    if (*in_0).is_null() { return }
    (**in_0).signedAuthPack.data = 0 as *mut libc::c_char;
    (**in_0).signedAuthPack.length = 0 as libc::c_int as libc::c_uint;
    (**in_0).trustedCertifiers =
        0 as *mut *mut krb5_external_principal_identifier;
    (**in_0).kdcPkId.data = 0 as *mut libc::c_char;
    (**in_0).kdcPkId.length = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
#[c2rust::src_loc = "230:1"]
pub unsafe extern "C" fn init_krb5_reply_key_pack(mut in_0:
                                                      *mut *mut krb5_reply_key_pack) {
    *in_0 =
        malloc(::std::mem::size_of::<krb5_reply_key_pack>() as libc::c_ulong)
            as *mut krb5_reply_key_pack;
    if (*in_0).is_null() { return }
    (**in_0).replyKey.contents = 0 as *mut krb5_octet;
    (**in_0).replyKey.length = 0 as libc::c_int as libc::c_uint;
    (**in_0).asChecksum.contents = 0 as *mut krb5_octet;
    (**in_0).asChecksum.length = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
#[c2rust::src_loc = "241:1"]
pub unsafe extern "C" fn init_krb5_pa_pk_as_rep(mut in_0:
                                                    *mut *mut krb5_pa_pk_as_rep) {
    *in_0 =
        malloc(::std::mem::size_of::<krb5_pa_pk_as_rep>() as libc::c_ulong) as
            *mut krb5_pa_pk_as_rep;
    if (*in_0).is_null() { return }
    (**in_0).u.dh_Info.serverDHNonce.length =
        0 as libc::c_int as libc::c_uint;
    (**in_0).u.dh_Info.serverDHNonce.data = 0 as *mut libc::c_char;
    (**in_0).u.dh_Info.dhSignedData.length = 0 as libc::c_int as libc::c_uint;
    (**in_0).u.dh_Info.dhSignedData.data = 0 as *mut libc::c_char;
    (**in_0).u.encKeyPack.length = 0 as libc::c_int as libc::c_uint;
    (**in_0).u.encKeyPack.data = 0 as *mut libc::c_char;
    (**in_0).u.dh_Info.kdfID = 0 as *mut krb5_data;
}
#[no_mangle]
#[c2rust::src_loc = "255:1"]
pub unsafe extern "C" fn init_krb5_subject_pk_info(mut in_0:
                                                       *mut *mut krb5_subject_pk_info) {
    *in_0 =
        malloc(::std::mem::size_of::<krb5_subject_pk_info>() as libc::c_ulong)
            as *mut krb5_subject_pk_info;
    if (*in_0).is_null() { return }
    (**in_0).algorithm.parameters.data = 0 as *mut libc::c_char;
    (**in_0).algorithm.parameters.length = 0 as libc::c_int as libc::c_uint;
    (**in_0).subjectPublicKey.data = 0 as *mut libc::c_char;
    (**in_0).subjectPublicKey.length = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
#[c2rust::src_loc = "266:1"]
pub unsafe extern "C" fn pkinit_copy_krb5_data(mut dst: *mut krb5_data,
                                               mut src: *const krb5_data)
 -> krb5_error_code {
    if dst.is_null() || src.is_null() { return 22 as libc::c_int }
    if (*src).data.is_null() {
        (*dst).data = 0 as *mut libc::c_char;
        (*dst).length = 0 as libc::c_int as libc::c_uint;
        return 0 as libc::c_int
    }
    (*dst).data = malloc((*src).length as libc::c_ulong) as *mut libc::c_char;
    if (*dst).data.is_null() { return 12 as libc::c_int }
    memcpy((*dst).data as *mut libc::c_void,
           (*src).data as *const libc::c_void,
           (*src).length as libc::c_ulong);
    (*dst).length = (*src).length;
    return 0 as libc::c_int;
}
/* debugging functions */
#[no_mangle]
#[c2rust::src_loc = "285:1"]
pub unsafe extern "C" fn print_buffer(mut buf: *const libc::c_uchar,
                                      mut len: libc::c_uint) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if len <= 0 as libc::c_int as libc::c_uint { return }
    i = 0 as libc::c_int as libc::c_uint;
    while i < len {
        pkiDebug(b"%02x \x00" as *const u8 as *const libc::c_char,
                 *buf.offset(i as isize) as libc::c_int);
        i = i.wrapping_add(1)
    }
    pkiDebug(b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
 * COPYRIGHT (C) 2006,2007
 * THE REGENTS OF THE UNIVERSITY OF MICHIGAN
 * ALL RIGHTS RESERVED
 *
 * Permission is granted to use, copy, create derivative works
 * and redistribute this software and such derivative works
 * for any purpose, so long as the name of The University of
 * Michigan is not used in any advertising or publicity
 * pertaining to the use of distribution of this software
 * without specific, written prior authorization.  If the
 * above copyright notice or any other identification of the
 * University of Michigan is included in any copy of any
 * portion of this software, then the disclaimer below must
 * also be included.
 *
 * THIS SOFTWARE IS PROVIDED AS IS, WITHOUT REPRESENTATION
 * FROM THE UNIVERSITY OF MICHIGAN AS TO ITS FITNESS FOR ANY
 * PURPOSE, AND WITHOUT WARRANTY BY THE UNIVERSITY OF
 * MICHIGAN OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING
 * WITHOUT LIMITATION THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE
 * REGENTS OF THE UNIVERSITY OF MICHIGAN SHALL NOT BE LIABLE
 * FOR ANY DAMAGES, INCLUDING SPECIAL, INDIRECT, INCIDENTAL, OR
 * CONSEQUENTIAL DAMAGES, WITH RESPECT TO ANY CLAIM ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OF THE SOFTWARE, EVEN
 * IF IT HAS BEEN OR IS HEREAFTER ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGES.
 */
/* pkinit_kdc_ocsp has been removed */
/* Make pkiDebug(fmt,...) print, or not.  */
/* Still evaluates for side effects.  */
/* This is better if the compiler doesn't inline variadic functions
   well, but gcc will warn about "left-hand operand of comma
   expression has no effect".  Still evaluates for side effects.  */
/* #define pkiDebug	(void) */
/* Solaris compiler doesn't grok __FUNCTION__
 * hack for now.  Fix all the uses eventually. */
/* Macros to deal with converting between various data types... */
/*
 * notes about crypto contexts:
 *
 * the basic idea is that there are crypto contexts that live at
 * both the plugin level and request level. the identity context (that
 * keeps info about your own certs and such) is separate because
 * it is needed at different levels for the kdc and and the client.
 * (the kdc's identity is at the plugin level, the client's identity
 * information could change per-request.)
 * the identity context is meant to have the entity's cert,
 * a list of trusted and intermediate cas, a list of crls, and any
 * pkcs11 information.  the req context is meant to have the
 * received certificate and the DH related information. the plugin
 * context is meant to have global crypto information, i.e., OIDs
 * and constant DH parameter information.
 */
/*
 * plugin crypto context should keep plugin common information,
 * eg., OIDs, known DHparams
 */
/*
 * request crypto context should keep reqyest common information,
 * eg., received credentials, DH parameters of this request
 */
/*
 * identity context should keep information about credentials
 * for the request, eg., my credentials, trusted ca certs,
 * intermediate ca certs, crls, pkcs11 info
 */
/*
 * this structure keeps information about the config options
 */
/* require EKU checking (default is true) */
/* accept secondary EKU (default is false) */
/* allow UPN-SAN instead of pkinit-SAN */
/* selects DH or RSA based pkinit */
/* require CRL for a CA (default is false) */
/* require freshness token (default is false) */
/* disable freshness token on client for testing */
/* minimum DH modulus size allowed */
/*
 * this structure keeps options used for a given request
 */
/* initial request DH modulus size (default=1024) */
/*
 * information about identity from config file or command line
 */
/*
 * Client's plugin context
 */
/*
 * Client's per-request context
 */
/*
 * KDC's (per-realm) plugin context
 */
/*
 * KDC's per-request context
 */
/*
 * Functions in pkinit_lib.c
 */
/*
 * Functions in pkinit_identity.c
 */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN/OUT */
/* IN/OUT */
/* IN/OUT */
/* IN (optional) */
/* IN */
/* IN */
/* IN */
/* IN */
/* IN/OUT */
/* IN/OUT */
/* IN/OUT */
/* IN */
/* IN (optional) */
/*
 * Client's list of identities for which it needs PINs or passwords
 */
/*
 * initialization and free functions
 */
/*
 * Functions in pkinit_profile.c
 */
/*
 * debugging functions
 */
#[no_mangle]
#[c2rust::src_loc = "297:1"]
pub unsafe extern "C" fn print_buffer_bin(mut buf: *mut libc::c_uchar,
                                          mut len: libc::c_uint,
                                          mut filename: *mut libc::c_char) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if len <= 0 as libc::c_int as libc::c_uint || filename.is_null() {
        return
    }
    f = fopen(filename, b"w\x00" as *const u8 as *const libc::c_char);
    if f.is_null() { return }
    fcntl(fileno(f), 2 as libc::c_int, 1 as libc::c_int);
    i = 0 as libc::c_int as libc::c_uint;
    while i < len {
        fputc(*buf.offset(i as isize) as libc::c_int, f);
        i = i.wrapping_add(1)
    }
    fclose(f);
}
