use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:33"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    use super::stdint_intn_h::int32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:33"]
pub mod k5_platform_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "888:1"]
        pub fn krb5int_strlcat(dst: *mut libc::c_char,
                               src: *const libc::c_char, siz: size_t)
         -> size_t;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "132:14"]
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:33"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
pub use self::types_h::__int32_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_principal_data, krb5_principal};
use self::stdlib_h::{malloc, free};
use self::k5_platform_h::krb5int_strlcat;
use self::string_h::{strlen, strncmp, strcmp, strncat, strncpy, memcpy};
use self::assert_h::__assert_fail;
/*
 * subrealm - determine if r2 is a subrealm of r1
 *
 *            SUBREALM takes two realms, r1 and r2, and
 *            determines if r2 is a subrealm of r1.
 *            r2 is a subrealm of r1 if (r1 is a prefix
 *            of r2 AND r1 and r2 begin with a /) or if
 *            (r1 is a suffix of r2 and neither r1 nor r2
 *            begin with a /).
 *
 * RETURNS:   If r2 is a subrealm, and r1 is a prefix, the number
 *            of characters in the suffix of r2 is returned as a
 *            negative number.
 *
 *            If r2 is a subrealm, and r1 is a suffix, the number
 *            of characters in the prefix of r2 is returned as a
 *            positive number.
 *
 *            If r2 is not a subrealm, SUBREALM returns 0.
 */
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn subrealm(mut r1: *mut libc::c_char,
                              mut r2: *mut libc::c_char) -> libc::c_int {
    let mut l1: size_t = 0;
    let mut l2: size_t = 0;
    l1 = strlen(r1);
    l2 = strlen(r2);
    if l2 <= l1 { return 0 as libc::c_int }
    if *r1 as libc::c_int == '/' as i32 && *r2 as libc::c_int == '/' as i32 &&
           strncmp(r1, r2, l1) == 0 as libc::c_int {
        return l1.wrapping_sub(l2) as libc::c_int
    }
    if *r1 as libc::c_int != '/' as i32 && *r2 as libc::c_int != '/' as i32 &&
           strncmp(r1, r2.offset(l2 as isize).offset(-(l1 as isize)), l1) ==
               0 as libc::c_int {
        return l2.wrapping_sub(l1) as libc::c_int
    }
    return 0 as libc::c_int;
}
/* authind.c */
/* cammac.c */
/* do_as_req.c */
/* do_tgs_req.c */
/* dispatch.c */
/* kdc_preauth.c */
/* kdc_preauth_ec.c */
/* kdc_preauth_enctsc.c */
/* kdc_authdata.c */
/* replay.c */
/* kdc_util.c */
/* FAST*/
/*
 * If *requestptr contains FX_FAST padata, compute the armor key, verify the
 * checksum over checksummed_data, decode the FAST request, and substitute
 * *requestptr with the inner request.  Set the armor_key, cookie, and
 * fast_options fields in state.  state->cookie will be set for a non-FAST
 * request if it contains FX_COOKIE padata.  If inner_body_out is non-NULL, set
 * *inner_body_out to a copy of the encoded inner body, or to NULL if the
 * request is not a FAST request.
 */
/* Information handle for kdcpreauth callbacks.  All pointers are aliases. */
/* RFC 4120: KRB5KDC_ERR_KEY_TOO_WEAK
 * RFC 4556: KRB5KDC_ERR_DH_KEY_PARAMETERS_NOT_ACCEPTED */
/* TGS-REQ options where the service can be a non-TGS principal  */
/* TGS-REQ options which are not compatible with referrals */
/*
 * Mask of KDC options that request the corresponding ticket flag with
 * the same number.  Some of these are invalid for AS-REQs, but
 * validate_as_request() takes care of that.  KDC_OPT_RENEWABLE isn't
 * here because it needs special handling in
 * kdc_get_ticket_renewtime().
 *
 * According to RFC 4120 section 3.1.3 the following AS-REQ options
 * request their corresponding ticket flags if local policy allows:
 *
 * KDC_OPT_FORWARDABLE  KDC_OPT_ALLOW_POSTDATE
 * KDC_OPT_POSTDATED    KDC_OPT_PROXIABLE
 * KDC_OPT_RENEWABLE
 *
 * RFC 1510 section A.6 shows pseudocode indicating that the following
 * TGS-REQ options request their corresponding ticket flags if local
 * policy allows:
 *
 * KDC_OPT_FORWARDABLE  KDC_OPT_FORWARDED
 * KDC_OPT_PROXIABLE    KDC_OPT_PROXY
 * KDC_OPT_POSTDATED    KDC_OPT_RENEWABLE
 *
 * The above list omits KDC_OPT_ALLOW_POSTDATE, but RFC 4120 section
 * 5.4.1 says the TGS also handles it.
 *
 * RFC 6112 makes KDC_OPT_REQUEST_ANONYMOUS the same bit number as
 * TKT_FLG_ANONYMOUS.
 */
