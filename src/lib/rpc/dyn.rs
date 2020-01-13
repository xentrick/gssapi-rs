use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:17"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:17"]
pub mod types_h {
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:17"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:17"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/lib/rpc/dyn.h:21"]
pub mod dyn_h {
    #[c2rust::src_loc = "26:1"]
    pub type DynPtr = *mut libc::c_char;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:16"]
    pub struct _DynObject {
        pub array: DynPtr,
        pub el_size: libc::c_int,
        pub num_el: libc::c_int,
        pub size: libc::c_int,
        pub inc: libc::c_int,
        pub debug: libc::c_int,
        pub paranoid: libc::c_int,
        pub initzero: libc::c_int,
    }
    #[c2rust::src_loc = "27:1"]
    pub type DynObject = *mut _DynObject;
    /* DO NOT ADD ANYTHING AFTER THIS #endif */
    /* _Dyn_h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/rpc/dynP.h:21"]
pub mod dynP_h {
    #[c2rust::src_loc = "38:1"]
    pub type DynObjectP = *mut _DynObject;
    /*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the private header file.
 *
 * There are no restrictions on this code; however, if you make any
 * changes, I request that you document them so that I do not get
 * credit or blame for your modifications.
 *
 * Written by Barr3y Jaspan, Student Information Processing Board (SIPB)
 * and MIT-Project Athena, 1989.
 */
    /*
 * dynP.h -- private header file included by source files for libdyn.a.
 */
    /*
 * Rep invariant:
 * 1) el_size is the number of bytes per element in the object
 * 2) num_el is the number of elements currently in the object.  It is
 * one higher than the highest index at which an element lives.
 * 3) size is the number of elements the object can hold without
 * resizing.  num_el <= index.
 * 4) inc is a multiple of the number of elements the object grows by
 * each time it is reallocated.
 */
    #[c2rust::src_loc = "38:1"]
    pub type DynObjectRecP = _DynObject;
    use super::dyn_h::_DynObject;
    /* DON'T ADD STUFF AFTER THIS #endif */
    /* _DynP_h */
}
#[c2rust::header_src = "/usr/include/stdio.h:17"]
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
#[c2rust::header_src = "/usr/include/stdlib.h:18"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:19"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::dyn_h::{DynPtr, _DynObject, DynObject};
pub use self::dynP_h::{DynObjectP, DynObjectRecP};
use self::stdio_h::{stderr, fprintf};
use self::stdlib_h::{malloc, realloc, free};
use self::string_h::{memcpy, memmove, memset};
/*
 * This file is the collected implementation of libdyn.a, the C
 * Dynamic Object library.  It contains everything.
 *
 * There are no restrictions on this code; however, if you make any
 * changes, I request that you document them so that I do not get
 * credit or blame for your modifications.
 *
 * Written by Barr3y Jaspan, Student Information Processing Board (SIPB)
 * and MIT-Project Athena, 1989.
 *
 * 2002-07-17 Collected full implementation into one source file for
 *            easy inclusion into the one library still dependent on
 *            libdyn.  Assume memmove.  Old ChangeLog appended.
 */
/* old dyn_append.c */
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the source code for the function DynAppend().
 */
/*
 * Made obsolete by DynInsert, now just a convenience function.
 */
#[no_mangle]
#[c2rust::src_loc = "33:1"]
pub unsafe extern "C" fn gssrpcint_DynAppend(mut obj: DynObjectP,
                                             mut els: DynPtr,
                                             mut num: libc::c_int)
 -> libc::c_int {
    return gssrpcint_DynInsert(obj, gssrpcint_DynSize(obj),
                               els as *mut libc::c_void, num);
}
#[c2rust::src_loc = "53:12"]
static mut default_increment: libc::c_int = 100 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn gssrpcint_DynCreate(mut el_size: libc::c_int,
                                             mut inc: libc::c_int)
 -> DynObject {
    let mut obj: DynObjectP = 0 as *mut _DynObject;
    obj =
        malloc(::std::mem::size_of::<DynObjectRecP>() as libc::c_ulong) as
            DynObjectP;
    if obj.is_null() { return 0 as DynObject }
    (*obj).array = malloc(1 as libc::c_int as libc::c_ulong) as DynPtr;
    if (*obj).array.is_null() {
        free(obj as *mut libc::c_void);
        return 0 as DynObject
    }
    *(*obj).array.offset(0 as libc::c_int as isize) =
        '\u{0}' as i32 as libc::c_char;
    (*obj).el_size = el_size;
    (*obj).size = 0 as libc::c_int;
    (*obj).num_el = (*obj).size;
    (*obj).paranoid = 0 as libc::c_int;
    (*obj).debug = (*obj).paranoid;
    (*obj).inc = if inc != 0 { inc } else { default_increment };
    (*obj).initzero = 0 as libc::c_int;
    return obj;
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn gssrpcint_DynCopy(mut obj: DynObjectP) -> DynObject {
    let mut obj1: DynObjectP = 0 as *mut _DynObject;
    obj1 =
        malloc(::std::mem::size_of::<DynObjectRecP>() as libc::c_ulong) as
            DynObjectP;
    if obj1.is_null() { return 0 as DynObject }
    (*obj1).el_size = (*obj).el_size;
    (*obj1).num_el = (*obj).num_el;
    (*obj1).size = (*obj).size;
    (*obj1).inc = (*obj).inc;
    (*obj1).debug = (*obj).debug;
    (*obj1).paranoid = (*obj).paranoid;
    (*obj1).initzero = (*obj).initzero;
    (*obj1).array =
        malloc(((*obj1).el_size * (*obj1).size) as size_t) as
            *mut libc::c_char;
    if (*obj1).array.is_null() {
        free(obj1 as *mut libc::c_void);
        return 0 as DynObject
    }
    memcpy((*obj1).array as *mut libc::c_void,
           (*obj).array as *const libc::c_void,
           ((*obj1).el_size * (*obj1).size) as size_t);
    return obj1;
}
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub unsafe extern "C" fn gssrpcint_DynDestroy(mut obj: DynObjectP)
 -> libc::c_int {
    if (*obj).paranoid != 0 {
        if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: destroy: zeroing %d bytes from %p.\n\x00" as
                        *const u8 as *const libc::c_char,
                    (*obj).el_size * (*obj).size, (*obj).array);
        }
        memset((*obj).array as *mut libc::c_void, 0 as libc::c_int,
               ((*obj).el_size * (*obj).size) as size_t);
    }
    free((*obj).array as *mut libc::c_void);
    free(obj as *mut libc::c_void);
    return -(1000 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "121:1"]
pub unsafe extern "C" fn gssrpcint_DynRelease(mut obj: DynObjectP)
 -> libc::c_int {
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: release: freeing object structure.\n\x00" as *const u8
                    as *const libc::c_char);
    }
    free(obj as *mut libc::c_void);
    return -(1000 as libc::c_int);
}
/* old dyn_debug.c */
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the source code for the function DynDebug().
 */
