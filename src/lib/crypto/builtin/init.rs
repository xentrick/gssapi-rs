use ::libc;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/init.c - Module init and cleanup functions */
/*
 * Copyright (C) 2010 by the Massachusetts Institute of Technology.
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
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn krb5int_crypto_impl_init() -> libc::c_int {
    return 0 as libc::c_int;
}
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
/* Compute a CMAC checksum over data. */
/* Translate an RFC 3961 key usage to a Microsoft RC4 usage. */
/* Ensure library initialization has occurred. */
/* DES default state initialization handler (used by module enc providers). */
/* Default state cleanup handler (used by module enc providers). */
/* ** Input/output vector processing declarations **/
/* iov array we are iterating over */
/* size of iov array */
/* size of blocks we will be obtaining */
/* should we process SIGN_ONLY blocks */
/* read index into iov array */
/* read index into iov contents */
/* write index into iov array */
/* write index into iov contents */
/* ** Crypto module declarations ***/
/* Modules must implement the k5_sha256() function prototyped in k5-int.h. */
/* Modules must implement the following enc_providers and hash_providers: */
/* Modules must implement the following functions. */
/* Set the parity bits to the correct values in keybits. */
/* Return true if keybits is a weak or semi-weak DES key. */
/* Compute an HMAC using the provided hash function, key, and data, storing the
 * result into output (caller-allocated). */
/* As above, using a keyblock as the key input. */
/*
 * Compute the PBKDF2 (see RFC 2898) of password and salt, with the specified
 * count, using HMAC with the specified hash as the pseudo-random function,
 * storing the result into out (caller-allocated).
 */
/* The following are used by test programs and are just handler functions from
 * the AES and Camellia enc providers. */
/* These can be used to safely set up and tear down module global state. */
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn krb5int_crypto_impl_cleanup() { }