/* Copy KDC options that request the corresponding ticket flags. */
/*
 * Mask of ticket flags for the TGS to propagate from a ticket to a
 * derivative ticket.
 *
 * RFC 4120 section 2.1 says the following flags are carried forward
 * from an initial ticket to derivative tickets:
 *
 * TKT_FLG_PRE_AUTH
 * TKT_FLG_HW_AUTH
 *
 * RFC 4120 section 2.6 says TKT_FLG_FORWARDED is carried forward to
 * derivative tickets.  Proxy tickets are basically identical to
 * forwarded tickets except that a TGT may never be proxied, therefore
 * tickets derived from proxy tickets should have TKT_FLAG_PROXY set.
 * RFC 4120 and RFC 1510 apparently have an accidental omission in not
 * requiring that tickets derived from a proxy ticket have
 * TKT_FLG_PROXY set.  Previous code also omitted this behavior.
 *
 * RFC 6112 section 4.2 implies that TKT_FLG_ANONYMOUS must be
 * propagated from an anonymous ticket to derivative tickets.
 */
/* Copy appropriate header ticket flags to new ticket. */
/*
 * add_to_transited  Adds the name of the realm which issued the
 *                   ticket granting ticket on which the new ticket to
 *                   be issued is based (note that this is the same as
 *                   the realm of the server listed in the ticket
 *                   granting ticket.
 *
 * ASSUMPTIONS:  This procedure assumes that the transited field from
 *               the existing ticket granting ticket already appears
 *               in compressed form.  It will add the new realm while
 *               maintaining that form.   As long as each successive
 *               realm is added using this (or a similar) routine, the
 *               transited field will be in compressed form.  The
 *               basis step is an empty transited field which is, by
 *               its nature, in its most compressed form.
 *
 * ARGUMENTS: krb5_data *tgt_trans  Transited field from TGT
 *            krb5_data *new_trans  The transited field for the new ticket
 *            krb5_principal tgs    Name of ticket granting server
 *                                  This includes the realm of the KDC
 *                                  that issued the ticket granting
 *                                  ticket.  This is the realm that is
 *                                  to be added to the transited field.
 *            krb5_principal client Name of the client
 *            krb5_principal server The name of the requested server.
 *                                  This may be the an intermediate
 *                                  ticket granting server.
 *
 *            The last two argument are needed since they are
 *            implicitly part of the transited field of the new ticket
 *            even though they are not explicitly listed.
 *
 * RETURNS:   krb5_error_code - Success, or out of memory
 *
 * MODIFIES:  new_trans:  ->length will contain the length of the new
 *                        transited field.
 *
 *                        If ->data was not null when this procedure
 *                        is called, the memory referenced by ->data
 *                        will be deallocated.
 *
 *                        Memory will be allocated for the new transited field
 *                        ->data will be updated to point to the newly
 *                        allocated memory.
 *
 * BUGS:  The space allocated for the new transited field is the
 *        maximum that might be needed given the old transited field,
 *        and the realm to be added.  This length is calculated
 *        assuming that no compression of the new realm is possible.
 *        This has no adverse consequences other than the allocation
 *        of more space than required.
 *
 *        This procedure will not yet use the null subfield notation,
 *        and it will get confused if it sees it.
 *
 *        This procedure does not check for quoted commas in realm
 *        names.
 */
#[no_mangle]
#[c2rust::src_loc = "130:1"]
pub unsafe extern "C" fn data2string(mut d: *mut krb5_data)
 -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s =
        malloc((*d).length.wrapping_add(1 as libc::c_int as libc::c_uint) as
                   libc::c_ulong) as *mut libc::c_char;
    if !s.is_null() {
        if (*d).length > 0 as libc::c_int as libc::c_uint {
            memcpy(s as *mut libc::c_void, (*d).data as *const libc::c_void,
                   (*d).length as libc::c_ulong);
        }
        *s.offset((*d).length as isize) = 0 as libc::c_int as libc::c_char
    }
    return s;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kdc/kdc_util.h */