#[no_mangle]
#[c2rust::src_loc = "137:1"]
pub unsafe extern "C" fn gssrpcint_DynDebug(mut obj: DynObjectP,
                                            mut state: libc::c_int)
 -> libc::c_int {
    (*obj).debug = state;
    fprintf(stderr,
            b"dyn: debug: Debug state set to %d.\n\x00" as *const u8 as
                *const libc::c_char, state);
    return -(1000 as libc::c_int);
}
/* old dyn_delete.c */
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the source code for the function DynDelete().
 */
/*
 * Checkers!  Get away from that "hard disk erase" button!
 *    (Stupid dog.  He almost did it to me again ...)
 */
#[no_mangle]
#[c2rust::src_loc = "158:1"]
pub unsafe extern "C" fn gssrpcint_DynDelete(mut obj: DynObjectP,
                                             mut idx: libc::c_int)
 -> libc::c_int {
    if idx < 0 as libc::c_int {
        if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: delete: bad index %d\n\x00" as *const u8 as
                        *const libc::c_char, idx);
        }
        return -(1002 as libc::c_int)
    }
    if idx >= (*obj).num_el {
        if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: delete: Highest index is %d.\n\x00" as *const u8 as
                        *const libc::c_char, (*obj).num_el);
        }
        return -(1002 as libc::c_int)
    }
    if idx == (*obj).num_el - 1 as libc::c_int {
        if (*obj).paranoid != 0 {
            if (*obj).debug != 0 {
                fprintf(stderr,
                        b"dyn: delete: last element, zeroing.\n\x00" as
                            *const u8 as *const libc::c_char);
            }
            memset((*obj).array.offset((idx * (*obj).el_size) as isize) as
                       *mut libc::c_void, 0 as libc::c_int,
                   (*obj).el_size as size_t);
        } else if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: delete: last element, punting.\n\x00" as *const u8
                        as *const libc::c_char);
        }
    } else {
        if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: delete: copying %d bytes from %p + %d to + %d.\n\x00"
                        as *const u8 as *const libc::c_char,
                    (*obj).el_size * ((*obj).num_el - idx), (*obj).array,
                    (idx + 1 as libc::c_int) * (*obj).el_size,
                    idx * (*obj).el_size);
        }
        memmove((*obj).array.offset((idx * (*obj).el_size) as isize) as
                    *mut libc::c_void,
                (*obj).array.offset(((idx + 1 as libc::c_int) *
                                         (*obj).el_size) as isize) as
                    *const libc::c_void,
                ((*obj).el_size as
                     size_t).wrapping_mul(((*obj).num_el - idx) as
                                              libc::c_ulong));
        if (*obj).paranoid != 0 {
            if (*obj).debug != 0 {
                fprintf(stderr,
                        b"dyn: delete: zeroing %d bytes from %p + %d\n\x00" as
                            *const u8 as *const libc::c_char, (*obj).el_size,
                        (*obj).array,
                        (*obj).el_size * ((*obj).num_el - 1 as libc::c_int));
            }
            memset((*obj).array.offset(((*obj).el_size *
                                            ((*obj).num_el -
                                                 1 as libc::c_int)) as isize)
                       as *mut libc::c_void, 0 as libc::c_int,
                   (*obj).el_size as size_t);
        }
    }
    (*obj).num_el -= 1;
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: delete: done.\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    return -(1000 as libc::c_int);
}
/* old dyn_initzero.c */
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the source code for the function DynInitZero().
 */
