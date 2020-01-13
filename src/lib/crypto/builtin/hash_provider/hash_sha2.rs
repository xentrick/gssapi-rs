use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint32_t, __uint64_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:33"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "183:1"]
    pub type krb5_cryptotype = krb5_int32;
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
    #[c2rust::src_loc = "415:16"]
    pub struct _krb5_crypto_iov {
        pub flags: krb5_cryptotype,
        pub data: krb5_data,
    }
    /* *
 * Structure to describe a region of text to be encrypted or decrypted.
 *
 * The @a flags member describes the type of the iov.
 * The @a data member points to the memory that will be manipulated.
 * All iov APIs take a pointer to the first element of an array of krb5_crypto_iov's
 * along with the size of that array. Buffer contents are manipulated in-place;
 * data is overwritten. Callers must allocate the right number of krb5_crypto_iov
 * structures before calling into an iov API.
 */
    #[c2rust::src_loc = "415:1"]
    pub type krb5_crypto_iov = _krb5_crypto_iov;
    use super::stdint_intn_h::int32_t;
    /* *< @ref KRB5_CRYPTO_TYPE type of the iov */
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/krb/crypto_int.h:33"]
pub mod crypto_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:8"]
    pub struct krb5_hash_provider {
        pub hash_name: [libc::c_char; 8],
        pub hashsize: size_t,
        pub blocksize: size_t,
        pub hash: Option<unsafe extern "C" fn(_: *const krb5_crypto_iov,
                                              _: size_t, _: *mut krb5_data)
                             -> krb5_error_code>,
    }
    use super::stddef_h::size_t;
    use super::krb5_h::{krb5_error_code, krb5_crypto_iov, krb5_data};
    /* CRYPTO_INT_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/crypto/builtin/sha2/sha2.h:33"]
pub mod sha2_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/sha2/sha2.h - SHA-256 declarations */
/*
 * Copyright (c) 1995 - 2001 Kungliga Tekniska Högskolan
 * (Royal Institute of Technology, Stockholm, Sweden).
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the Institute nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE INSTITUTE AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE INSTITUTE OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
    /* SHA-384 and SHA-512 use the same state object. */
    #[c2rust::src_loc = "62:1"]
    pub type SHA256_CTX = sha256state;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct sha256state {
        pub sz: [libc::c_uint; 2],
        pub counter: [uint32_t; 8],
        pub save: [libc::c_uchar; 64],
    }
    #[c2rust::src_loc = "63:1"]
    pub type SHA384_CTX = sha512state;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:8"]
    pub struct sha512state {
        pub sz: [uint64_t; 2],
        pub counter: [uint64_t; 8],
        pub save: [libc::c_uchar; 128],
    }
    use super::stdint_uintn_h::{uint32_t, uint64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn k5_sha256_final(_: *mut libc::c_void, _: *mut SHA256_CTX);
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn k5_sha256_update(_: *mut SHA256_CTX, _: *const libc::c_void,
                                _: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn k5_sha256_init(_: *mut SHA256_CTX);
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn k5_sha384_final(_: *mut libc::c_void, _: *mut SHA384_CTX);
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn k5_sha384_update(_: *mut SHA384_CTX, _: *const libc::c_void,
                                _: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "70:1"]
        pub fn k5_sha384_init(_: *mut SHA384_CTX);
    }
    /* SHA2_H */
}
pub use self::types_h::{__int32_t, __uint32_t, __uint64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint32_t, uint64_t};
pub use self::krb5_h::{krb5_int32, krb5_cryptotype, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, _krb5_crypto_iov,
                       krb5_crypto_iov};
pub use self::crypto_int_h::krb5_hash_provider;
pub use self::sha2_h::{SHA256_CTX, sha256state, SHA384_CTX, sha512state,
                       k5_sha256_final, k5_sha256_update, k5_sha256_init,
                       k5_sha384_final, k5_sha384_update, k5_sha384_init};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/crypto/builtin/hash_provider/sha2.c - SHA-2 hash providers */