/*
 * Portions Copyright (C) 2007 Apple Inc.
 * Copyright 1990, 2007, 2014 by the Massachusetts Institute of Technology.
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
 *
 * Declarations for policy.c
 */
#[no_mangle]
#[c2rust::src_loc = "143:1"]
pub unsafe extern "C" fn add_to_transited(mut tgt_trans: *mut krb5_data,
                                          mut new_trans: *mut krb5_data,
                                          mut tgs: krb5_principal,
                                          mut client: krb5_principal,
                                          mut server: krb5_principal)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut realm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut trans: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut otrans: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut otrans_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0;
    /* The following are for stepping through the transited field     */
    let mut prev: [libc::c_char; 500] =
        [0; 500]; /* Expanded current realm name     */
    let mut next: [libc::c_char; 500] =
        [0; 500]; /* count of last character in current and next */
    let mut current: [libc::c_char; 500] =
        [0; 500]; /* prefix length                               */
    let mut exp: [libc::c_char; 500] =
        [0; 500]; /* TRUE = new realm has been added             */
    let mut i: libc::c_int = 0;
    let mut clst: libc::c_int = 0;
    let mut nlst: libc::c_int = 0;
    let mut pl: libc::c_int = 0;
    let mut pl1: libc::c_int = 0;
    let mut added: libc::c_int = 0;
    realm = data2string(&mut (*tgs).realm);
    if realm.is_null() { return 12 as libc::c_int }
    otrans = data2string(tgt_trans);
    if otrans.is_null() {
        free(realm as *mut libc::c_void);
        return 12 as libc::c_int
    }
    /* Keep track of start so we can free */
    otrans_ptr = otrans;
    /* +1 for null,
       +1 for extra comma which may be added between
       +1 for potential space when leading slash in realm */
    bufsize =
        strlen(realm).wrapping_add(strlen(otrans)).wrapping_add(3 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong);
    if bufsize > 500 as libc::c_int as libc::c_ulong {
        bufsize = 500 as libc::c_int as size_t
    }
    trans = malloc(bufsize) as *mut libc::c_char;
    if trans.is_null() {
        retval = 12 as libc::c_int
    } else {
        if !(*new_trans).data.is_null() {
            free((*new_trans).data as *mut libc::c_void);
        }
        (*new_trans).data = trans;
        (*new_trans).length = 0 as libc::c_int as libc::c_uint;
        *trans.offset(0 as libc::c_int as isize) =
            '\u{0}' as i32 as libc::c_char;
        /* For the purpose of appending, the realm preceding the first */
    /* realm in the transited field is considered the null realm   */
        prev[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        /* read field into current */
        i = 0 as libc::c_int;
        loop  {
            if !(*otrans as libc::c_int != '\u{0}' as i32) {
                current_block = 8180496224585318153;
                break ;
            }
            if *otrans as libc::c_int == '\\' as i32 {
                otrans = otrans.offset(1);
                if *otrans as libc::c_int == '\u{0}' as i32 {
                    current_block = 8180496224585318153;
                    break ;
                }
            } else if *otrans as libc::c_int == ',' as i32 {
                otrans = otrans.offset(1);
                current_block = 8180496224585318153;
                break ;
            } else {
                let fresh0 = otrans;
                otrans = otrans.offset(1);
                let fresh1 = i;
                i = i + 1;
                current[fresh1 as usize] = *fresh0;
                if !(i >= 500 as libc::c_int) { continue ; }
                retval = -(1765328341 as libc::c_long) as krb5_error_code;
                current_block = 11057481214361780476;
                break ;
            }
        }
        match current_block {
            11057481214361780476 => { }
            _ => {
                current[i as usize] = '\u{0}' as i32 as libc::c_char;
                added =
                    ((*client).realm.length as libc::c_ulong == strlen(realm)
                         &&
                         strncmp((*client).realm.data, realm, strlen(realm))
                             == 0 ||
                         (*server).realm.length as libc::c_ulong ==
                             strlen(realm) &&
                             strncmp((*server).realm.data, realm,
                                     strlen(realm)) == 0) as libc::c_int;
                's_196:
                    loop  {
                        if !(current[0 as libc::c_int as usize] != 0) {
                            current_block = 12890877304563811856;
                            break ;
                        }
                        /* figure out expanded form of current name */
                        clst =
                            strlen(current.as_mut_ptr()).wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_ulong)
                                as libc::c_int;
                        if current[0 as libc::c_int as usize] as libc::c_int
                               == ' ' as i32 {
                            strncpy(exp.as_mut_ptr(),
                                    current.as_mut_ptr().offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                                    (::std::mem::size_of::<[libc::c_char; 500]>()
                                         as
                                         libc::c_ulong).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong));
                            exp[(::std::mem::size_of::<[libc::c_char; 500]>()
                                     as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as usize] = '\u{0}' as i32 as libc::c_char
                        } else if current[0 as libc::c_int as usize] as
                                      libc::c_int == '/' as i32 &&
                                      prev[0 as libc::c_int as usize] as
                                          libc::c_int == '/' as i32 {
                            strncpy(exp.as_mut_ptr(), prev.as_mut_ptr(),
                                    (::std::mem::size_of::<[libc::c_char; 500]>()
                                         as
                                         libc::c_ulong).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong));
                            exp[(::std::mem::size_of::<[libc::c_char; 500]>()
                                     as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            if strlen(exp.as_mut_ptr()).wrapping_add(strlen(current.as_mut_ptr())).wrapping_add(1
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_ulong)
                                   >= 500 as libc::c_int as libc::c_ulong {
                                retval =
                                    -(1765328341 as libc::c_long) as
                                        krb5_error_code;
                                current_block = 11057481214361780476;
                                break ;
                            } else {
                                strncat(exp.as_mut_ptr(),
                                        current.as_mut_ptr(),
                                        (::std::mem::size_of::<[libc::c_char; 500]>()
                                             as
                                             libc::c_ulong).wrapping_sub(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong).wrapping_sub(strlen(exp.as_mut_ptr())));
                            }
                        } else if current[clst as usize] as libc::c_int ==
                                      '.' as i32 {
                            strncpy(exp.as_mut_ptr(), current.as_mut_ptr(),
                                    (::std::mem::size_of::<[libc::c_char; 500]>()
                                         as
                                         libc::c_ulong).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong));
                            exp[(::std::mem::size_of::<[libc::c_char; 500]>()
                                     as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            if strlen(exp.as_mut_ptr()).wrapping_add(strlen(prev.as_mut_ptr())).wrapping_add(1
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                                   >= 500 as libc::c_int as libc::c_ulong {
                                retval =
                                    -(1765328341 as libc::c_long) as
                                        krb5_error_code;
                                current_block = 11057481214361780476;
                                break ;
                            } else {
                                strncat(exp.as_mut_ptr(), prev.as_mut_ptr(),
                                        (::std::mem::size_of::<[libc::c_char; 500]>()
                                             as
                                             libc::c_ulong).wrapping_sub(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong).wrapping_sub(strlen(exp.as_mut_ptr())));
                            }
                        } else {
                            strncpy(exp.as_mut_ptr(), current.as_mut_ptr(),
                                    (::std::mem::size_of::<[libc::c_char; 500]>()
                                         as
                                         libc::c_ulong).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong));
                            exp[(::std::mem::size_of::<[libc::c_char; 500]>()
                                     as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as usize] = '\u{0}' as i32 as libc::c_char
                        }
                        /* read field into next */
                        i = 0 as libc::c_int;
                        while *otrans as libc::c_int != '\u{0}' as i32 {
                            if *otrans as libc::c_int == '\\' as i32 {
                                otrans = otrans.offset(1);
                                if *otrans as libc::c_int == '\u{0}' as i32 {
                                    break ;
                                }
                            } else if *otrans as libc::c_int == ',' as i32 {
                                otrans = otrans.offset(1);
                                break ;
                            } else {
                                let fresh2 = otrans;
                                otrans = otrans.offset(1);
                                let fresh3 = i;
                                i = i + 1;
                                next[fresh3 as usize] = *fresh2;
                                if !(i >= 500 as libc::c_int) { continue ; }
                                retval =
                                    -(1765328341 as libc::c_long) as
                                        krb5_error_code;
                                current_block = 11057481214361780476;
                                break 's_196 ;
                            }
                        }
                        next[i as usize] = '\u{0}' as i32 as libc::c_char;
                        nlst = i - 1 as libc::c_int;
                        if strcmp(exp.as_mut_ptr(), realm) == 0 {
                            added = 1 as libc::c_int
                        }
                        /* If we still have to insert the new realm */
                        if added == 0 {
                            /* Is the next field compressed?  If not, and if the new */
            /* realm is a subrealm of the current realm, compress    */
            /* the new realm, and insert immediately following the   */
            /* current one.  Note that we can not do this if the next*/
            /* field is already compressed since it would mess up    */
            /* what has already been done.  In most cases, this is   */
            /* not a problem because the realm to be added will be a */
            /* subrealm of the next field too, and we will catch     */
            /* it in a future iteration.                             */
                            /* Note that the second test here is an unsigned comparison,
               so the first half (or a cast) is also required.  */
                            if nlst < 0 as libc::c_int ||
                                   nlst <
                                       ::std::mem::size_of::<[libc::c_char; 500]>()
                                           as libc::c_ulong as libc::c_int {
                            } else {
                                __assert_fail(b"nlst < 0 || nlst < (int)sizeof(next)\x00"
                                                  as *const u8 as
                                                  *const libc::c_char,
                                              b"kdc_transit.c\x00" as
                                                  *const u8 as
                                                  *const libc::c_char,
                                              296 as libc::c_int as
                                                  libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 107],
                                                                        &[libc::c_char; 107]>(b"krb5_error_code add_to_transited(krb5_data *, krb5_data *, krb5_principal, krb5_principal, krb5_principal)\x00")).as_ptr());
                            }
                            if (nlst < 0 as libc::c_int ||
                                    next[nlst as usize] as libc::c_int !=
                                        '.' as i32) &&
                                   next[0 as libc::c_int as usize] as
                                       libc::c_int != '/' as i32 &&
                                   {
                                       pl = subrealm(exp.as_mut_ptr(), realm);
                                       (pl) != 0
                                   } {
                                added = 1 as libc::c_int;
                                current[(::std::mem::size_of::<[libc::c_char; 500]>()
                                             as
                                             libc::c_ulong).wrapping_sub(1 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                                            as usize] =
                                    '\u{0}' as i32 as libc::c_char;
                                if strlen(current.as_mut_ptr()).wrapping_add((if pl
                                                                                     >
                                                                                     0
                                                                                         as
                                                                                         libc::c_int
                                                                                 {
                                                                                  pl
                                                                              } else {
                                                                                  -pl
                                                                              })
                                                                                 as
                                                                                 libc::c_ulong).wrapping_add(2
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_ulong)
                                       >= 500 as libc::c_int as libc::c_ulong
                                   {
                                    retval =
                                        -(1765328341 as libc::c_long) as
                                            krb5_error_code;
                                    current_block = 11057481214361780476;
                                    break ;
                                } else {
                                    strncat(current.as_mut_ptr(),
                                            b",\x00" as *const u8 as
                                                *const libc::c_char,
                                            (::std::mem::size_of::<[libc::c_char; 500]>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong).wrapping_sub(strlen(current.as_mut_ptr())));
                                    if pl > 0 as libc::c_int {
                                        strncat(current.as_mut_ptr(), realm,
                                                pl as libc::c_uint as
                                                    libc::c_ulong);
                                    } else {
                                        strncat(current.as_mut_ptr(),
                                                realm.offset(strlen(realm) as
                                                                 isize).offset(pl
                                                                                   as
                                                                                   isize),
                                                -pl as libc::c_uint as
                                                    libc::c_ulong);
                                    }
                                }
                            } else {
                                /* Whether or not the next field is compressed, if the    */
            /* realm to be added is a superrealm of the current realm,*/
            /* then the current realm can be compressed.  First the   */
            /* realm to be added must be compressed relative to the   */
            /* previous realm (if possible), and then the current     */
            /* realm compressed relative to the new realm.  Note that */
            /* if the realm to be added is also a superrealm of the   */
            /* previous realm, it would have been added earlier, and  */
            /* we would not reach this step this time around.         */
                                pl = subrealm(realm, exp.as_mut_ptr());
                                if pl != 0 {
                                    added = 1 as libc::c_int;
                                    current[0 as libc::c_int as usize] =
                                        '\u{0}' as i32 as libc::c_char;
                                    pl1 = subrealm(prev.as_mut_ptr(), realm);
                                    if pl1 != 0 {
                                        if strlen(current.as_mut_ptr()).wrapping_add((if pl1
                                                                                             >
                                                                                             0
                                                                                                 as
                                                                                                 libc::c_int
                                                                                         {
                                                                                          pl1
                                                                                      } else {
                                                                                          -pl1
                                                                                      })
                                                                                         as
                                                                                         libc::c_ulong).wrapping_add(1
                                                                                                                         as
                                                                                                                         libc::c_int
                                                                                                                         as
                                                                                                                         libc::c_ulong)
                                               >=
                                               500 as libc::c_int as
                                                   libc::c_ulong {
                                            retval =
                                                -(1765328341 as libc::c_long)
                                                    as krb5_error_code;
                                            current_block =
                                                11057481214361780476;
                                            break ;
                                        } else if pl1 > 0 as libc::c_int {
                                            strncat(current.as_mut_ptr(),
                                                    realm,
                                                    pl1 as libc::c_uint as
                                                        libc::c_ulong);
                                        } else {
                                            strncat(current.as_mut_ptr(),
                                                    realm.offset(strlen(realm)
                                                                     as
                                                                     isize).offset(pl1
                                                                                       as
                                                                                       isize),
                                                    -pl1 as libc::c_uint as
                                                        libc::c_ulong);
                                        }
                                    } else {
                                        /* If not a subrealm */
                                        if *realm.offset(0 as libc::c_int as
                                                             isize) as
                                               libc::c_int == '/' as i32 &&
                                               prev[0 as libc::c_int as usize]
                                                   as libc::c_int != 0 {
                                            if strlen(current.as_mut_ptr()).wrapping_add(2
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong)
                                                   >=
                                                   500 as libc::c_int as
                                                       libc::c_ulong {
                                                retval =
                                                    -(1765328341 as
                                                          libc::c_long) as
                                                        krb5_error_code;
                                                current_block =
                                                    11057481214361780476;
                                                break ;
                                            } else {
                                                strncat(current.as_mut_ptr(),
                                                        b" \x00" as *const u8
                                                            as
                                                            *const libc::c_char,
                                                        (::std::mem::size_of::<[libc::c_char; 500]>()
                                                             as
                                                             libc::c_ulong).wrapping_sub(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong).wrapping_sub(strlen(current.as_mut_ptr())));
                                                current[(::std::mem::size_of::<[libc::c_char; 500]>()
                                                             as
                                                             libc::c_ulong).wrapping_sub(1
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong)
                                                            as usize] =
                                                    '\u{0}' as i32 as
                                                        libc::c_char
                                            }
                                        }
                                        if strlen(current.as_mut_ptr()).wrapping_add(strlen(realm)).wrapping_add(1
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong)
                                               >=
                                               500 as libc::c_int as
                                                   libc::c_ulong {
                                            retval =
                                                -(1765328341 as libc::c_long)
                                                    as krb5_error_code;
                                            current_block =
                                                11057481214361780476;
                                            break ;
                                        } else {
                                            strncat(current.as_mut_ptr(),
                                                    realm,
                                                    (::std::mem::size_of::<[libc::c_char; 500]>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong).wrapping_sub(strlen(current.as_mut_ptr())));
                                            current[(::std::mem::size_of::<[libc::c_char; 500]>()
                                                         as
                                                         libc::c_ulong).wrapping_sub(1
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as usize] =
                                                '\u{0}' as i32 as libc::c_char
                                        }
                                    }
                                    if strlen(current.as_mut_ptr()).wrapping_add((if pl
                                                                                         >
                                                                                         0
                                                                                             as
                                                                                             libc::c_int
                                                                                     {
                                                                                      pl
                                                                                  } else {
                                                                                      -pl
                                                                                  })
                                                                                     as
                                                                                     libc::c_ulong).wrapping_add(2
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong)
                                           >=
                                           500 as libc::c_int as libc::c_ulong
                                       {
                                        retval =
                                            -(1765328341 as libc::c_long) as
                                                krb5_error_code;
                                        current_block = 11057481214361780476;
                                        break ;
                                    } else {
                                        strncat(current.as_mut_ptr(),
                                                b",\x00" as *const u8 as
                                                    *const libc::c_char,
                                                (::std::mem::size_of::<[libc::c_char; 500]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong).wrapping_sub(strlen(current.as_mut_ptr())));
                                        current[(::std::mem::size_of::<[libc::c_char; 500]>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)
                                                    as usize] =
                                            '\u{0}' as i32 as libc::c_char;
                                        if pl > 0 as libc::c_int {
                                            strncat(current.as_mut_ptr(),
                                                    exp.as_mut_ptr(),
                                                    pl as libc::c_uint as
                                                        libc::c_ulong);
                                        } else {
                                            strncat(current.as_mut_ptr(),
                                                    exp.as_mut_ptr().offset(strlen(exp.as_mut_ptr())
                                                                                as
                                                                                isize).offset(pl
                                                                                                  as
                                                                                                  isize),
                                                    -pl as libc::c_uint as
                                                        libc::c_ulong);
                                        }
                                    }
                                }
                            }
                        }
                        if (*new_trans).length !=
                               0 as libc::c_int as libc::c_uint {
                            if krb5int_strlcat(trans,
                                               b",\x00" as *const u8 as
                                                   *const libc::c_char,
                                               bufsize) >= bufsize {
                                retval =
                                    -(1765328341 as libc::c_long) as
                                        krb5_error_code;
                                current_block = 11057481214361780476;
                                break ;
                            }
                        }
                        if krb5int_strlcat(trans, current.as_mut_ptr(),
                                           bufsize) >= bufsize {
                            retval =
                                -(1765328341 as libc::c_long) as
                                    krb5_error_code;
                            current_block = 11057481214361780476;
                            break ;
                        } else {
                            (*new_trans).length =
                                strlen(trans) as libc::c_uint;
                            strncpy(prev.as_mut_ptr(), exp.as_mut_ptr(),
                                    (::std::mem::size_of::<[libc::c_char; 500]>()
                                         as
                                         libc::c_ulong).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong));
                            prev[(::std::mem::size_of::<[libc::c_char; 500]>()
                                      as
                                      libc::c_ulong).wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_ulong)
                                     as usize] =
                                '\u{0}' as i32 as libc::c_char;
                            strncpy(current.as_mut_ptr(), next.as_mut_ptr(),
                                    (::std::mem::size_of::<[libc::c_char; 500]>()
                                         as
                                         libc::c_ulong).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong));
                            current[(::std::mem::size_of::<[libc::c_char; 500]>()
                                         as
                                         libc::c_ulong).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as usize] =
                                '\u{0}' as i32 as libc::c_char
                        }
                    }
                match current_block {
                    11057481214361780476 => { }
                    _ => {
                        if added == 0 {
                            if (*new_trans).length !=
                                   0 as libc::c_int as libc::c_uint {
                                if krb5int_strlcat(trans,
                                                   b",\x00" as *const u8 as
                                                       *const libc::c_char,
                                                   bufsize) >= bufsize {
                                    retval =
                                        -(1765328341 as libc::c_long) as
                                            krb5_error_code;
                                    current_block = 11057481214361780476;
                                } else {
                                    current_block = 14714495436747744489;
                                }
                            } else { current_block = 14714495436747744489; }
                            match current_block {
                                11057481214361780476 => { }
                                _ => {
                                    if *realm.offset(0 as libc::c_int as
                                                         isize) as libc::c_int
                                           == '/' as i32 &&
                                           *trans.offset(0 as libc::c_int as
                                                             isize) as
                                               libc::c_int != 0 {
                                        if krb5int_strlcat(trans,
                                                           b" \x00" as
                                                               *const u8 as
                                                               *const libc::c_char,
                                                           bufsize) >= bufsize
                                           {
                                            retval =
                                                -(1765328341 as libc::c_long)
                                                    as krb5_error_code;
                                            current_block =
                                                11057481214361780476;
                                        } else {
                                            current_block =
                                                13174377073168946860;
                                        }
                                    } else {
                                        current_block = 13174377073168946860;
                                    }
                                    match current_block {
                                        11057481214361780476 => { }
                                        _ => {
                                            if krb5int_strlcat(trans, realm,
                                                               bufsize) >=
                                                   bufsize {
                                                retval =
                                                    -(1765328341 as
                                                          libc::c_long) as
                                                        krb5_error_code;
                                                current_block =
                                                    11057481214361780476;
                                            } else {
                                                (*new_trans).length =
                                                    strlen(trans) as
                                                        libc::c_uint;
                                                current_block =
                                                    5710330377809666066;
                                            }
                                        }
                                    }
                                }
                            }
                        } else { current_block = 5710330377809666066; }
                        match current_block {
                            11057481214361780476 => { }
                            _ => { retval = 0 as libc::c_int }
                        }
                    }
                }
            }
        }
    }
    free(realm as *mut libc::c_void);
    free(otrans_ptr as *mut libc::c_void);
    return retval;
}
