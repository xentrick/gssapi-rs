use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:27"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:27"]
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:27"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:27"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "360:1"]
    pub unsafe extern "C" fn k5_mutex_finish_init(mut m_0: *mut k5_mutex_t)
     -> libc::c_int {
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m_0: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m_0);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m_0: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m_0);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          388 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void k5_mutex_unlock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    use super::pthreadtypes_h::pthread_mutex_t;
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m_0: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m_0: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m_0: *mut k5_os_mutex) -> libc::c_int;
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:27"]
pub mod gssapi_h {
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    use super::stdint_uintn_h::uint32_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/errmap.h:131"]
pub mod errmap_h {
    #[c2rust::src_loc = "149:1"]
    pub type mecherrmap__right_t = mecherror;
    /*
 * This file is generated, please don't edit it.
 * script: ../../../util/gen.pl
 * args:   bimap errmap.h NAME=mecherrmap LEFT=OM_uint32 RIGHT=struct mecherror LEFTPRINT=print_OM_uint32 RIGHTPRINT=mecherror_print LEFTCMP=cmp_OM_uint32 RIGHTCMP=mecherror_cmp
 * The rest of this file is copied from a template, with
 * substitutions.  See the template for copyright info.
 */
/* start of t_bimap header template */
/*
 * bidirectional mapping table, add-only
 *
 * Parameters:
 * NAME
 * LEFT, RIGHT - types
 * LEFTCMP, RIGHTCMP - comparison functions
 *
 * Methods:
 * int init() - nonzero is error code, if any possible
 * long size()
 * void foreach(int (*)(LEFT, RIGHT, void*), void*)
 * int add(LEFT, RIGHT) - 0 = success, -1 = allocation failure
 * const struct mecherror *findleft(OM_uint32) - null iff not found
 * const OM_uint32 *findright(struct mecherror)
 * void destroy() - destroys container, doesn't delete elements
 *
 * initial implementation: flat array of (left,right) pairs
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:8"]
    pub struct mecherrmap__pair {
        pub l: OM_uint32,
        pub r: mecherror,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "151:9"]
    pub struct mecherrmap {
        pub a: mecherrmap__pairarray,
        pub nextidx: libc::c_long,
    }
    #[c2rust::src_loc = "62:1"]
    pub type mecherrmap__pairarray = mecherrmap__pairarray__header;
    /* end of t_bimap header template */
/* start of t_array template */
    /*
 * array type, derived from template
 *
 * parameters:
 * NAME: mecherrmap__pairarray
 * TYPE: struct mecherrmap__pair
 *
 * methods:
 * int init() -> nonzero if fail initial allocation
 * unsigned long size() -> nonnegative number of values stored
 * int grow(newsize) -> negative if fail allocation, memset(,0,) new space
 * struct mecherrmap__pair *getaddr(idx) -> aborts if out of range
 * void set(idx, value) -> aborts if out of range
 * struct mecherrmap__pair get(idx) -> value, or aborts if out of range
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:8"]
    pub struct mecherrmap__pairarray__header {
        pub allocated: size_t,
        pub elts: *mut mecherrmap__pair,
    }
    /* end of t_array template */
/* start of t_bimap body template */
    /* for use in cases where text substitutions may not work, like putting
   "const" before a type that turns out to be "char *"  */
    #[c2rust::src_loc = "148:1"]
    pub type mecherrmap__left_t = OM_uint32;
    #[inline]
    #[c2rust::src_loc = "64:1"]
    pub unsafe extern "C" fn mecherrmap__pairarray_init(mut arr:
                                                            *mut mecherrmap__pairarray)
     -> libc::c_int {
        (*arr).elts =
            calloc(10 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<mecherrmap__pair>() as libc::c_ulong)
                as *mut mecherrmap__pair;
        if (*arr).elts.is_null() { return 12 as libc::c_int }
        (*arr).allocated = 10 as libc::c_int as size_t;
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "74:1"]
    pub unsafe extern "C" fn mecherrmap__pairarray_size(mut arr:
                                                            *mut mecherrmap__pairarray)
     -> libc::c_long {
        return (*arr).allocated as libc::c_long;
    }
    #[inline]
    #[c2rust::src_loc = "80:1"]
    pub unsafe extern "C" fn mecherrmap__pairarray_max_size(mut arr:
                                                                *mut mecherrmap__pairarray)
     -> libc::c_ulong {
        let mut upper_bound: size_t = 0;
        upper_bound =
            (18446744073709551615 as
                 libc::c_ulong).wrapping_div(::std::mem::size_of::<mecherrmap__pair>()
                                                 as libc::c_ulong);
        if upper_bound >
               (9223372036854775807 as libc::c_long as
                    libc::c_ulong).wrapping_mul(2 as
                                                    libc::c_ulong).wrapping_add(1
                                                                                    as
                                                                                    libc::c_ulong)
           {
            upper_bound =
                (9223372036854775807 as libc::c_long as
                     libc::c_ulong).wrapping_mul(2 as
                                                     libc::c_ulong).wrapping_add(1
                                                                                     as
                                                                                     libc::c_ulong)
        }
        return upper_bound;
    }
    #[inline]
    #[c2rust::src_loc = "91:1"]
    pub unsafe extern "C" fn mecherrmap__pairarray_grow(mut arr:
                                                            *mut mecherrmap__pairarray,
                                                        mut newcount:
                                                            libc::c_ulong)
     -> libc::c_int {
        let mut oldsize: size_t =
            (::std::mem::size_of::<mecherrmap__pair>() as
                 libc::c_ulong).wrapping_mul((*arr).allocated);
        let mut newsize: size_t = 0;
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        if newcount > 9223372036854775807 as libc::c_long as libc::c_ulong {
            return -(1 as libc::c_int)
        }
        if newcount < (*arr).allocated { return 0 as libc::c_int }
        if newcount > mecherrmap__pairarray_max_size(arr) {
            return -(1 as libc::c_int)
        }
        newsize =
            (::std::mem::size_of::<mecherrmap__pair>() as
                 libc::c_ulong).wrapping_mul(newcount);
        ptr = realloc((*arr).elts as *mut libc::c_void, newsize);
        if ptr.is_null() { return -(1 as libc::c_int) }
        memset((ptr as *mut libc::c_char).offset(oldsize as isize) as
                   *mut libc::c_void, 0 as libc::c_int,
               newsize.wrapping_sub(oldsize));
        (*arr).elts = ptr as *mut mecherrmap__pair;
        (*arr).allocated = newcount;
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "115:1"]
    pub unsafe extern "C" fn mecherrmap__pairarray_getaddr(mut arr:
                                                               *mut mecherrmap__pairarray,
                                                           mut idx:
                                                               libc::c_long)
     -> *mut mecherrmap__pair {
        if idx < 0 as libc::c_int as libc::c_long ||
               idx as libc::c_ulong >= (*arr).allocated {
            abort();
        }
        return (*arr).elts.offset(idx as isize);
    }
    #[inline]
    #[c2rust::src_loc = "123:1"]
    pub unsafe extern "C" fn mecherrmap__pairarray_set(mut arr:
                                                           *mut mecherrmap__pairarray,
                                                       mut idx: libc::c_long,
                                                       mut value:
                                                           mecherrmap__pair) {
        let mut newvalp: *mut mecherrmap__pair = 0 as *mut mecherrmap__pair;
        newvalp = mecherrmap__pairarray_getaddr(arr, idx);
        *newvalp = value;
    }
    #[inline]
    #[c2rust::src_loc = "137:1"]
    pub unsafe extern "C" fn mecherrmap__pairarray_destroy(mut arr:
                                                               *mut mecherrmap__pairarray) {
        free((*arr).elts as *mut libc::c_void);
        (*arr).elts = 0 as *mut mecherrmap__pair;
    }
    #[inline]
    #[c2rust::src_loc = "156:1"]
    pub unsafe extern "C" fn mecherrmap_init(mut m_0: *mut mecherrmap)
     -> libc::c_int {
        (*m_0).nextidx = 0 as libc::c_int as libc::c_long;
        return mecherrmap__pairarray_init(&mut (*m_0).a);
    }
    #[inline]
    #[c2rust::src_loc = "163:1"]
    pub unsafe extern "C" fn mecherrmap_size(mut m_0: *mut mecherrmap)
     -> libc::c_long {
        return mecherrmap__pairarray_size(&mut (*m_0).a);
    }
    #[inline]
    #[c2rust::src_loc = "169:1"]
    pub unsafe extern "C" fn mecherrmap_foreach(mut m_0: *mut mecherrmap,
                                                mut fn_0:
                                                    Option<unsafe extern "C" fn(_:
                                                                                    OM_uint32,
                                                                                _:
                                                                                    mecherror,
                                                                                _:
                                                                                    *mut libc::c_void)
                                                               ->
                                                                   libc::c_int>,
                                                mut p: *mut libc::c_void) {
        let mut i: libc::c_long = 0;
        let mut sz: libc::c_long = 0;
        sz = (*m_0).nextidx;
        i = 0 as libc::c_int as libc::c_long;
        while i < sz {
            let mut pair: *mut mecherrmap__pair = 0 as *mut mecherrmap__pair;
            pair = mecherrmap__pairarray_getaddr(&mut (*m_0).a, i);
            if Some(fn_0.expect("non-null function pointer")).expect("non-null function pointer")((*pair).l,
                                                                                                  (*pair).r,
                                                                                                  p)
                   != 0 as libc::c_int {
                break ;
            }
            i += 1
        };
    }
    #[inline]
    #[c2rust::src_loc = "182:1"]
    pub unsafe extern "C" fn mecherrmap_add(mut m_0: *mut mecherrmap,
                                            mut l: OM_uint32,
                                            mut r: mecherror) -> libc::c_int {
        let mut i: libc::c_long = 0;
        let mut sz: libc::c_long = 0;
        let mut newpair: mecherrmap__pair =
            mecherrmap__pair{l: 0,
                             r:
                                 mecherror{mech:
                                               gss_OID_desc{length: 0,
                                                            elements:
                                                                0 as
                                                                    *mut libc::c_void,},
                                           code: 0,},};
        let mut err: libc::c_int = 0;
        sz = (*m_0).nextidx;
        /* Make sure we're not duplicating.  */
        i = 0 as libc::c_int as libc::c_long;
        while i < sz {
            let mut pair: *mut mecherrmap__pair = 0 as *mut mecherrmap__pair;
            pair = mecherrmap__pairarray_getaddr(&mut (*m_0).a, i);
            if Some(Some(cmp_OM_uint32 as
                             unsafe extern "C" fn(_: OM_uint32, _: OM_uint32)
                                 ->
                                     libc::c_int).expect("non-null function pointer")).expect("non-null function pointer")(l,
                                                                                                                           (*pair).l)
                   != 0 as libc::c_int {
            } else {
                __assert_fail(b"(*cmp_OM_uint32)(l, pair->l) != 0\x00" as
                                  *const u8 as *const libc::c_char,
                              b"./errmap.h\x00" as *const u8 as
                                  *const libc::c_char,
                              194 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 62],
                                                        &[libc::c_char; 62]>(b"int mecherrmap_add(mecherrmap *, OM_uint32, struct mecherror)\x00")).as_ptr());
            }
            if Some(Some(cmp_OM_uint32 as
                             unsafe extern "C" fn(_: OM_uint32, _: OM_uint32)
                                 ->
                                     libc::c_int).expect("non-null function pointer")).expect("non-null function pointer")(l,
                                                                                                                           (*pair).l)
                   == 0 as libc::c_int {
                abort();
            }
            if Some(Some(mecherror_cmp as
                             unsafe extern "C" fn(_: mecherror, _: mecherror)
                                 ->
                                     libc::c_int).expect("non-null function pointer")).expect("non-null function pointer")(r,
                                                                                                                           (*pair).r)
                   != 0 as libc::c_int {
            } else {
                __assert_fail(b"(*mecherror_cmp)(r, pair->r) != 0\x00" as
                                  *const u8 as *const libc::c_char,
                              b"./errmap.h\x00" as *const u8 as
                                  *const libc::c_char,
                              197 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 62],
                                                        &[libc::c_char; 62]>(b"int mecherrmap_add(mecherrmap *, OM_uint32, struct mecherror)\x00")).as_ptr());
            }
            if Some(Some(mecherror_cmp as
                             unsafe extern "C" fn(_: mecherror, _: mecherror)
                                 ->
                                     libc::c_int).expect("non-null function pointer")).expect("non-null function pointer")(r,
                                                                                                                           (*pair).r)
                   == 0 as libc::c_int {
                abort();
            }
            i += 1
        }
        newpair.l = l;
        newpair.r = r;
        if sz >=
               9223372036854775807 as libc::c_long -
                   1 as libc::c_int as libc::c_long {
            return 12 as libc::c_int
        }
        err =
            mecherrmap__pairarray_grow(&mut (*m_0).a,
                                       (sz + 1 as libc::c_int as libc::c_long)
                                           as libc::c_ulong);
        if err != 0 { return err }
        mecherrmap__pairarray_set(&mut (*m_0).a, sz, newpair);
        (*m_0).nextidx += 1;
        return 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "213:1"]
    pub unsafe extern "C" fn mecherrmap_findleft(mut m_0: *mut mecherrmap,
                                                 mut l: OM_uint32)
     -> *const mecherrmap__right_t {
        let mut i: libc::c_long = 0;
        let mut sz: libc::c_long = 0;
        sz = mecherrmap_size(m_0);
        i = 0 as libc::c_int as libc::c_long;
        while i < sz {
            let mut pair: *mut mecherrmap__pair = 0 as *mut mecherrmap__pair;
            pair = mecherrmap__pairarray_getaddr(&mut (*m_0).a, i);
            if Some(Some(cmp_OM_uint32 as
                             unsafe extern "C" fn(_: OM_uint32, _: OM_uint32)
                                 ->
                                     libc::c_int).expect("non-null function pointer")).expect("non-null function pointer")(l,
                                                                                                                           (*pair).l)
                   == 0 as libc::c_int {
                return &mut (*pair).r
            }
            i += 1
        }
        return 0 as *const mecherrmap__right_t;
    }
    #[inline]
    #[c2rust::src_loc = "227:1"]
    pub unsafe extern "C" fn mecherrmap_findright(mut m_0: *mut mecherrmap,
                                                  mut r: mecherror)
     -> *const mecherrmap__left_t {
        let mut i: libc::c_long = 0;
        let mut sz: libc::c_long = 0;
        sz = mecherrmap_size(m_0);
        i = 0 as libc::c_int as libc::c_long;
        while i < sz {
            let mut pair: *mut mecherrmap__pair = 0 as *mut mecherrmap__pair;
            pair = mecherrmap__pairarray_getaddr(&mut (*m_0).a, i);
            if Some(Some(mecherror_cmp as
                             unsafe extern "C" fn(_: mecherror, _: mecherror)
                                 ->
                                     libc::c_int).expect("non-null function pointer")).expect("non-null function pointer")(r,
                                                                                                                           (*pair).r)
                   == 0 as libc::c_int {
                return &mut (*pair).l
            }
            i += 1
        }
        return 0 as *const mecherrmap__left_t;
    }
    #[inline]
    #[c2rust::src_loc = "269:1"]
    pub unsafe extern "C" fn mecherrmap_destroy(mut m_0: *mut mecherrmap) {
        mecherrmap__pairarray_destroy(&mut (*m_0).a);
    }
    use super::{mecherror, cmp_OM_uint32, mecherror_cmp};
    use super::gssapi_h::OM_uint32;
    use super::stddef_h::size_t;
    use super::stdlib_h::{calloc, realloc, abort, free};
    use super::string_h::memset;
    use super::assert_h::__assert_fail;
    /* end of t_bimap body template */
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
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
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:27"]
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
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __off_t, __off64_t};
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_finish_init,
                            k5_mutex_lock, k5_mutex_unlock,
                            k5_os_mutex_destroy, k5_os_mutex_lock,
                            k5_os_mutex_unlock};
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_uint32, OM_uint32, gss_OID_desc_struct,
                         gss_OID_desc, gss_OID};
