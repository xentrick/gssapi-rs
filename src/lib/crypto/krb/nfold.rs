use ::libc;
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
use self::string_h::memset;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/krb/crypto_int.h - Master libk5crypto internal header */
/*
 * Copyright (C) 2011 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
/* This header is the entry point for libk5crypto sources, and also documents
 * requirements for crypto modules and PRNG modules.  */
/* Enc providers and hash providers specify well-known ciphers and hashes to be
 * implemented by the crypto module. */
/* keybytes is the input size to make_key;
       keylength is the output size */
/* May be NULL if the cipher is not used for a cbc-mac checksum. */
/* May be NULL if there is no key-derived data cached.  */
/* ** RFC 3961 enctypes table ***/
/*
 * "Weak" means the enctype is believed to be vulnerable to practical attacks,
 * and will be disabled unless allow_weak_crypto is set to true.  "Deprecated"
 * means the enctype has been deprecated by the IETF, and affects display and
 * logging.
 */
/* ** RFC 3961 checksum types table ***/
/*
 * Compute a checksum over the header, data, padding, and sign-only fields of
 * the iov array data (of size num_data).  The output buffer will already be
 * allocated with ctp->compute_size bytes available; the handler just needs to
 * fill in the contents.  If ctp->enc is not NULL, the handler can assume that
 * key is a valid-length key of an enctype which uses that enc provider.
 */
/*
 * Verify a checksum over the header, data, padding, and sign-only fields of
 * the iov array data (of size num_data), and store the boolean result in
 * *valid.  The handler can assume that hash has length ctp->output_size.  If
 * ctp->enc is not NULL, the handler can assume that key a valid-length key of
 * an enctype which uses that enc provider.
 */
/* NULL means recompute checksum and compare */
/* Allocation size for checksum computation */
/* Possibly truncated output size */
/* ** Prototypes for enctype table functions ***/
/* Length */
/* Encrypt */
/* Decrypt */
/* String to key */
/* Random to key */
/* Pseudo-random function */
/* ** Prototypes for cksumtype handler functions ***/
/* ** Key derivation functions ***/
/* RFC 3961 section 5.1 */
/* NIST SP 800-108 with CMAC as PRF */
/* NIST SP 800-108 with HMAC as PRF */
/* ** Miscellaneous prototypes ***/
/* nfold algorithm from RFC 3961 */
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
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
 * n-fold(k-bits):
 * l = lcm(n,k)
 * r = l/k
 * s = k-bits | k-bits rot 13 | k-bits rot 13*2 | ... | k-bits rot 13*(r-1)
 * compute the 1's complement sum:
 * n-fold = s[0..n-1]+s[n..2n-1]+s[2n..3n-1]+..+s[(k-1)*n..k*n-1]
 */
/* representation: msb first, assume n and k are multiples of 8, and
 * that k>=16.  this is the case of all the cryptosystems which are
 * likely to be used.  this function can be replaced if that
 * assumption ever fails.  */
/* input length is in bits */
#[no_mangle]
#[c2rust::src_loc = "46:1"]
pub unsafe extern "C" fn krb5int_nfold(mut inbits: libc::c_uint,
                                       mut in_0: *const libc::c_uchar,
                                       mut outbits: libc::c_uint,
                                       mut out: *mut libc::c_uchar) {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut lcm: libc::c_int = 0;
    let mut byte: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut msbit: libc::c_int = 0;
    /* the code below is more readable if I make these bytes
       instead of bits */
    inbits >>= 3 as libc::c_int;
    outbits >>= 3 as libc::c_int;
    /* first compute lcm(n,k) */
    a = outbits as libc::c_int;
    b = inbits as libc::c_int;
    while b != 0 as libc::c_int { c = b; b = a % b; a = c }
    lcm =
        outbits.wrapping_mul(inbits).wrapping_div(a as libc::c_uint) as
            libc::c_int;
    /* now do the real work */
    memset(out as *mut libc::c_void, 0 as libc::c_int,
           outbits as libc::c_ulong);
    byte = 0 as libc::c_int;
    /* this will end up cycling through k lcm(k,n)/k times, which
       is correct */
    i = lcm - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        /* compute the msbit in k which gets added into this byte */
        msbit =
            (inbits <<
                 3 as
                     libc::c_int).wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint).wrapping_add((inbits
                                                                                   <<
                                                                                   3
                                                                                       as
                                                                                       libc::c_int).wrapping_add(13
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_uint).wrapping_mul((i
                                                                                                                                                     as
                                                                                                                                                     libc::c_uint).wrapping_div(inbits))).wrapping_add(inbits.wrapping_sub((i
                                                                                                                                                                                                                                as
                                                                                                                                                                                                                                libc::c_uint).wrapping_rem(inbits))
                                                                                                                                                                                                           <<
                                                                                                                                                                                                           3
                                                                                                                                                                                                               as
                                                                                                                                                                                                               libc::c_int).wrapping_rem(inbits
                                                                                                                                                                                                                                             <<
                                                                                                                                                                                                                                             3
                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                 libc::c_int)
                as libc::c_int;
        /* pull out the byte value itself */
        byte +=
            ((*in_0.offset(inbits.wrapping_sub(1 as libc::c_int as
                                                   libc::c_uint).wrapping_sub((msbit
                                                                                   >>
                                                                                   3
                                                                                       as
                                                                                       libc::c_int)
                                                                                  as
                                                                                  libc::c_uint).wrapping_rem(inbits)
                               as isize) as libc::c_int) << 8 as libc::c_int |
                 *in_0.offset(inbits.wrapping_sub((msbit >> 3 as libc::c_int)
                                                      as
                                                      libc::c_uint).wrapping_rem(inbits)
                                  as isize) as libc::c_int) >>
                (msbit & 7 as libc::c_int) + 1 as libc::c_int &
                0xff as libc::c_int;
        /* do the addition */
        byte +=
            *out.offset((i as libc::c_uint).wrapping_rem(outbits) as isize) as
                libc::c_int;
        *out.offset((i as libc::c_uint).wrapping_rem(outbits) as isize) =
            (byte & 0xff as libc::c_int) as libc::c_uchar;
        /* keep around the carry bit, if any */
        byte >>= 8 as libc::c_int;
        i -= 1
    }
    /* if there's a carry bit left over, add it back in */
    if byte != 0 {
        i =
            outbits.wrapping_sub(1 as libc::c_int as libc::c_uint) as
                libc::c_int;
        while i >= 0 as libc::c_int {
            /* do the addition */
            byte += *out.offset(i as isize) as libc::c_int;
            *out.offset(i as isize) =
                (byte & 0xff as libc::c_int) as libc::c_uchar;
            /* keep around the carry bit, if any */
            byte >>= 8 as libc::c_int;
            i -= 1
        }
    };
}