#[no_mangle]
#[c2rust::src_loc = "222:1"]
pub unsafe extern "C" fn gssrpcint_DynInitzero(mut obj: DynObjectP,
                                               mut state: libc::c_int)
 -> libc::c_int {
    (*obj).initzero = state;
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: initzero: initzero set to %d.\n\x00" as *const u8 as
                    *const libc::c_char, state);
    }
    return -(1000 as libc::c_int);
}
/* old dyn_insert.c */
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the source code for the function DynInsert().
 */
#[no_mangle]
#[c2rust::src_loc = "240:1"]
pub unsafe extern "C" fn gssrpcint_DynInsert(mut obj: DynObjectP,
                                             mut idx: libc::c_int,
                                             mut els_in: *mut libc::c_void,
                                             mut num: libc::c_int)
 -> libc::c_int {
    let mut els: DynPtr = els_in as DynPtr;
    let mut ret: libc::c_int = 0;
    if idx < 0 as libc::c_int || idx > (*obj).num_el {
        if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: insert: index %d is not in [0,%d]\n\x00" as
                        *const u8 as *const libc::c_char, idx, (*obj).num_el);
        }
        return -(1002 as libc::c_int)
    }
    if num < 1 as libc::c_int {
        if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: insert: cannot insert %d elements\n\x00" as
                        *const u8 as *const libc::c_char, num);
        }
        return -(1003 as libc::c_int)
    }
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: insert: Moving %d bytes from %p + %d to + %d\n\x00" as
                    *const u8 as *const libc::c_char,
                ((*obj).num_el - idx) * (*obj).el_size, (*obj).array,
                (*obj).el_size * idx, (*obj).el_size * (idx + num));
    }
    ret = gssrpcint_DynResize(obj, (*obj).num_el + num);
    if ret != -(1000 as libc::c_int) { return ret }
    memmove((*obj).array.offset(((*obj).el_size * (idx + num)) as isize) as
                *mut libc::c_void,
            (*obj).array.offset(((*obj).el_size * idx) as isize) as
                *const libc::c_void,
            (((*obj).num_el - idx) * (*obj).el_size) as size_t);
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: insert: Copying %d bytes from %p to %p + %d\n\x00" as
                    *const u8 as *const libc::c_char, (*obj).el_size * num,
                els, (*obj).array, (*obj).el_size * idx);
    }
    memmove((*obj).array.offset(((*obj).el_size * idx) as isize) as
                *mut libc::c_void, els as *const libc::c_void,
            ((*obj).el_size * num) as size_t);
    (*obj).num_el += num;
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: insert: done.\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    return -(1000 as libc::c_int);
}
/* old dyn_paranoid.c */
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the source code for the function DynDebug().
 */