pub use self::errmap_h::{mecherrmap__right_t, mecherrmap__pair, mecherrmap,
                         mecherrmap__pairarray, mecherrmap__pairarray__header,
                         mecherrmap__left_t, mecherrmap__pairarray_init,
                         mecherrmap__pairarray_size,
                         mecherrmap__pairarray_max_size,
                         mecherrmap__pairarray_grow,
                         mecherrmap__pairarray_getaddr,
                         mecherrmap__pairarray_set,
                         mecherrmap__pairarray_destroy, mecherrmap_init,
                         mecherrmap_size, mecherrmap_foreach, mecherrmap_add,
                         mecherrmap_findleft, mecherrmap_findright,
                         mecherrmap_destroy};
use self::stdlib_h::{malloc, calloc, realloc, free, abort};
use self::assert_h::__assert_fail;
use self::stdio_h::{stderr, fprintf};
use self::string_h::{memcpy, memset, memcmp, strerror};
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "40:8"]
pub struct mecherror {
    pub mech: gss_OID_desc,
    pub code: OM_uint32,
}
#[inline]
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn cmp_OM_uint32(mut m1: OM_uint32, mut m2: OM_uint32)
 -> libc::c_int {
    if m1 < m2 {
        return -(1 as libc::c_int)
    } else if m1 > m2 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[inline]
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn mecherror_cmp(mut m1: mecherror, mut m2: mecherror)
 -> libc::c_int {
    if m1.code < m2.code { return -(1 as libc::c_int) }
    if m1.code > m2.code { return 1 as libc::c_int }
    if m1.mech.length < m2.mech.length { return -(1 as libc::c_int) }
    if m1.mech.length > m2.mech.length { return 1 as libc::c_int }
    if m1.mech.length == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int
    }
    return memcmp(m1.mech.elements, m2.mech.elements,
                  m1.mech.length as libc::c_ulong);
}
#[inline]
#[c2rust::src_loc = "78:1"]
unsafe extern "C" fn mecherror_copy(mut dest: *mut mecherror,
                                    mut src: mecherror) -> libc::c_int {
    *dest = src;
    if src.mech.length > 0 as libc::c_int as libc::c_uint {
        (*dest).mech.elements = malloc(src.mech.length as libc::c_ulong);
        if (*dest).mech.elements.is_null() { return 12 as libc::c_int }
        memcpy((*dest).mech.elements, src.mech.elements,
               src.mech.length as libc::c_ulong);
    } else { (*dest).mech.elements = 0 as *mut libc::c_void }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "134:19"]
