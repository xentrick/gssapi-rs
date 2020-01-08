pub type krb5_auth_pack = crate::k5_int_pkinit_h::_krb5_auth_pack;
pub type krb5_kdc_dh_key_info = crate::k5_int_pkinit_h::_krb5_kdc_dh_key_info;
pub type krb5_pa_pk_as_rep = crate::k5_int_pkinit_h::_krb5_pa_pk_as_rep;
pub type krb5_pa_pk_as_req = crate::k5_int_pkinit_h::_krb5_pa_pk_as_req;
pub type krb5_reply_key_pack = crate::k5_int_pkinit_h::_krb5_reply_key_pack;
pub type krb5_algorithm_identifier = crate::k5_int_pkinit_h::_krb5_algorithm_identifier;
pub type krb5_external_principal_identifier =
    crate::k5_int_pkinit_h::_krb5_external_principal_identifier;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_pk_authenticator {
    pub cusec: crate::krb5_h::krb5_int32,
    pub ctime: crate::krb5_h::krb5_timestamp,
    pub nonce: crate::krb5_h::krb5_int32,
    pub paChecksum: crate::krb5_h::krb5_checksum,
    pub freshnessToken: *mut crate::krb5_h::krb5_data,
}
pub type krb5_pk_authenticator = crate::k5_int_pkinit_h::_krb5_pk_authenticator;
/* AlgorithmIdentifier */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_algorithm_identifier {
    pub algorithm: crate::krb5_h::krb5_data,
    pub parameters: crate::krb5_h::krb5_data,
}
/* Optional */

/* SubjectPublicKeyInfo */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_subject_pk_info {
    pub algorithm: crate::k5_int_pkinit_h::krb5_algorithm_identifier,
    pub subjectPublicKey: crate::krb5_h::krb5_data,
}
pub type krb5_subject_pk_info = crate::k5_int_pkinit_h::_krb5_subject_pk_info;
/* BIT STRING */

/* * AuthPack from RFC 4556*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_auth_pack {
    pub pkAuthenticator: crate::k5_int_pkinit_h::krb5_pk_authenticator,
    pub clientPublicValue: *mut crate::k5_int_pkinit_h::krb5_subject_pk_info,
    pub supportedCMSTypes: *mut *mut crate::k5_int_pkinit_h::krb5_algorithm_identifier,
    pub clientDHNonce: crate::krb5_h::krb5_data,
    pub supportedKDFs: *mut *mut crate::krb5_h::krb5_data,
}
/* OIDs of KDFs; OPTIONAL */

/* ExternalPrincipalIdentifier */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_external_principal_identifier {
    pub subjectName: crate::krb5_h::krb5_data,
    pub issuerAndSerialNumber: crate::krb5_h::krb5_data,
    pub subjectKeyIdentifier: crate::krb5_h::krb5_data,
}
/* Optional */

/* PA-PK-AS-REQ (rfc4556 -- PA TYPE 16) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_pa_pk_as_req {
    pub signedAuthPack: crate::krb5_h::krb5_data,
    pub trustedCertifiers: *mut *mut crate::k5_int_pkinit_h::krb5_external_principal_identifier,
    pub kdcPkId: crate::krb5_h::krb5_data,
}
/* Optional */

/* * Pkinit DHRepInfo */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_dh_rep_info {
    pub dhSignedData: crate::krb5_h::krb5_data,
    pub serverDHNonce: crate::krb5_h::krb5_data,
    pub kdfID: *mut crate::krb5_h::krb5_data,
}
pub type krb5_dh_rep_info = crate::k5_int_pkinit_h::_krb5_dh_rep_info;
/* OID of selected KDF OPTIONAL */

/* KDCDHKeyInfo */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_kdc_dh_key_info {
    pub subjectPublicKey: crate::krb5_h::krb5_data,
    pub nonce: crate::krb5_h::krb5_int32,
    pub dhKeyExpiration: crate::krb5_h::krb5_timestamp,
}
/* Optional */

/* ReplyKeyPack */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_reply_key_pack {
    pub replyKey: crate::krb5_h::krb5_keyblock,
    pub asChecksum: crate::krb5_h::krb5_checksum,
}
/* PA-PK-AS-REP (rfc4556 -- PA TYPE 17) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _krb5_pa_pk_as_rep {
    pub choice: crate::k5_int_pkinit_h::krb5_pa_pk_as_rep_selection,
    pub u: crate::k5_int_pkinit_h::krb5_pa_pk_as_rep_choices,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union krb5_pa_pk_as_rep_choices {
    pub dh_Info: crate::k5_int_pkinit_h::krb5_dh_rep_info,
    pub encKeyPack: crate::krb5_h::krb5_data,
}
pub type krb5_pa_pk_as_rep_selection = i32;
pub const choice_pa_pk_as_rep_encKeyPack: crate::k5_int_pkinit_h::krb5_pa_pk_as_rep_selection = 1;
pub const choice_pa_pk_as_rep_dhInfo: crate::k5_int_pkinit_h::krb5_pa_pk_as_rep_selection = 0;
pub const choice_pa_pk_as_rep_UNKNOWN: crate::k5_int_pkinit_h::krb5_pa_pk_as_rep_selection = -1;