#[no_mangle]
#[c2rust::src_loc = "293:1"]
pub unsafe extern "C" fn gssrpcint_DynParanoid(mut obj: DynObjectP,
                                               mut state: libc::c_int)
 -> libc::c_int {
    (*obj).paranoid = state;
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: paranoid: Paranoia set to %d.\n\x00" as *const u8 as
                    *const libc::c_char, state);
    }
    return -(1000 as libc::c_int);
}
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the public header file.
 *
 * There are no restrictions on this code; however, if you make any
 * changes, I request that you document them so that I do not get
 * credit or blame for your modifications.
 *
 * Written by Barr3y Jaspan, Student Information Processing Board (SIPB)
 * and MIT-Project Athena, 1989.
 *
 * 2002-07-17 Moved here from util/dyn; for old changes see dyn.c.
 *            Added macros to rename exposed symbols.  For newer changes
 *            see ChangeLog in the current directory.
 */
/*
 * dyn.h -- header file to be included by programs linking against
 * libdyn.a.
 */
/* Function macros */
/* Return status codes */
/*@null@*/
/*@only@*/
/*@null@*/
/*@only@*/
/*@only@*/
/*@observer@*/
/*@dependent@*/
/*@null@*/
/*@observer@*/
/* old dyn_put.c */
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the source code for the functions DynGet() and DynAdd().
 */
#[no_mangle]
#[c2rust::src_loc = "311:1"]
pub unsafe extern "C" fn gssrpcint_DynArray(mut obj: DynObjectP) -> DynPtr {
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: array: returning array pointer %p.\n\x00" as *const u8
                    as *const libc::c_char, (*obj).array);
    }
    return (*obj).array;
}
#[no_mangle]
#[c2rust::src_loc = "321:1"]
pub unsafe extern "C" fn gssrpcint_DynGet(mut obj: DynObjectP,
                                          mut num: libc::c_int) -> DynPtr {
    if num < 0 as libc::c_int {
        if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: get: bad index %d\n\x00" as *const u8 as
                        *const libc::c_char, num);
        }
        return 0 as DynPtr
    }
    if num >= (*obj).num_el {
        if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: get: highest element is %d.\n\x00" as *const u8 as
                        *const libc::c_char, (*obj).num_el);
        }
        return 0 as DynPtr
    }
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: get: Returning address %p + %d.\n\x00" as *const u8 as
                    *const libc::c_char, (*obj).array, (*obj).el_size * num);
    }
    return (*obj).array.offset(((*obj).el_size * num) as isize);
}
#[no_mangle]
#[c2rust::src_loc = "345:1"]
pub unsafe extern "C" fn gssrpcint_DynAdd(mut obj: DynObjectP,
                                          mut el: *mut libc::c_void)
 -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = gssrpcint_DynPut(obj, el, (*obj).num_el);
    if ret != -(1000 as libc::c_int) { return ret }
    (*obj).num_el += 1;
    return ret;
}
/*
 * WARNING!  There is a reason this function is not documented in the
 * man page.  If DynPut used to mutate already existing elements,
 * everything will go fine.  If it is used to add new elements
 * directly, however, the state within the object (such as
 * obj->num_el) will not be updated properly and many other functions
 * in the library will lose.  Have a nice day.
 */