static mut m: mecherrmap =
    mecherrmap{a:
                   mecherrmap__pairarray{allocated: 0,
                                         elts:
                                             0 as *const mecherrmap__pair as
                                                 *mut mecherrmap__pair,},
               nextidx: 0,};
#[c2rust::src_loc = "135:19"]
static mut mutex: k5_mutex_t =
    pthread_mutex_t{__data:
                        {
                            let mut init =
                                __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                  __count:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __owner: 0 as libc::c_int,
                                                  __nusers:
                                                      0 as libc::c_int as
                                                          libc::c_uint,
                                                  __kind: 0 as libc::c_int,
                                                  __spins:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __elision:
                                                      0 as libc::c_int as
                                                          libc::c_short,
                                                  __list:
                                                      {
                                                          let mut init =
                                                              __pthread_internal_list{__prev:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,
                                                                                      __next:
                                                                                          0
                                                                                              as
                                                                                              *const __pthread_internal_list
                                                                                              as
                                                                                              *mut __pthread_internal_list,};
                                                          init
                                                      },};
                            init
                        },};
#[c2rust::src_loc = "136:18"]
static mut next_fake: OM_uint32 = 100000 as libc::c_int as OM_uint32;
#[no_mangle]
#[c2rust::src_loc = "138:1"]
pub unsafe extern "C" fn gssint_mecherrmap_init() -> libc::c_int {
    let mut err: libc::c_int = 0;
    err = mecherrmap_init(&mut m);
    if err != 0 { return err }
    err = k5_mutex_finish_init(&mut mutex);
    if err != 0 { mecherrmap_destroy(&mut m); return err }
    return 0 as libc::c_int;
}
/* Currently the enumeration template doesn't handle freeing
   element storage when destroying the collection.  */
