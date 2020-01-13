use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:33"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint64_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:33"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "715:12"]
    pub struct C2RustUnnamed {
        pub i: uint64_t,
    }
    #[inline]
    #[c2rust::src_loc = "710:1"]
    pub unsafe extern "C" fn load_64_le(mut cvp: *const libc::c_void)
     -> uint64_t {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return (*(p as *const C2RustUnnamed)).i;
    }
    use super::stdint_uintn_h::uint64_t;
    /* K5_PLATFORM_H */
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
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
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __uint64_t};
pub use self::stdint_uintn_h::{uint8_t, uint64_t};
pub use self::k5_platform_h::{C2RustUnnamed, load_64_le};
use self::stdlib_h::{malloc, calloc, free};
use self::string_h::{memcmp, memcpy};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "44:8"]
pub struct k5_hashtab {
    pub k0: uint64_t,
    pub k1: uint64_t,
    pub nbuckets: size_t,
    pub nentries: size_t,
    pub buckets: *mut bucket_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "49:19"]
pub struct bucket_list {
    pub slh_first: *mut entry,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/hash.c - hash table implementation */
/*
 * Copyright (C) 2018 by the Massachusetts Institute of Technology.
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "37:8"]
pub struct entry {
    pub key: *const libc::c_void,
    pub klen: size_t,
    pub val: *mut libc::c_void,
    pub next: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "41:5"]
pub struct C2RustUnnamed_0 {
    pub sle_next: *mut entry,
}
/* Return x rotated to the left by r bits. */
#[inline]
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn rotl64(mut x: uint64_t, mut r: libc::c_int) -> uint64_t {
    return x << r | x >> 64 as libc::c_int - r;
}
#[inline]
#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn sipround(mut v0: *mut uint64_t, mut v1: *mut uint64_t,
                              mut v2: *mut uint64_t, mut v3: *mut uint64_t) {
    *v0 = (*v0 as libc::c_ulong).wrapping_add(*v1) as uint64_t as uint64_t;
    *v2 = (*v2 as libc::c_ulong).wrapping_add(*v3) as uint64_t as uint64_t;
    *v1 = rotl64(*v1, 13 as libc::c_int) ^ *v0;
    *v3 = rotl64(*v3, 16 as libc::c_int) ^ *v2;
    *v0 = rotl64(*v0, 32 as libc::c_int);
    *v2 = (*v2 as libc::c_ulong).wrapping_add(*v1) as uint64_t as uint64_t;
    *v0 = (*v0 as libc::c_ulong).wrapping_add(*v3) as uint64_t as uint64_t;
    *v1 = rotl64(*v1, 17 as libc::c_int) ^ *v2;
    *v3 = rotl64(*v3, 21 as libc::c_int) ^ *v0;
    *v2 = rotl64(*v2, 32 as libc::c_int);
}
/* SipHash-2-4 from https://131002.net/siphash/siphash.pdf (Jean-Philippe
 * Aumasson and Daniel J. Bernstein) */
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn siphash24(mut data: *const uint8_t, mut len: size_t,
                               mut k0: uint64_t, mut k1: uint64_t)
 -> uint64_t {
    let mut v0: uint64_t =
        k0 ^ 0x736f6d6570736575 as libc::c_long as libc::c_ulong;
    let mut v1: uint64_t =
        k1 ^ 0x646f72616e646f6d as libc::c_long as libc::c_ulong;
    let mut v2: uint64_t =
        k0 ^ 0x6c7967656e657261 as libc::c_long as libc::c_ulong;
    let mut v3: uint64_t =
        k1 ^ 0x7465646279746573 as libc::c_long as libc::c_ulong;
    let mut mi: uint64_t = 0;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut end: *const uint8_t =
        data.offset(len.wrapping_sub(len.wrapping_rem(8 as libc::c_int as
                                                          libc::c_ulong)) as
                        isize);
    let mut last: [uint8_t; 8] =
        [0 as libc::c_int as uint8_t, 0, 0, 0, 0, 0, 0, 0];
    /* Process each full 8-byte chunk of data. */
    p = data;
    while p < end {
        mi = load_64_le(p as *const libc::c_void);
        v3 ^= mi;
        sipround(&mut v0, &mut v1, &mut v2, &mut v3);
        sipround(&mut v0, &mut v1, &mut v2, &mut v3);
        v0 ^= mi;
        p = p.offset(8 as libc::c_int as isize)
    }
    /* Process the last 0-7 bytes followed by the length mod 256. */
    memcpy(last.as_mut_ptr() as *mut libc::c_void, end as *const libc::c_void,
           len.wrapping_rem(8 as libc::c_int as libc::c_ulong));
    last[7 as libc::c_int as usize] =
        (len & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    mi = load_64_le(last.as_mut_ptr() as *const libc::c_void);
    v3 ^= mi;
    sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    v0 ^= mi;
    /* Finalize. */
    v2 ^= 0xff as libc::c_int as libc::c_ulong;
    sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    sipround(&mut v0, &mut v1, &mut v2, &mut v3);
    return v0 ^ v1 ^ v2 ^ v3;
}
#[no_mangle]
#[c2rust::src_loc = "114:1"]
pub unsafe extern "C" fn k5_siphash24(mut data: *const uint8_t,
                                      mut len: size_t,
                                      mut seed: *const uint8_t) -> uint64_t {
    let mut k0: uint64_t = load_64_le(seed as *const libc::c_void);
    let mut k1: uint64_t =
        load_64_le(seed.offset(8 as libc::c_int as isize) as
                       *const libc::c_void);
    return siphash24(data, len, k0, k1);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-hash.h - hash table interface definitions */
/*
 * Copyright (C) 2018 by the Massachusetts Institute of Technology.
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
/*
 * This file contains declarations for a simple hash table using siphash.  Some
 * limitations which might need to be addressed in the future:
 *
 * - The table does not manage caller memory.  This limitation could be
 *   addressed by adding an optional free callback to k5_hashtab_create(), to
 *   be called by k5_hashtab_free() and k5_hashtab_remove().
 *
 * - There is no way to iterate over a hash table.
 *
 * - k5_hashtab_add() does not check for duplicate entries.
 */
/*
 * Create a new hash table in *ht_out.  seed must point to random bytes if keys
 * might be under the control of an attacker; otherwise it may be NULL.
 * initial_buckets controls the initial allocation of hash buckets; pass zero
 * to use a default value.  The number of hash buckets will be doubled as the
 * number of entries increases.  Return 0 on success, ENOMEM on failure.
 */
#[no_mangle]
#[c2rust::src_loc = "123:1"]
pub unsafe extern "C" fn k5_hashtab_create(mut seed: *const uint8_t,
                                           mut initial_buckets: size_t,
                                           mut ht_out: *mut *mut k5_hashtab)
 -> libc::c_int {
    let mut ht: *mut k5_hashtab = 0 as *mut k5_hashtab;
    *ht_out = 0 as *mut k5_hashtab;
    ht =
        malloc(::std::mem::size_of::<k5_hashtab>() as libc::c_ulong) as
            *mut k5_hashtab;
    if ht.is_null() { return 12 as libc::c_int }
    if !seed.is_null() {
        (*ht).k0 = load_64_le(seed as *const libc::c_void);
        (*ht).k1 =
            load_64_le(seed.offset(8 as libc::c_int as isize) as
                           *const libc::c_void)
    } else { (*ht).k1 = 0 as libc::c_int as uint64_t; (*ht).k0 = (*ht).k1 }
    (*ht).nbuckets =
        if initial_buckets > 0 as libc::c_int as libc::c_ulong {
            initial_buckets
        } else { 64 as libc::c_int as libc::c_ulong };
    (*ht).nentries = 0 as libc::c_int as size_t;
    (*ht).buckets =
        calloc((*ht).nbuckets,
               ::std::mem::size_of::<bucket_list>() as libc::c_ulong) as
            *mut bucket_list;
    if (*ht).buckets.is_null() {
        free(ht as *mut libc::c_void);
        return 12 as libc::c_int
    }
    *ht_out = ht;
    return 0 as libc::c_int;
}
/* Release the memory used by a hash table.  Keys and values are the caller's
 * responsibility. */
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub unsafe extern "C" fn k5_hashtab_free(mut ht: *mut k5_hashtab) {
    let mut i: size_t = 0;
    let mut ent: *mut entry = 0 as *mut entry;
    i = 0 as libc::c_int as size_t;
    while i < (*ht).nbuckets {
        while !(*(*ht).buckets.offset(i as isize)).slh_first.is_null() {
            ent = (*(*ht).buckets.offset(i as isize)).slh_first;
            let ref mut fresh0 =
                (*(*ht).buckets.offset(i as isize)).slh_first;
            *fresh0 =
                (*(*(*ht).buckets.offset(i as
                                             isize)).slh_first).next.sle_next;
            free(ent as *mut libc::c_void);
        }
        i = i.wrapping_add(1)
    }
    free((*ht).buckets as *mut libc::c_void);
    free(ht as *mut libc::c_void);
}
#[c2rust::src_loc = "170:1"]
unsafe extern "C" fn resize_table(mut ht: *mut k5_hashtab) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut newsize: size_t =
        (*ht).nbuckets.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    let mut newbuckets: *mut bucket_list = 0 as *mut bucket_list;
    let mut ent: *mut entry = 0 as *mut entry;
    newbuckets =
        calloc(newsize, ::std::mem::size_of::<bucket_list>() as libc::c_ulong)
            as *mut bucket_list;
    if newbuckets.is_null() { return 12 as libc::c_int }
    /* Rehash all the entries into the new buckets. */
    i = 0 as libc::c_int as size_t;
    while i < (*ht).nbuckets {
        while !(*(*ht).buckets.offset(i as isize)).slh_first.is_null() {
            ent = (*(*ht).buckets.offset(i as isize)).slh_first;
            j =
                siphash24((*ent).key as *const uint8_t, (*ent).klen, (*ht).k0,
                          (*ht).k1).wrapping_rem(newsize);
            let ref mut fresh1 =
                (*(*ht).buckets.offset(i as isize)).slh_first;
            *fresh1 =
                (*(*(*ht).buckets.offset(i as
                                             isize)).slh_first).next.sle_next;
            (*ent).next.sle_next = (*newbuckets.offset(j as isize)).slh_first;
            let ref mut fresh2 = (*newbuckets.offset(j as isize)).slh_first;
            *fresh2 = ent
        }
        i = i.wrapping_add(1)
    }
    free((*ht).buckets as *mut libc::c_void);
    (*ht).buckets = newbuckets;
    (*ht).nbuckets = newsize;
    return 0 as libc::c_int;
}
/* Add an entry to a hash table.  key and val must remain valid until the entry
 * is removed or the hash table is freed.  The caller must avoid duplicates. */
#[no_mangle]
#[c2rust::src_loc = "197:1"]
pub unsafe extern "C" fn k5_hashtab_add(mut ht: *mut k5_hashtab,
                                        mut key: *const libc::c_void,
                                        mut klen: size_t,
                                        mut val: *mut libc::c_void)
 -> libc::c_int {
    let mut i: size_t = 0;
    let mut ent: *mut entry = 0 as *mut entry;
    if (*ht).nentries == (*ht).nbuckets {
        if resize_table(ht) != 0 as libc::c_int { return 12 as libc::c_int }
    }
    ent =
        malloc(::std::mem::size_of::<entry>() as libc::c_ulong) as *mut entry;
    if ent.is_null() { return 12 as libc::c_int }
    (*ent).key = key;
    (*ent).klen = klen;
    (*ent).val = val;
    i =
        siphash24(key as *const uint8_t, klen, (*ht).k0,
                  (*ht).k1).wrapping_rem((*ht).nbuckets);
    (*ent).next.sle_next = (*(*ht).buckets.offset(i as isize)).slh_first;
    let ref mut fresh3 = (*(*ht).buckets.offset(i as isize)).slh_first;
    *fresh3 = ent;
    (*ht).nentries = (*ht).nentries.wrapping_add(1);
    return 0 as libc::c_int;
}
/* Remove an entry from a hash table by key.  Does not free key or the
 * associated value.  Return 1 if the key was found and removed, 0 if not. */
#[no_mangle]
#[c2rust::src_loc = "222:1"]
pub unsafe extern "C" fn k5_hashtab_remove(mut ht: *mut k5_hashtab,
                                           mut key: *const libc::c_void,
                                           mut klen: size_t) -> libc::c_int {
    let mut i: size_t = 0;
    let mut ent: *mut entry = 0 as *mut entry;
    i =
        siphash24(key as *const uint8_t, klen, (*ht).k0,
                  (*ht).k1).wrapping_rem((*ht).nbuckets);
    ent = (*(*ht).buckets.offset(i as isize)).slh_first;
    while !ent.is_null() {
        if (*ent).klen == klen &&
               memcmp((*ent).key, key, klen) == 0 as libc::c_int {
            if (*(*ht).buckets.offset(i as isize)).slh_first == ent {
                let ref mut fresh4 =
                    (*(*ht).buckets.offset(i as isize)).slh_first;
                *fresh4 =
                    (*(*(*ht).buckets.offset(i as
                                                 isize)).slh_first).next.sle_next
            } else {
                let mut curelm: *mut entry =
                    (*(*ht).buckets.offset(i as isize)).slh_first;
                while (*curelm).next.sle_next != ent {
                    curelm = (*curelm).next.sle_next
                }
                (*curelm).next.sle_next =
                    (*(*curelm).next.sle_next).next.sle_next
            }
            free(ent as *mut libc::c_void);
            (*ht).nentries = (*ht).nentries.wrapping_sub(1);
            return 1 as libc::c_int
        }
        ent = (*ent).next.sle_next
    }
    return 0 as libc::c_int;
}
/* Retrieve a value from a hash table by key. */
#[no_mangle]
#[c2rust::src_loc = "240:1"]
pub unsafe extern "C" fn k5_hashtab_get(mut ht: *mut k5_hashtab,
                                        mut key: *const libc::c_void,
                                        mut klen: size_t)
 -> *mut libc::c_void {
    let mut i: size_t = 0;
    let mut ent: *mut entry = 0 as *mut entry;
    i =
        siphash24(key as *const uint8_t, klen, (*ht).k0,
                  (*ht).k1).wrapping_rem((*ht).nbuckets);
    ent = (*(*ht).buckets.offset(i as isize)).slh_first;
    while !ent.is_null() {
        if (*ent).klen == klen &&
               memcmp((*ent).key, key, klen) == 0 as libc::c_int {
            return (*ent).val
        }
        ent = (*ent).next.sle_next
    }
    return 0 as *mut libc::c_void;
}