#[no_mangle]
#[c2rust::src_loc = "367:1"]
pub unsafe extern "C" fn gssrpcint_DynPut(mut obj: DynObjectP,
                                          mut el_in: *mut libc::c_void,
                                          mut idx: libc::c_int)
 -> libc::c_int {
    let mut el: DynPtr = el_in as DynPtr;
    let mut ret: libc::c_int = 0;
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: put: Writing %d bytes from %p to %p + %d\n\x00" as
                    *const u8 as *const libc::c_char, (*obj).el_size, el,
                (*obj).array, idx * (*obj).el_size);
    }
    ret = gssrpcint_DynResize(obj, idx);
    if ret != -(1000 as libc::c_int) { return ret }
    memmove((*obj).array.offset((idx * (*obj).el_size) as isize) as
                *mut libc::c_void, el as *const libc::c_void,
            (*obj).el_size as size_t);
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: put: done.\n\x00" as *const u8 as *const libc::c_char);
    }
    return -(1000 as libc::c_int);
}
/* old dyn_realloc.c */
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the source code for the internal function _DynRealloc().
 */
/*
 * Resize the array so that element req exists.
 */
#[no_mangle]
#[c2rust::src_loc = "400:1"]
pub unsafe extern "C" fn gssrpcint_DynResize(mut obj: DynObjectP,
                                             mut req: libc::c_int)
 -> libc::c_int {
    let mut size: libc::c_int = 0;
    if (*obj).size > req {
        return -(1000 as libc::c_int)
    } else if (*obj).inc > 0 as libc::c_int {
        return gssrpcint_DynRealloc(obj,
                                    (req - (*obj).size) / (*obj).inc +
                                        1 as libc::c_int)
    } else {
        if (*obj).size == 0 as libc::c_int {
            size = -(*obj).inc
        } else { size = (*obj).size }
        /*@-shiftsigned@*/
        while size <= req { size <<= 1 as libc::c_int }
        /*@=shiftsigned@*/
        return gssrpcint_DynRealloc(obj, size)
    };
}
/* Internal functions */
/*
 * Resize the array by num_incs units.  If obj->inc is positive, this
 * means make it obj->inc*num_incs elements larger.  If obj->inc is
 * negative, this means make the array num_incs elements long.
 *
 * Ideally, this function should not be called from outside the
 * library.  However, nothing will break if it is.
 */
