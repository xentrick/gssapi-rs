use ::libc;

pub mod k5_thread_h {

    #[inline]

    pub unsafe extern "C" fn k5_mutex_finish_init(
        mut _m_0: *mut crate::k5_thread_h::k5_mutex_t,
    ) -> i32 {
        return 0i32;
    }
    #[inline]

    pub unsafe extern "C" fn k5_mutex_lock(mut m_0: *mut crate::k5_thread_h::k5_mutex_t) {
        let mut r = crate::k5_thread_h::k5_os_mutex_lock(m_0);
        if r != 0i32 {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"k5_mutex_lock: Received error %d (%s)\n\x00" as *const u8 as *const i8,
                r,
                crate::stdlib::strerror(r),
            );
        }
        if r == 0i32 {
        } else {
            crate::stdlib::__assert_fail(
                b"r == 0\x00" as *const u8 as *const i8,
                b"../../../include/k5-thread.h\x00" as *const u8 as *const i8,
                376u32,
                (*::std::mem::transmute::<&[u8; 33], &[i8; 33]>(
                    b"void k5_mutex_lock(k5_mutex_t *)\x00",
                ))
                .as_ptr(),
            );
        };
    }
    #[inline]

    pub unsafe extern "C" fn k5_mutex_unlock(mut m_0: *mut crate::k5_thread_h::k5_mutex_t) {
        let mut r = crate::k5_thread_h::k5_os_mutex_unlock(m_0);
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

pub mod errmap_h {

    #[inline]

    pub unsafe extern "C" fn mecherrmap__pairarray_init(
        mut arr: *mut crate::errmap_h::mecherrmap__pairarray,
    ) -> i32 {
        (*arr).elts = crate::stdlib::calloc(
            10usize,
            ::std::mem::size_of::<crate::errmap_h::mecherrmap__pair>(),
        ) as *mut crate::errmap_h::mecherrmap__pair;
        if (*arr).elts.is_null() {
            return 12i32;
        }
        (*arr).allocated = 10usize;
        return 0i32;
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap__pairarray_size(
        mut arr: *mut crate::errmap_h::mecherrmap__pairarray,
    ) -> isize {
        return (*arr).allocated as isize;
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap__pairarray_max_size(
        mut _arr: *mut crate::errmap_h::mecherrmap__pairarray,
    ) -> usize {
        let mut upper_bound: crate::stddef_h::size_t = 0;
        upper_bound = (18446744073709551615 as usize)
            .wrapping_div(::std::mem::size_of::<crate::errmap_h::mecherrmap__pair>());
        if upper_bound
            > (9223372036854775807 as usize)
                .wrapping_mul(2usize)
                .wrapping_add(1usize)
        {
            upper_bound = (9223372036854775807 as usize)
                .wrapping_mul(2usize)
                .wrapping_add(1usize)
        }
        return upper_bound;
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap__pairarray_grow(
        mut arr: *mut crate::errmap_h::mecherrmap__pairarray,
        mut newcount: usize,
    ) -> i32 {
        let mut oldsize = (::std::mem::size_of::<crate::errmap_h::mecherrmap__pair>())
            .wrapping_mul((*arr).allocated);
        let mut newsize: crate::stddef_h::size_t = 0;
        let mut ptr = 0 as *mut libc::c_void;
        if newcount > 9223372036854775807 as usize {
            return -(1i32);
        }
        if newcount < (*arr).allocated {
            return 0i32;
        }
        if newcount > mecherrmap__pairarray_max_size(arr) {
            return -(1i32);
        }
        newsize =
            (::std::mem::size_of::<crate::errmap_h::mecherrmap__pair>()).wrapping_mul(newcount);
        ptr = crate::stdlib::realloc((*arr).elts as *mut libc::c_void, newsize);
        if ptr.is_null() {
            return -(1i32);
        }
        crate::stdlib::memset(
            (ptr as *mut i8).offset(oldsize as isize) as *mut libc::c_void,
            0i32,
            newsize.wrapping_sub(oldsize),
        );
        (*arr).elts = ptr as *mut crate::errmap_h::mecherrmap__pair;
        (*arr).allocated = newcount;
        return 0i32;
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap__pairarray_getaddr(
        mut arr: *mut crate::errmap_h::mecherrmap__pairarray,
        mut idx: isize,
    ) -> *mut crate::errmap_h::mecherrmap__pair {
        if idx < 0isize || idx as usize >= (*arr).allocated {
            crate::stdlib::abort();
        }
        return (*arr).elts.offset(idx);
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap__pairarray_set(
        mut arr: *mut crate::errmap_h::mecherrmap__pairarray,
        mut idx: isize,
        mut value: crate::errmap_h::mecherrmap__pair,
    ) {
        let mut newvalp = 0 as *mut crate::errmap_h::mecherrmap__pair;
        newvalp = mecherrmap__pairarray_getaddr(arr, idx);
        *newvalp = value;
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap__pairarray_destroy(
        mut arr: *mut crate::errmap_h::mecherrmap__pairarray,
    ) {
        crate::stdlib::free((*arr).elts as *mut libc::c_void);
        (*arr).elts = 0 as *mut crate::errmap_h::mecherrmap__pair;
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap_init(mut m_0: *mut crate::errmap_h::mecherrmap) -> i32 {
        (*m_0).nextidx = 0isize;
        return mecherrmap__pairarray_init(&mut (*m_0).a);
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap_size(mut m_0: *mut crate::errmap_h::mecherrmap) -> isize {
        return mecherrmap__pairarray_size(&mut (*m_0).a);
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap_foreach(
        mut m_0: *mut crate::errmap_h::mecherrmap,
        mut fn_0: Option<
            unsafe extern "C" fn(
                _: crate::gssapi_h::OM_uint32,
                _: mecherror,
                _: *mut libc::c_void,
            ) -> i32,
        >,
        mut p: *mut libc::c_void,
    ) {
        let mut i: isize = 0;
        let mut sz: isize = 0;
        sz = (*m_0).nextidx;
        i = 0isize;
        while i < sz {
            let mut pair = 0 as *mut crate::errmap_h::mecherrmap__pair;
            pair = mecherrmap__pairarray_getaddr(&mut (*m_0).a, i);
            if Some(fn_0.expect("non-null function pointer")).expect("non-null function pointer")(
                (*pair).l,
                (*pair).r,
                p,
            ) != 0i32
            {
                break;
            }
            i += 1
        }
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap_add(
        mut m_0: *mut crate::errmap_h::mecherrmap,
        mut l: crate::gssapi_h::OM_uint32,
        mut r: mecherror,
    ) -> i32 {
        let mut i: isize = 0;
        let mut sz: isize = 0;
        let mut newpair = crate::errmap_h::mecherrmap__pair {
            l: 0,
            r: mecherror {
                mech: crate::gssapi_h::gss_OID_desc {
                    length: 0,
                    elements: 0 as *mut libc::c_void,
                },
                code: 0,
            },
        };
        let mut err: i32 = 0;
        sz = (*m_0).nextidx;
        /* Make sure we're not duplicating.  */
        i = 0isize;
        while i < sz {
            let mut pair = 0 as *mut crate::errmap_h::mecherrmap__pair;
            pair = mecherrmap__pairarray_getaddr(&mut (*m_0).a, i);
            if Some(
                Some(
                    cmp_OM_uint32
                        as unsafe extern "C" fn(
                            _: crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::OM_uint32,
                        ) -> i32,
                )
                .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(l, (*pair).l)
                != 0i32
            {
            } else {
                crate::stdlib::__assert_fail(
                    b"(*cmp_OM_uint32)(l, pair->l) != 0\x00" as *const u8 as *const i8,
                    b"./errmap.h\x00" as *const u8 as *const i8,
                    194u32,
                    (*::std::mem::transmute::<&[u8; 62], &[i8; 62]>(
                        b"int mecherrmap_add(mecherrmap *, OM_uint32, struct mecherror)\x00",
                    ))
                    .as_ptr(),
                );
            }
            if Some(
                Some(
                    cmp_OM_uint32
                        as unsafe extern "C" fn(
                            _: crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::OM_uint32,
                        ) -> i32,
                )
                .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(l, (*pair).l)
                == 0i32
            {
                crate::stdlib::abort();
            }
            if Some(
                Some(mecherror_cmp as unsafe extern "C" fn(_: mecherror, _: mecherror) -> i32)
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(r, (*pair).r)
                != 0i32
            {
            } else {
                crate::stdlib::__assert_fail(
                    b"(*mecherror_cmp)(r, pair->r) != 0\x00" as *const u8 as *const i8,
                    b"./errmap.h\x00" as *const u8 as *const i8,
                    197u32,
                    (*::std::mem::transmute::<&[u8; 62], &[i8; 62]>(
                        b"int mecherrmap_add(mecherrmap *, OM_uint32, struct mecherror)\x00",
                    ))
                    .as_ptr(),
                );
            }
            if Some(
                Some(mecherror_cmp as unsafe extern "C" fn(_: mecherror, _: mecherror) -> i32)
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(r, (*pair).r)
                == 0i32
            {
                crate::stdlib::abort();
            }
            i += 1
        }
        newpair.l = l;
        newpair.r = r;
        if sz >= 9223372036854775807 as isize - 1isize {
            return 12i32;
        }
        err = mecherrmap__pairarray_grow(&mut (*m_0).a, (sz + 1isize) as usize);
        if err != 0 {
            return err;
        }
        mecherrmap__pairarray_set(&mut (*m_0).a, sz, newpair);
        (*m_0).nextidx += 1;
        return 0i32;
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap_findleft(
        mut m_0: *mut crate::errmap_h::mecherrmap,
        mut l: crate::gssapi_h::OM_uint32,
    ) -> *const crate::errmap_h::mecherrmap__right_t {
        let mut i: isize = 0;
        let mut sz: isize = 0;
        sz = mecherrmap_size(m_0);
        i = 0isize;
        while i < sz {
            let mut pair = 0 as *mut crate::errmap_h::mecherrmap__pair;
            pair = mecherrmap__pairarray_getaddr(&mut (*m_0).a, i);
            if Some(
                Some(
                    cmp_OM_uint32
                        as unsafe extern "C" fn(
                            _: crate::gssapi_h::OM_uint32,
                            _: crate::gssapi_h::OM_uint32,
                        ) -> i32,
                )
                .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(l, (*pair).l)
                == 0i32
            {
                return &mut (*pair).r;
            }
            i += 1
        }
        return 0 as *const crate::errmap_h::mecherrmap__right_t;
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap_findright(
        mut m_0: *mut crate::errmap_h::mecherrmap,
        mut r: mecherror,
    ) -> *const crate::errmap_h::mecherrmap__left_t {
        let mut i: isize = 0;
        let mut sz: isize = 0;
        sz = mecherrmap_size(m_0);
        i = 0isize;
        while i < sz {
            let mut pair = 0 as *mut crate::errmap_h::mecherrmap__pair;
            pair = mecherrmap__pairarray_getaddr(&mut (*m_0).a, i);
            if Some(
                Some(mecherror_cmp as unsafe extern "C" fn(_: mecherror, _: mecherror) -> i32)
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(r, (*pair).r)
                == 0i32
            {
                return &mut (*pair).l;
            }
            i += 1
        }
        return 0 as *const crate::errmap_h::mecherrmap__left_t;
    }
    #[inline]

    pub unsafe extern "C" fn mecherrmap_destroy(mut m_0: *mut crate::errmap_h::mecherrmap) {
        mecherrmap__pairarray_destroy(&mut (*m_0).a);
    }

    use crate::src::generic::util_errmap::cmp_OM_uint32;
    use crate::src::generic::util_errmap::mecherror;
    use crate::src::generic::util_errmap::mecherror_cmp;

    /* end of t_bimap body template */
}

pub use crate::stddef_h::size_t;

pub use crate::k5_thread_h::k5_mutex_t;
pub use crate::k5_thread_h::k5_os_mutex;
pub use crate::k5_thread_h::k5_os_mutex_destroy;
pub use crate::k5_thread_h::k5_os_mutex_lock;
pub use crate::k5_thread_h::k5_os_mutex_unlock;
pub use crate::src::generic::util_errmap::k5_thread_h::k5_mutex_finish_init;
pub use crate::src::generic::util_errmap::k5_thread_h::k5_mutex_lock;
pub use crate::src::generic::util_errmap::k5_thread_h::k5_mutex_unlock;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::pthread_mutex_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::errmap_h::mecherrmap;
pub use crate::errmap_h::mecherrmap__left_t;
pub use crate::errmap_h::mecherrmap__pair;
pub use crate::errmap_h::mecherrmap__pairarray;
pub use crate::errmap_h::mecherrmap__pairarray__header;
pub use crate::errmap_h::mecherrmap__right_t;
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap__pairarray_destroy;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap__pairarray_getaddr;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap__pairarray_grow;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap__pairarray_init;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap__pairarray_max_size;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap__pairarray_set;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap__pairarray_size;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap_add;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap_destroy;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap_findleft;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap_findright;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap_foreach;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap_init;
pub use crate::src::generic::util_errmap::errmap_h::mecherrmap_size;

/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2007, 2008 by the Massachusetts Institute of Technology.
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
 *
 */
/* The mapping table is 0-based, but let's export codes that are
1-based, keeping 0 for errors or unknown errors.

The elements in the mapping table currently have separate copies of
each OID stored.  This is a bit wasteful, but we are assuming the
table isn't likely to grow very large.  */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mecherror {
    pub mech: crate::gssapi_h::gss_OID_desc,
    pub code: crate::gssapi_h::OM_uint32,
}
#[inline]

unsafe extern "C" fn cmp_OM_uint32(
    mut m1: crate::gssapi_h::OM_uint32,
    mut m2: crate::gssapi_h::OM_uint32,
) -> i32 {
    if m1 < m2 {
        return -(1i32);
    } else if m1 > m2 {
        return 1i32;
    } else {
        return 0i32;
    };
}
#[inline]

unsafe extern "C" fn mecherror_cmp(mut m1: mecherror, mut m2: mecherror) -> i32 {
    if m1.code < m2.code {
        return -(1i32);
    }
    if m1.code > m2.code {
        return 1i32;
    }
    if m1.mech.length < m2.mech.length {
        return -(1i32);
    }
    if m1.mech.length > m2.mech.length {
        return 1i32;
    }
    if m1.mech.length == 0u32 {
        return 0i32;
    }
    return crate::stdlib::memcmp(m1.mech.elements, m2.mech.elements, m1.mech.length as usize);
}
#[inline]

unsafe extern "C" fn mecherror_copy(mut dest: *mut mecherror, mut src: mecherror) -> i32 {
    *dest = src;
    if src.mech.length > 0u32 {
        (*dest).mech.elements = crate::stdlib::malloc(src.mech.length as usize);
        if (*dest).mech.elements.is_null() {
            return 12i32;
        }
        crate::stdlib::memcpy(
            (*dest).mech.elements,
            src.mech.elements,
            src.mech.length as usize,
        );
    } else {
        (*dest).mech.elements = 0 as *mut libc::c_void
    }
    return 0i32;
}

static mut m: crate::errmap_h::mecherrmap = crate::errmap_h::mecherrmap {
    a: crate::errmap_h::mecherrmap__pairarray {
        allocated: 0,
        elts: 0 as *mut crate::errmap_h::mecherrmap__pair,
    },
    nextidx: 0,
};

static mut mutex: crate::k5_thread_h::k5_mutex_t = crate::stdlib::pthread_mutex_t {
    __data: {
        let mut init = crate::stdlib::__pthread_mutex_s {
            __lock: 0i32,
            __count: 0u32,
            __owner: 0i32,
            __nusers: 0u32,
            __kind: 0i32,
            __spins: 0i16,
            __elision: 0i16,
            __list: {
                let mut init = crate::stdlib::__pthread_internal_list {
                    __prev: 0 as *mut crate::stdlib::__pthread_internal_list,
                    __next: 0 as *mut crate::stdlib::__pthread_internal_list,
                };
                init
            },
        };
        init
    },
};

static mut next_fake: crate::gssapi_h::OM_uint32 = 100000u32;
#[no_mangle]

pub unsafe extern "C" fn gssint_mecherrmap_init() -> i32 {
    let mut err: i32 = 0;
    err = mecherrmap_init(&mut m);
    if err != 0 {
        return err;
    }
    err = k5_mutex_finish_init(&mut mutex);
    if err != 0 {
        mecherrmap_destroy(&mut m);
        return err;
    }
    return 0i32;
}
/* Currently the enumeration template doesn't handle freeing
element storage when destroying the collection.  */

unsafe extern "C" fn free_one(
    mut _i: crate::gssapi_h::OM_uint32,
    mut value: mecherror,
    mut _p: *mut libc::c_void,
) -> i32 {
    crate::stdlib::free(value.mech.elements);
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn gssint_mecherrmap_destroy() {
    mecherrmap_foreach(
        &mut m,
        Some(
            free_one
                as unsafe extern "C" fn(
                    _: crate::gssapi_h::OM_uint32,
                    _: mecherror,
                    _: *mut libc::c_void,
                ) -> i32,
        ),
        0 as *mut libc::c_void,
    );
    mecherrmap_destroy(&mut m);
    crate::k5_thread_h::k5_os_mutex_destroy(&mut mutex);
}
#[no_mangle]

pub unsafe extern "C" fn gssint_mecherrmap_map(
    mut minor: crate::gssapi_h::OM_uint32,
    mut oid: *const crate::gssapi_h::gss_OID_desc,
) -> crate::gssapi_h::OM_uint32 {
    let mut mep = 0 as *const mecherror;
    let mut me = mecherror {
        mech: crate::gssapi_h::gss_OID_desc {
            length: 0,
            elements: 0 as *mut libc::c_void,
        },
        code: 0,
    };
    let mut me_copy = mecherror {
        mech: crate::gssapi_h::gss_OID_desc {
            length: 0,
            elements: 0 as *mut libc::c_void,
        },
        code: 0,
    };
    let mut p = 0 as *const crate::gssapi_h::OM_uint32;
    let mut err: i32 = 0;
    let mut new_status: crate::gssapi_h::OM_uint32 = 0;
    me.code = minor;
    me.mech = *oid;
    k5_mutex_lock(&mut mutex);
    /* Is this status+oid already mapped?  */
    p = mecherrmap_findright(&mut m, me);
    if !p.is_null() {
        k5_mutex_unlock(&mut mutex);
        return *p;
    }
    /* Is this status code already mapped to something else
    mech-specific?  */
    mep = mecherrmap_findleft(&mut m, minor);
    if mep.is_null() {
        /* Map it to itself plus this mech-oid.  */
        new_status = minor
    } else {
        loop
        /* Already assigned.  Pick a fake new value and map it.  */
        /* There's a theoretical infinite loop risk here, if we fill
           in 2**32 values.  Also, returning 0 has a special
           meaning.  */
        {
            next_fake = next_fake.wrapping_add(1);
            new_status = next_fake;
            (new_status) == 0u32;
            if mecherrmap_findleft(&mut m, new_status).is_null() {
                break;
            }
        }
    }
    err = mecherror_copy(&mut me_copy, me);
    if err != 0 {
        k5_mutex_unlock(&mut mutex);
        return err as crate::gssapi_h::OM_uint32;
    }
    err = mecherrmap_add(&mut m, new_status, me_copy);
    k5_mutex_unlock(&mut mutex);
    if err != 0 {
        crate::stdlib::free(me_copy.mech.elements);
    }
    if err != 0 {
        return 0u32;
    } else {
        return new_status;
    };
}

static mut no_oid: crate::gssapi_h::gss_OID_desc = {
    let mut init = crate::gssapi_h::gss_OID_desc_struct {
        length: 0u32,
        elements: 0 as *mut libc::c_void,
    };
    init
};
#[no_mangle]

pub unsafe extern "C" fn gssint_mecherrmap_map_errcode(
    mut errcode: crate::gssapi_h::OM_uint32,
) -> crate::gssapi_h::OM_uint32 {
    return gssint_mecherrmap_map(errcode, &mut no_oid);
}
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * $Id$
 */
/* * helper macros **/
/* this code knows that an int on the wire is 32 bits.  The type of
num should be at least this big, or the extra shifts may do weird
things */
/* * malloc wrappers; these may actually do something later */
/* * helper functions **/
/* hide names from applications, especially glib applications */
/* flags for g_verify_token_header() */
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* buffer */
/* minor_status */
/* set */
/* minor_status */
/* set */
/* minor_status */
/* oid */
/* new_oid */
/* minor_status */
/* oid_set */
/* minor_status */
/* member_oid */
/* oid_set */
/* minor_status */
/* member */
/* set */
/* present */
/* minor_status */
/* oid */
/* oid_str */
/* minor_status */
/* oid_str */
/* oid */
/* minor_status */
/* prefix */
/* prefix_len */
/* suffix */
/* oid */
/* minor_status */
/*prefix */
/* prefix_len */
/* oid */
/* suffix */
#[no_mangle]

pub unsafe extern "C" fn gssint_mecherrmap_get(
    mut minor: crate::gssapi_h::OM_uint32,
    mut mech_oid: crate::gssapi_h::gss_OID,
    mut mech_minor: *mut crate::gssapi_h::OM_uint32,
) -> i32 {
    let mut p = 0 as *const mecherror;
    if minor == 0u32 {
        return 22i32;
    }
    k5_mutex_lock(&mut mutex);
    p = mecherrmap_findleft(&mut m, minor);
    k5_mutex_unlock(&mut mutex);
    if p.is_null() {
        return 22i32;
    }
    *mech_oid = (*p).mech;
    *mech_minor = (*p).code;
    return 0i32;
}
