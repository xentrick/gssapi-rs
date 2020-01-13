use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:41"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:41"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/sys/types.h:41"]
pub mod sys_types_h {
    #[c2rust::src_loc = "158:1"]
    pub type u_int8_t = __uint8_t;
    #[c2rust::src_loc = "160:1"]
    pub type u_int32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
pub use self::types_h::{__uint8_t, __uint32_t};
pub use self::stddef_h::size_t;
pub use self::sys_types_h::{u_int8_t, u_int32_t};
/* Default hash function. */
#[no_mangle]
#[c2rust::src_loc = "56:13"]
pub static mut __default_hash:
           Option<unsafe extern "C" fn(_: *const libc::c_void, _: size_t)
                      -> u_int32_t> =
    unsafe {
        Some(hash4 as
                 unsafe extern "C" fn(_: *const libc::c_void, _: size_t)
                     -> u_int32_t)
    };
/*-
 * Copyright (c) 1990, 1993
 *	The Regents of the University of California.  All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Margo Seltzer.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. All advertising materials mentioning features or use of this software
 *    must display the following acknowledgement:
 *	This product includes software developed by the University of
 *	California, Berkeley and its contributors.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/* LIBC_SCCS and not lint */
/*
 * Assume that we've already split the bucket to which this key hashes,
 * calculate that bucket, and check that in fact we did already split it.
 *
 * EJB's original hsearch hash.
 */
/* Chris Torek's hash function. */
#[c2rust::src_loc = "161:1"]
unsafe extern "C" fn hash4(mut key: *const libc::c_void, mut len: size_t)
 -> u_int32_t {
    let mut h: u_int32_t = 0;
    let mut loop_0: u_int32_t = 0;
    let mut k: *const u_int8_t = 0 as *const u_int8_t;
    h = 0 as libc::c_int as u_int32_t;
    k = key as *const u_int8_t;
    if len > 0 as libc::c_int as libc::c_ulong {
        loop_0 =
            (len.wrapping_add(8 as libc::c_int as
                                  libc::c_ulong).wrapping_sub(1 as libc::c_int
                                                                  as
                                                                  libc::c_ulong)
                 >> 3 as libc::c_int) as u_int32_t;
        's_119:
            {
                let mut current_block_18: u64;
                match len &
                          (8 as libc::c_int - 1 as libc::c_int) as
                              libc::c_ulong {
                    0 => { current_block_18 = 11875828834189669668; }
                    7 => { current_block_18 = 1119373698233436522; }
                    6 => { current_block_18 = 7917834318927108011; }
                    5 => { current_block_18 = 1907019040617419533; }
                    4 => { current_block_18 = 13115551051516724588; }
                    3 => { current_block_18 = 4404584186068957266; }
                    2 => { current_block_18 = 775645747195235551; }
                    1 => { current_block_18 = 6409029412943682534; }
                    _ => { current_block_18 = 11307063007268554308; }
                }
                loop  {
                    match current_block_18 {
                        11307063007268554308 => { break 's_119 ; }
                        11875828834189669668 => {
                            /* All fall throughs */
                            let fresh0 = k;
                            k = k.offset(1);
                            h =
                                (h <<
                                     5 as
                                         libc::c_int).wrapping_add(h).wrapping_add(*fresh0
                                                                                       as
                                                                                       libc::c_uint);
                            current_block_18 = 1119373698233436522;
                        }
                        1119373698233436522 => {
                            let fresh1 = k;
                            k = k.offset(1);
                            h =
                                (h <<
                                     5 as
                                         libc::c_int).wrapping_add(h).wrapping_add(*fresh1
                                                                                       as
                                                                                       libc::c_uint);
                            current_block_18 = 7917834318927108011;
                        }
                        7917834318927108011 => {
                            let fresh2 = k;
                            k = k.offset(1);
                            h =
                                (h <<
                                     5 as
                                         libc::c_int).wrapping_add(h).wrapping_add(*fresh2
                                                                                       as
                                                                                       libc::c_uint);
                            current_block_18 = 1907019040617419533;
                        }
                        1907019040617419533 => {
                            let fresh3 = k;
                            k = k.offset(1);
                            h =
                                (h <<
                                     5 as
                                         libc::c_int).wrapping_add(h).wrapping_add(*fresh3
                                                                                       as
                                                                                       libc::c_uint);
                            current_block_18 = 13115551051516724588;
                        }
                        13115551051516724588 => {
                            let fresh4 = k;
                            k = k.offset(1);
                            h =
                                (h <<
                                     5 as
                                         libc::c_int).wrapping_add(h).wrapping_add(*fresh4
                                                                                       as
                                                                                       libc::c_uint);
                            current_block_18 = 4404584186068957266;
                        }
                        4404584186068957266 => {
                            let fresh5 = k;
                            k = k.offset(1);
                            h =
                                (h <<
                                     5 as
                                         libc::c_int).wrapping_add(h).wrapping_add(*fresh5
                                                                                       as
                                                                                       libc::c_uint);
                            current_block_18 = 775645747195235551;
                        }
                        775645747195235551 => {
                            let fresh6 = k;
                            k = k.offset(1);
                            h =
                                (h <<
                                     5 as
                                         libc::c_int).wrapping_add(h).wrapping_add(*fresh6
                                                                                       as
                                                                                       libc::c_uint);
                            current_block_18 = 6409029412943682534;
                        }
                        _ => {
                            let fresh7 = k;
                            k = k.offset(1);
                            h =
                                (h <<
                                     5 as
                                         libc::c_int).wrapping_add(h).wrapping_add(*fresh7
                                                                                       as
                                                                                       libc::c_uint);
                            loop_0 = loop_0.wrapping_sub(1);
                            if loop_0 != 0 {
                                current_block_18 = 11875828834189669668;
                            } else {
                                current_block_18 = 11307063007268554308;
                            }
                        }
                    }
                }
            }
    }
    return h;
}