#[no_mangle]
#[c2rust::src_loc = "433:1"]
pub unsafe extern "C" fn gssrpcint_DynRealloc(mut obj: DynObjectP,
                                              mut num_incs: libc::c_int)
 -> libc::c_int {
    let mut temp: DynPtr = 0 as *mut libc::c_char;
    let mut new_size_in_bytes: libc::c_int = 0;
    if (*obj).inc > 0 as libc::c_int {
        new_size_in_bytes =
            (*obj).el_size * ((*obj).size + (*obj).inc * num_incs)
    } else { new_size_in_bytes = (*obj).el_size * num_incs }
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: alloc: Increasing object by %d bytes (%d incs).\n\x00"
                    as *const u8 as *const libc::c_char,
                new_size_in_bytes - (*obj).el_size * (*obj).size, num_incs);
    }
    temp =
        realloc((*obj).array as *mut libc::c_void,
                new_size_in_bytes as size_t) as DynPtr;
    if temp.is_null() {
        if (*obj).debug != 0 {
            fprintf(stderr,
                    b"dyn: alloc: Out of memory.\n\x00" as *const u8 as
                        *const libc::c_char);
        }
        return -(1001 as libc::c_int)
    } else {
        (*obj).array = temp;
        if (*obj).inc > 0 as libc::c_int {
            (*obj).size += (*obj).inc * num_incs
        } else { (*obj).size = num_incs }
    }
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: alloc: done.\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    return -(1000 as libc::c_int);
}
/* old dyn_size.c */
/*
 * This file is part of libdyn.a, the C Dynamic Object library.  It
 * contains the source code for the function DynSize().
 */
#[no_mangle]
#[c2rust::src_loc = "478:1"]
pub unsafe extern "C" fn gssrpcint_DynSize(mut obj: DynObjectP)
 -> libc::c_int {
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: size: returning size %d.\n\x00" as *const u8 as
                    *const libc::c_char, (*obj).num_el);
    }
    return (*obj).num_el;
}
#[no_mangle]
#[c2rust::src_loc = "487:1"]
pub unsafe extern "C" fn gssrpcint_DynCapacity(mut obj: DynObjectP)
 -> libc::c_int {
    if (*obj).debug != 0 {
        fprintf(stderr,
                b"dyn: capacity: returning cap of %d.\n\x00" as *const u8 as
                    *const libc::c_char, (*obj).size);
    }
    return (*obj).size;
}
/* Old change log, as it relates to source code; build system stuff
   discarded.

2001-10-09  Ken Raeburn  <raeburn@mit.edu>

	* dyn.h, dynP.h: Make prototypes unconditional.  Don't define
	P().

2001-04-25  Ezra Peisach  <epeisach@mit.edu>

	* dyn.h: Lclint annotate functions.

	* dyn_create.c (DynCreate): Do not assume that malloc(0) is valid
	and returns a valid pointer. Fix memory leak if malloc fails.

	* dyn_realloc.c (_DynResize): Turn off warning of shifting a
	signed variable.

Thu Nov  9 15:31:31 2000  Ezra Peisach  <epeisach@mit.edu>

	* dyn_create.c (DynCopy): Arguments to memcpy were reversed. Found
 	while playing with lclint.

2000-11-09  Ezra Peisach  <epeisach@mit.edu>

	* dyn_create.c, dyn_delete.c, dyn_insert.c, dyn_put.c,
	dyn_realloc.c: Cast arguments to malloc(), realloc(), memmove() to
	size_t.

	* dynP.h: Provide full prototypes for _DynRealloc() and _DynResize().

	* dyn.h: Add prototype for DynAppend.

2000-06-29  Ezra Peisach  <epeisach@mit.edu>

	* dyn_insert.c, dyn_put.c: Include string.h for memmove prototype.

2000-06-28  Ezra Peisach  <epeisach@mit.edu>

	* dyn_create.c, dyn_delete.c, dyn_insert.c, dyn_put.c: Use %p
	format for displaying pointers.

2000-06-26  Ezra Peisach  <epeisach@mit.edu>

	* dyn_realloc.c: Remove unused variable.

Sat Dec  6 22:50:03 1997  Ezra Peisach  <epeisach@mit.edu>

	* dyn_delete.c: Include <string.h>

Mon Jul 22 21:37:52 1996  Ezra Peisach  <epeisach@mit.edu>

	* dyn.h: If __STDC__ is not defined, generate prototypes implying
		functions and not variables.

Mon Jul 22 04:20:48 1996  Marc Horowitz  <marc@mit.edu>

	* dyn_insert.c (DynInsert): what used to be #ifdef POSIX, should
 	be #ifdef HAVE_MEMMOVE
*/