#[c2rust::src_loc = "156:1"]
unsafe extern "C" fn free_one(mut i: OM_uint32, mut value: mecherror,
                              mut p: *mut libc::c_void) -> libc::c_int {
    free(value.mech.elements);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "162:1"]
pub unsafe extern "C" fn gssint_mecherrmap_destroy() {
    mecherrmap_foreach(&mut m,
                       Some(free_one as
                                unsafe extern "C" fn(_: OM_uint32,
                                                     _: mecherror,
                                                     _: *mut libc::c_void)
                                    -> libc::c_int), 0 as *mut libc::c_void);
    mecherrmap_destroy(&mut m);
    k5_os_mutex_destroy(&mut mutex);
}
#[no_mangle]
#[c2rust::src_loc = "169:1"]
pub unsafe extern "C" fn gssint_mecherrmap_map(mut minor: OM_uint32,
                                               mut oid: *const gss_OID_desc)
 -> OM_uint32 {
    let mut mep: *const mecherror = 0 as *const mecherror;
    let mut me: mecherror =
        mecherror{mech:
                      gss_OID_desc{length: 0,
                                   elements: 0 as *mut libc::c_void,},
                  code: 0,};
    let mut me_copy: mecherror =
        mecherror{mech:
                      gss_OID_desc{length: 0,
                                   elements: 0 as *mut libc::c_void,},
                  code: 0,};
    let mut p: *const OM_uint32 = 0 as *const OM_uint32;
    let mut err: libc::c_int = 0;
    let mut new_status: OM_uint32 = 0;
    me.code = minor;
    me.mech = *oid;
    k5_mutex_lock(&mut mutex);
    /* Is this status+oid already mapped?  */
    p = mecherrmap_findright(&mut m, me);
    if !p.is_null() { k5_mutex_unlock(&mut mutex); return *p }
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
            (new_status) == 0 as libc::c_int as libc::c_uint;
            if mecherrmap_findleft(&mut m, new_status).is_null() { break ; }
        }
    }
    err = mecherror_copy(&mut me_copy, me);
    if err != 0 { k5_mutex_unlock(&mut mutex); return err as OM_uint32 }
    err = mecherrmap_add(&mut m, new_status, me_copy);
    k5_mutex_unlock(&mut mutex);
    if err != 0 { free(me_copy.mech.elements); }
    if err != 0 {
        return 0 as libc::c_int as OM_uint32
    } else { return new_status };
}
#[c2rust::src_loc = "241:21"]
static mut no_oid: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 0 as libc::c_int as OM_uint32,
                                elements:
                                    0 as *const libc::c_void as
                                        *mut libc::c_void,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "242:1"]
pub unsafe extern "C" fn gssint_mecherrmap_map_errcode(mut errcode: OM_uint32)
 -> OM_uint32 {
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
#[c2rust::src_loc = "247:1"]
pub unsafe extern "C" fn gssint_mecherrmap_get(mut minor: OM_uint32,
                                               mut mech_oid: gss_OID,
                                               mut mech_minor: *mut OM_uint32)
 -> libc::c_int {
    let mut p: *const mecherror = 0 as *const mecherror;
    if minor == 0 as libc::c_int as libc::c_uint { return 22 as libc::c_int }
    k5_mutex_lock(&mut mutex);
    p = mecherrmap_findleft(&mut m, minor);
    k5_mutex_unlock(&mut mutex);
    if p.is_null() { return 22 as libc::c_int }
    *mech_oid = (*p).mech;
    *mech_minor = (*p).code;
    return 0 as libc::c_int;
}