/*
 * Copyright (C) 2015 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn k5_sha256_hash(mut data: *const krb5_crypto_iov,
                                    mut num_data: size_t,
                                    mut output: *mut krb5_data)
 -> krb5_error_code {
    let mut ctx: SHA256_CTX =
        SHA256_CTX{sz: [0; 2], counter: [0; 8], save: [0; 64],};
    let mut i: size_t = 0;
    let mut iov: *const krb5_crypto_iov = 0 as *const krb5_crypto_iov;
    if (*output).length != 32 as libc::c_int as libc::c_uint {
        return -(1765328206 as libc::c_long) as krb5_error_code
    }
    k5_sha256_init(&mut ctx);
    i = 0 as libc::c_int as size_t;
    while i < num_data {
        iov = &*data.offset(i as isize) as *const krb5_crypto_iov;
        if (*iov).flags == 1 as libc::c_int ||
               ((*iov).flags == 2 as libc::c_int ||
                    (*iov).flags == 4 as libc::c_int) ||
               (*iov).flags == 3 as libc::c_int {
            k5_sha256_update(&mut ctx,
                             (*iov).data.data as *const libc::c_void,
                             (*iov).data.length as size_t);
        }
        i = i.wrapping_add(1)
    }
    k5_sha256_final((*output).data as *mut libc::c_void, &mut ctx);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn k5_sha384_hash(mut data: *const krb5_crypto_iov,
                                    mut num_data: size_t,
                                    mut output: *mut krb5_data)
 -> krb5_error_code {
    let mut ctx: SHA384_CTX =
        SHA384_CTX{sz: [0; 2], counter: [0; 8], save: [0; 128],};
    let mut i: size_t = 0;
    let mut iov: *const krb5_crypto_iov = 0 as *const krb5_crypto_iov;
    if (*output).length != 48 as libc::c_int as libc::c_uint {
        return -(1765328206 as libc::c_long) as krb5_error_code
    }
    k5_sha384_init(&mut ctx);
    i = 0 as libc::c_int as size_t;
    while i < num_data {
        iov = &*data.offset(i as isize) as *const krb5_crypto_iov;
        if (*iov).flags == 1 as libc::c_int ||
               ((*iov).flags == 2 as libc::c_int ||
                    (*iov).flags == 4 as libc::c_int) ||
               (*iov).flags == 3 as libc::c_int {
            k5_sha384_update(&mut ctx,
                             (*iov).data.data as *const libc::c_void,
                             (*iov).data.length as size_t);
        }
        i = i.wrapping_add(1)
    }
    k5_sha384_final((*output).data as *mut libc::c_void, &mut ctx);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "76:33"]
pub static mut krb5int_hash_sha256: krb5_hash_provider =
    unsafe {
        {
            let mut init =
                krb5_hash_provider{hash_name: [83, 72, 65, 45, 50, 53, 54, 0],
                                   hashsize: 32 as libc::c_int as size_t,
                                   blocksize: 64 as libc::c_int as size_t,
                                   hash:
                                       Some(k5_sha256_hash as
                                                unsafe extern "C" fn(_:
                                                                         *const krb5_crypto_iov,
                                                                     _:
                                                                         size_t,
                                                                     _:
                                                                         *mut krb5_data)
                                                    -> krb5_error_code),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "83:33"]
pub static mut krb5int_hash_sha384: krb5_hash_provider =
    unsafe {
        {
            let mut init =
                krb5_hash_provider{hash_name: [83, 72, 65, 45, 51, 56, 52, 0],
                                   hashsize: 48 as libc::c_int as size_t,
                                   blocksize: 128 as libc::c_int as size_t,
                                   hash:
                                       Some(k5_sha384_hash as
                                                unsafe extern "C" fn(_:
                                                                         *const krb5_crypto_iov,
                                                                     _:
                                                                         size_t,
                                                                     _:
                                                                         *mut krb5_data)
                                                    -> krb5_error_code),};
            init
        }
    };
