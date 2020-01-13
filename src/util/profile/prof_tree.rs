use ::libc;
use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:21"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:21"]
pub mod types_h {
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:21"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::__uint64_t;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:21"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:21"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:21"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:21"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:21"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:21"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
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
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:21"]
pub mod k5_plugin_h {
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
        /* Just those definitions which are needed by util/support/plugins.c,
   which gets compiled before util/et is built, which happens before
   we can construct krb5.h, which is included by k5-int.h.

   So, no krb5 types.  */
        /*
 * Plugins normally export fixed symbol names, but when statically
 * linking plugins, we need a different symbol name for each plugin.
 * The first argument to PLUGIN_SYMBOL_NAME acts as the
 * differentiator, and is only used for static plugin linking.
 *
 * Although this macro (and thus this header file) are used in plugins
 * whose code lies inside the krb5 tree, plugins maintained separately
 * from the krb5 tree do not need it; they can just use the fixed
 * symbol name unconditionally.
 */
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:21"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/prof_int.h:21"]
pub mod prof_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "93:8"]
    pub struct _profile_t {
        pub magic: prf_magic_t,
        pub first_file: prf_file_t,
        pub vt: *mut profile_vtable,
        pub cbdata: *mut libc::c_void,
        pub lib_handle: prf_lib_handle_t,
    }
    #[c2rust::src_loc = "86:1"]
    pub type prf_lib_handle_t = *mut _prf_lib_handle_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "80:8"]
    pub struct _prf_lib_handle_t {
        pub lock: k5_mutex_t,
        pub refcount: libc::c_int,
        pub plugin_handle: *mut plugin_file_handle,
    }
    #[c2rust::src_loc = "71:1"]
    pub type prf_file_t = *mut _prf_file_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:8"]
    pub struct _prf_file_t {
        pub magic: prf_magic_t,
        pub data: *mut _prf_data_t,
        pub next: *mut _prf_file_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct _prf_data_t {
        pub magic: prf_magic_t,
        pub lock: k5_mutex_t,
        pub root: *mut profile_node,
        pub last_stat: time_t,
        pub timestamp: time_t,
        pub frac_ts: libc::c_ulong,
        pub flags: libc::c_int,
        pub upd_serial: libc::c_int,
        pub fslen: size_t,
        pub pad: C2RustUnnamed,
        pub refcount: libc::c_int,
        pub next: *mut _prf_data_t,
        pub filespec: [libc::c_char; 15],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub union C2RustUnnamed {
        pub d: libc::c_double,
        pub p: *mut libc::c_void,
        pub ll: uint64_t,
        pub m: k5_mutex_t,
    }
    #[c2rust::src_loc = "19:1"]
    pub type prf_magic_t = libc::c_long;
    #[c2rust::src_loc = "62:1"]
    pub type prf_data_t = *mut _prf_data_t;
    use super::profile_h::profile_vtable;
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_plugin_h::plugin_file_handle;
    use super::profile_node;
    use super::time_t_h::time_t;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::com_err_h::errcode_t;
    extern "C" {
        /* prof_file.c */
        #[no_mangle]
        #[c2rust::src_loc = "221:1"]
        pub fn profile_update_file_data_locked(data: prf_data_t,
                                               ret_modspec:
                                                   *mut *mut libc::c_char)
         -> errcode_t;
    }
    /* prof_set.c -- included from profile.h */
    /* Others included from profile.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/profile.h:21"]
pub mod profile_h {
    /*
 * profile.h
 */
    /* Used by profile_init_flags(). */
    /* Allow module declaration */
    /*
 * Used by the profile iterator in prof_get.c
 */
    /* __cplusplus */
    /* path as C string */
    /* list of : separated paths, C string */
    /* path as C string */
    /* list of : separated paths, C string */
    /*
 * profile_init_vtable allows a caller to create a profile-compatible object
 * with a different back end.
 */
    /*
 * Mandatory: Look up all of the relations for names, placing the resulting
 * values in *ret_values.  If no relations exist, return PROF_NO_RELATION, or
 * PROF_NO_SECTION to indicate that one of the intermediate names does not
 * exist as a section.  The list will be freed with free_values.
 */
    /* Mandatory: Free a list of strings returned by get_values. */
    /* Optional: Release any data associated with the profile. */
    /*
 * Optional (mandatory if cleanup is defined): Generate a new cbdata pointer
 * for a copy of the profile.  If not implemented, the new profile will receive
 * the same cbdata pointer as the old one.
 */
    /*
 * Optional: Create an iterator handle.
 *
 * If flags contains PROFILE_ITER_LIST_SECTION, iterate over all of the
 * relations and sections within names.  Otherwise, iterate over the relation
 * values for names, or produce a single section result if names is a section.
 *
 * If flags contains PROFILE_ITER_SECTIONS_ONLY, produce only sections.
 *
 * If flags contains PROFILE_ITER_RELATIONS_ONLY, produce only relations.
 */
    /*
 * Optional (mandatory if iterator_create is defined): Produce the next
 * relation or section in an iteration.  If producing a section result, set
 * *ret_value to NULL.  The returned strings will be freed with free_string.
 */
    /*
 * Optional (mandatory if iterator_create is defined): Free the memory for an
 * iterator.
 */
    /* Optional (mandatory if iterator is defined): Free a string value. */
    /*
 * Optional: Determine if a profile is writable.  If not implemented, the
 * profile is never writable.
 */
    /*
 * Optional: Determine if a profile is modified in memory relative to the
 * persistent store.  If not implemented, the profile is assumed to never be
 * modified.
 */
    /*
 * Optional: Change the value of a relation, or remove it if new_value is NULL.
 * If old_value is set and the relation does not have that value, return
 * PROF_NO_RELATION.
 */
    /*
 * Optional: Rename a section to new_name, or remove the section if new_name is
 * NULL.
 */
    /*
 * Optional: Add a new relation, or a new section if new_value is NULL.  Add
 * any intermediate sections as necessary.
 */
    /*
 * Optional: Flush any pending memory updates to the persistent store.  If
 * implemented, this function will be called by profile_release as well as
 * profile_flush, so make sure it's not inefficient to flush an unmodified
 * profile.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "242:8"]
    pub struct profile_vtable {
        pub minor_ver: libc::c_int,
        pub get_values: profile_get_values_fn,
        pub free_values: profile_free_values_fn,
        pub cleanup: profile_cleanup_fn,
        pub copy: profile_copy_fn,
        pub iterator_create: profile_iterator_create_fn,
        pub iterator: profile_iterator_fn,
        pub iterator_free: profile_iterator_free_fn,
        pub free_string: profile_free_string_fn,
        pub writable: profile_writable_fn,
        pub modified: profile_modified_fn,
        pub update_relation: profile_update_relation_fn,
        pub rename_section: profile_rename_section_fn,
        pub add_relation: profile_add_relation_fn,
        pub flush: profile_flush_fn,
    }
    #[c2rust::src_loc = "239:1"]
    pub type profile_flush_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long>;
    #[c2rust::src_loc = "229:1"]
    pub type profile_add_relation_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "221:1"]
    pub type profile_rename_section_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "213:1"]
    pub type profile_update_relation_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "205:1"]
    pub type profile_modified_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_int)
                   -> libc::c_long>;
    #[c2rust::src_loc = "197:1"]
    pub type profile_writable_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_int)
                   -> libc::c_long>;
    #[c2rust::src_loc = "190:1"]
    pub type profile_free_string_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_char) -> ()>;
    #[c2rust::src_loc = "186:1"]
    pub type profile_iterator_free_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "178:1"]
    pub type profile_iterator_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut libc::c_char,
                                    _: *mut *mut libc::c_char)
                   -> libc::c_long>;
    #[c2rust::src_loc = "169:1"]
    pub type profile_iterator_create_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const *const libc::c_char,
                                    _: libc::c_int, _: *mut *mut libc::c_void)
                   -> libc::c_long>;
    #[c2rust::src_loc = "155:1"]
    pub type profile_copy_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *mut libc::c_void)
                   -> libc::c_long>;
    #[c2rust::src_loc = "147:1"]
    pub type profile_cleanup_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "143:1"]
    pub type profile_free_values_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *mut libc::c_char) -> ()>;
    #[c2rust::src_loc = "138:1"]
    pub type profile_get_values_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const *const libc::c_char,
                                    _: *mut *mut *mut libc::c_char)
                   -> libc::c_long>;
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::prof_int_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/string.h:21"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:21"]
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
#[c2rust::header_src = "/usr/include/stdio.h:21"]
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
#[c2rust::header_src = "/usr/include/assert.h:21"]
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
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint64_t, __off_t, __off64_t, __time_t};
pub use self::stdint_uintn_h::uint64_t;
pub use self::time_t_h::time_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_lock,
                            k5_mutex_unlock, k5_os_mutex_lock,
                            k5_os_mutex_unlock};
use self::k5_plugin_h::plugin_file_handle;
pub use self::com_err_h::errcode_t;
pub use self::prof_int_h::{_profile_t, prf_lib_handle_t, _prf_lib_handle_t,
                           prf_file_t, _prf_file_t, _prf_data_t,
                           C2RustUnnamed, prf_magic_t, prf_data_t,
                           profile_update_file_data_locked};
pub use self::profile_h::{profile_vtable, profile_flush_fn,
                          profile_add_relation_fn, profile_rename_section_fn,
                          profile_update_relation_fn, profile_modified_fn,
                          profile_writable_fn, profile_free_string_fn,
                          profile_iterator_free_fn, profile_iterator_fn,
                          profile_iterator_create_fn, profile_copy_fn,
                          profile_cleanup_fn, profile_free_values_fn,
                          profile_get_values_fn, profile_t};
use self::string_h::{memset, strcmp, strdup, strerror};
use self::stdlib_h::{malloc, free};
use self::stdio_h::{stderr, fprintf};
use self::assert_h::__assert_fail;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * prof_tree.c --- these routines maintain the parse tree of the
 *      config file.
 *
 * All of the details of how the tree is stored is abstracted away in
 * this file; all of the other profile routines build, access, and
 * modify the tree via the accessor functions found in this file.
 *
 * Each node may represent either a relation or a section header.
 *
 * A section header must have its value field be null, and may have one
 * or more child nodes, pointed to by first_child.
 *
 * A relation has as its value a pointer to allocated memory
 * containing a string.  Its first_child pointer must be null.
 *
 */
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
#[c2rust::src_loc = "31:8"]
pub struct profile_node {
    pub magic: errcode_t,
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub group_level: libc::c_int,
    #[bitfield(name = "final_0", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "deleted", ty = "libc::c_uint", bits = "1..=1")]
    pub final_0_deleted: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub first_child: *mut profile_node,
    pub parent: *mut profile_node,
    pub next: *mut profile_node,
    pub prev: *mut profile_node,
}
/*
 * This is a general-purpose iterator for returning all nodes that
 * match the specified name array.
 */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "394:8"]
pub struct profile_node_iterator {
    pub magic: prf_magic_t,
    pub flags: libc::c_int,
    pub names: *const *const libc::c_char,
    pub name: *const libc::c_char,
    pub file: prf_file_t,
    pub file_serial: libc::c_int,
    pub done_idx: libc::c_int,
    pub node: *mut profile_node,
    pub num: libc::c_int,
}
/*
 * Free a node, and any children
 */
#[no_mangle]
#[c2rust::src_loc = "50:1"]
pub unsafe extern "C" fn profile_free_node(mut node: *mut profile_node) {
    let mut child: *mut profile_node = 0 as *mut profile_node;
    let mut next: *mut profile_node = 0 as *mut profile_node;
    if (*node).magic != -(1429577727 as libc::c_long) { return }
    if !(*node).name.is_null() { free((*node).name as *mut libc::c_void); }
    if !(*node).value.is_null() { free((*node).value as *mut libc::c_void); }
    child = (*node).first_child;
    while !child.is_null() {
        next = (*child).next;
        profile_free_node(child);
        child = next
    }
    (*node).magic = 0 as libc::c_int as errcode_t;
    free(node as *mut libc::c_void);
}
/*
 * Create a node
 */
#[no_mangle]
#[c2rust::src_loc = "87:1"]
pub unsafe extern "C" fn profile_create_node(mut name: *const libc::c_char,
                                             mut value: *const libc::c_char,
                                             mut ret_node:
                                                 *mut *mut profile_node)
 -> errcode_t {
    let mut new: *mut profile_node = 0 as *mut profile_node;
    new =
        malloc(::std::mem::size_of::<profile_node>() as libc::c_ulong) as
            *mut profile_node;
    if new.is_null() { return 12 as libc::c_int as errcode_t }
    memset(new as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<profile_node>() as libc::c_ulong);
    /* Set magic here so profile_free_node will free memory */
    (*new).magic = -(1429577727 as libc::c_long);
    (*new).name = strdup(name);
    if (*new).name.is_null() {
        profile_free_node(new);
        return 12 as libc::c_int as errcode_t
    }
    if !value.is_null() {
        (*new).value = strdup(value);
        if (*new).value.is_null() {
            profile_free_node(new);
            return 12 as libc::c_int as errcode_t
        }
    }
    *ret_node = new;
    return 0 as libc::c_int as errcode_t;
}
/*
 * This function verifies that all of the representation invarients of
 * the profile are true.  If not, we have a programming bug somewhere,
 * probably in this file.
 */
#[no_mangle]
#[c2rust::src_loc = "120:1"]
pub unsafe extern "C" fn profile_verify_node(mut node: *mut profile_node)
 -> errcode_t {
    let mut p: *mut profile_node = 0 as *mut profile_node;
    let mut last: *mut profile_node = 0 as *mut profile_node;
    let mut retval: errcode_t = 0;
    if (*node).magic != -(1429577727 as libc::c_long) {
        return -(1429577727 as libc::c_long)
    }
    if !(*node).value.is_null() && !(*node).first_child.is_null() {
        return -(1429577723 as libc::c_long)
    }
    last = 0 as *mut profile_node;
    p = (*node).first_child;
    while !p.is_null() {
        if (*p).prev != last { return -(1429577722 as libc::c_long) }
        if !last.is_null() && (*last).next != p {
            return -(1429577722 as libc::c_long)
        }
        if (*node).group_level + 1 as libc::c_int != (*p).group_level {
            return -(1429577721 as libc::c_long)
        }
        if (*p).parent != node { return -(1429577720 as libc::c_long) }
        retval = profile_verify_node(p);
        if retval != 0 { return retval }
        last = p;
        p = (*p).next
    }
    return 0 as libc::c_int as errcode_t;
}
/*
 * Add a node to a particular section
 */
#[no_mangle]
#[c2rust::src_loc = "150:1"]
pub unsafe extern "C" fn profile_add_node(mut section: *mut profile_node,
                                          mut name: *const libc::c_char,
                                          mut value: *const libc::c_char,
                                          mut ret_node:
                                              *mut *mut profile_node)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    let mut p: *mut profile_node = 0 as *mut profile_node;
    let mut last: *mut profile_node = 0 as *mut profile_node;
    let mut new: *mut profile_node = 0 as *mut profile_node;
    if (*section).magic != -(1429577727 as libc::c_long) {
        return -(1429577727 as libc::c_long)
    }
    if !(*section).value.is_null() { return -(1429577724 as libc::c_long) }
    /*
     * Find the place to insert the new node.  If we are adding a subsection
     * and already have a subsection with that name, merge them.  Otherwise,
     * we look for the place *after* the last match of the node name, since
     * order matters.
     */
    p = (*section).first_child;
    last = 0 as *mut profile_node;
    while !p.is_null() {
        let mut cmp: libc::c_int = 0;
        cmp = strcmp((*p).name, name);
        if cmp > 0 as libc::c_int { break ; }
        if value.is_null() && cmp == 0 as libc::c_int && (*p).value.is_null()
               && (*p).deleted() as libc::c_int != 1 as libc::c_int {
            /* Found duplicate subsection, so don't make a new one. */
            *ret_node = p;
            return 0 as libc::c_int as errcode_t
        }
        last = p;
        p = (*p).next
    }
    retval = profile_create_node(name, value, &mut new);
    if retval != 0 { return retval }
    (*new).group_level = (*section).group_level + 1 as libc::c_int;
    (*new).set_deleted(0 as libc::c_int as libc::c_uint);
    (*new).parent = section;
    (*new).prev = last;
    (*new).next = p;
    if !p.is_null() { (*p).prev = new }
    if !last.is_null() {
        (*last).next = new
    } else { (*section).first_child = new }
    if !ret_node.is_null() { *ret_node = new }
    return 0 as libc::c_int as errcode_t;
}
/*
 * Set the final flag on a particular node.
 */
#[no_mangle]
#[c2rust::src_loc = "201:1"]
pub unsafe extern "C" fn profile_make_node_final(mut node: *mut profile_node)
 -> errcode_t {
    if (*node).magic != -(1429577727 as libc::c_long) {
        return -(1429577727 as libc::c_long)
    }
    (*node).set_final_0(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int as errcode_t;
}
/*
 * Check the final flag on a node
 */
#[no_mangle]
#[c2rust::src_loc = "212:1"]
pub unsafe extern "C" fn profile_is_node_final(mut node: *mut profile_node)
 -> libc::c_int {
    return ((*node).final_0() as libc::c_int != 0 as libc::c_int) as
               libc::c_int;
}
/*
 * Return the name of a node.  (Note: this is for internal functions
 * only; if the name needs to be returned from an exported function,
 * strdup it first!)
 */
#[no_mangle]
#[c2rust::src_loc = "222:1"]
pub unsafe extern "C" fn profile_get_node_name(mut node: *mut profile_node)
 -> *const libc::c_char {
    return (*node).name;
}
/*
 * Return the value of a node.  (Note: this is for internal functions
 * only; if the name needs to be returned from an exported function,
 * strdup it first!)
 */
#[no_mangle]
#[c2rust::src_loc = "232:1"]
pub unsafe extern "C" fn profile_get_node_value(mut node: *mut profile_node)
 -> *const libc::c_char {
    return (*node).value;
}
/*
 * Iterate through the section, returning the nodes which match
 * the given name.  If name is NULL, then interate through all the
 * nodes in the section.  If section_flag is non-zero, only return the
 * section which matches the name; don't return relations.  If value
 * is non-NULL, then only return relations which match the requested
 * value.  (The value argument is ignored if section_flag is non-zero.)
 *
 * The first time this routine is called, the state pointer must be
 * null.  When this profile_find_node_relation() returns, if the state
 * pointer is non-NULL, then this routine should be called again.
 * (This won't happen if section_flag is non-zero, obviously.)
 *
 */
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn profile_find_node(mut section: *mut profile_node,
                                           mut name: *const libc::c_char,
                                           mut value: *const libc::c_char,
                                           mut section_flag: libc::c_int,
                                           mut state: *mut *mut libc::c_void,
                                           mut node: *mut *mut profile_node)
 -> errcode_t {
    let mut p: *mut profile_node = 0 as *mut profile_node;
    if (*section).magic != -(1429577727 as libc::c_long) {
        return -(1429577727 as libc::c_long)
    }
    p = *state as *mut profile_node;
    if !p.is_null() {
        if (*p).magic != -(1429577727 as libc::c_long) {
            return -(1429577727 as libc::c_long)
        }
    } else { p = (*section).first_child }
    let mut current_block_11: u64;
    while !p.is_null() {
        if !(!name.is_null() && strcmp((*p).name, name) != 0) {
            if section_flag != 0 {
                if !(*p).value.is_null() {
                    current_block_11 = 3640593987805443782;
                } else { current_block_11 = 15652330335145281839; }
            } else if (*p).value.is_null() {
                current_block_11 = 3640593987805443782;
            } else if !value.is_null() && strcmp((*p).value, value) != 0 {
                current_block_11 = 3640593987805443782;
            } else { current_block_11 = 15652330335145281839; }
            match current_block_11 {
                3640593987805443782 => { }
                _ => {
                    if !((*p).deleted() != 0) {
                        /* A match! */
                        if !node.is_null() { *node = p }
                        break ;
                    }
                }
            }
        }
        p = (*p).next
    }
    if p.is_null() {
        *state = 0 as *mut libc::c_void;
        return if section_flag != 0 {
                   -(1429577726 as libc::c_long)
               } else { -(1429577725 as libc::c_long) }
    }
    let mut current_block_17: u64;
    /*
     * OK, we've found one match; now let's try to find another
     * one.  This way, if we return a non-zero state pointer,
     * there's guaranteed to be another match that's returned.
     */
    p = (*p).next;
    while !p.is_null() {
        if !(!name.is_null() && strcmp((*p).name, name) != 0) {
            if section_flag != 0 {
                if !(*p).value.is_null() {
                    current_block_17 = 18386322304582297246;
                } else { current_block_17 = 14434620278749266018; }
            } else if (*p).value.is_null() {
                current_block_17 = 18386322304582297246;
            } else if !value.is_null() && strcmp((*p).value, value) != 0 {
                current_block_17 = 18386322304582297246;
            } else { current_block_17 = 14434620278749266018; }
            match current_block_17 {
                18386322304582297246 => { }
                _ => { if !((*p).deleted() != 0) { break ; } }
            }
        }
        p = (*p).next
    }
    *state = p as *mut libc::c_void;
    return 0 as libc::c_int as errcode_t;
}
/*
 * Iterate through the section, returning the relations which match
 * the given name.  If name is NULL, then interate through all the
 * relations in the section.  The first time this routine is called,
 * the state pointer must be null.  When this profile_find_node_relation()
 * returns, if the state pointer is non-NULL, then this routine should
 * be called again.
 *
 * The returned character string in value points to the stored
 * character string in the parse string.  Before this string value is
 * returned to a calling application (profile_find_node_relation is not an
 * exported interface), it should be strdup()'ed.
 */
#[no_mangle]
#[c2rust::src_loc = "327:1"]
pub unsafe extern "C" fn profile_find_node_relation(mut section:
                                                        *mut profile_node,
                                                    mut name:
                                                        *const libc::c_char,
                                                    mut state:
                                                        *mut *mut libc::c_void,
                                                    mut ret_name:
                                                        *mut *mut libc::c_char,
                                                    mut value:
                                                        *mut *mut libc::c_char)
 -> errcode_t {
    let mut p: *mut profile_node = 0 as *mut profile_node;
    let mut retval: errcode_t = 0;
    retval =
        profile_find_node(section, name, 0 as *const libc::c_char,
                          0 as libc::c_int, state, &mut p);
    if retval != 0 { return retval }
    if !p.is_null() {
        if !value.is_null() { *value = (*p).value }
        if !ret_name.is_null() { *ret_name = (*p).name }
    }
    return 0 as libc::c_int as errcode_t;
}
/*
 * Iterate through the section, returning the subsections which match
 * the given name.  If name is NULL, then interate through all the
 * subsections in the section.  The first time this routine is called,
 * the state pointer must be null.  When this profile_find_node_subsection()
 * returns, if the state pointer is non-NULL, then this routine should
 * be called again.
 *
 * This is (plus accessor functions for the name and value given a
 * profile node) makes this function mostly syntactic sugar for
 * profile_find_node.
 */
#[no_mangle]
#[c2rust::src_loc = "359:1"]
pub unsafe extern "C" fn profile_find_node_subsection(mut section:
                                                          *mut profile_node,
                                                      mut name:
                                                          *const libc::c_char,
                                                      mut state:
                                                          *mut *mut libc::c_void,
                                                      mut ret_name:
                                                          *mut *mut libc::c_char,
                                                      mut subsection:
                                                          *mut *mut profile_node)
 -> errcode_t {
    let mut p: *mut profile_node = 0 as *mut profile_node;
    let mut retval: errcode_t = 0;
    retval =
        profile_find_node(section, name, 0 as *const libc::c_char,
                          1 as libc::c_int, state, &mut p);
    if retval != 0 { return retval }
    if !p.is_null() {
        if !subsection.is_null() { *subsection = p }
        if !ret_name.is_null() { *ret_name = (*p).name }
    }
    return 0 as libc::c_int as errcode_t;
}
/*
 * This function returns the parent of a particular node.
 */
#[no_mangle]
#[c2rust::src_loc = "383:1"]
pub unsafe extern "C" fn profile_get_node_parent(mut section:
                                                     *mut profile_node,
                                                 mut parent:
                                                     *mut *mut profile_node)
 -> errcode_t {
    *parent = (*section).parent;
    return 0 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "406:1"]
pub unsafe extern "C" fn profile_node_iterator_create(mut profile: profile_t,
                                                      mut names:
                                                          *const *const libc::c_char,
                                                      mut flags: libc::c_int,
                                                      mut ret_iter:
                                                          *mut *mut libc::c_void)
 -> errcode_t {
    let mut iter: *mut profile_node_iterator =
        0 as *mut profile_node_iterator;
    let mut done_idx: libc::c_int = 0 as libc::c_int;
    if profile.is_null() { return -(1429577704 as libc::c_long) }
    if (*profile).magic != -(1429577710 as libc::c_long) {
        return -(1429577710 as libc::c_long)
    }
    if names.is_null() { return -(1429577705 as libc::c_long) }
    if flags & 0x1 as libc::c_int == 0 {
        if (*names.offset(0 as libc::c_int as isize)).is_null() {
            return -(1429577705 as libc::c_long)
        }
        done_idx = 1 as libc::c_int
    }
    iter =
        malloc(::std::mem::size_of::<profile_node_iterator>() as
                   libc::c_ulong) as *mut profile_node_iterator;
    if iter.is_null() { return 12 as libc::c_int as errcode_t }
    (*iter).magic = -(1429577694 as libc::c_long);
    (*iter).names = names;
    (*iter).flags = flags;
    (*iter).file = (*profile).first_file;
    (*iter).done_idx = done_idx;
    (*iter).node = 0 as *mut profile_node;
    (*iter).num = 0 as libc::c_int;
    *ret_iter = iter as *mut libc::c_void;
    return 0 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "440:1"]
pub unsafe extern "C" fn profile_node_iterator_free(mut iter_p:
                                                        *mut *mut libc::c_void) {
    let mut iter: *mut profile_node_iterator =
        0 as *mut profile_node_iterator;
    if iter_p.is_null() { return }
    iter = *iter_p as *mut profile_node_iterator;
    if iter.is_null() || (*iter).magic != -(1429577694 as libc::c_long) {
        return
    }
    free(iter as *mut libc::c_void);
    *iter_p = 0 as *mut libc::c_void;
}
/*
 * Note: the returned character strings in ret_name and ret_value
 * points to the stored character string in the parse string.  Before
 * this string value is returned to a calling application
 * (profile_node_iterator is not an exported interface), it should be
 * strdup()'ed.
 */
#[no_mangle]
#[c2rust::src_loc = "460:1"]
pub unsafe extern "C" fn profile_node_iterator(mut iter_p:
                                                   *mut *mut libc::c_void,
                                               mut ret_node:
                                                   *mut *mut profile_node,
                                               mut ret_name:
                                                   *mut *mut libc::c_char,
                                               mut ret_value:
                                                   *mut *mut libc::c_char)
 -> errcode_t {
    let mut iter: *mut profile_node_iterator =
        *iter_p as *mut profile_node_iterator;
    let mut section: *mut profile_node = 0 as *mut profile_node;
    let mut p: *mut profile_node = 0 as *mut profile_node;
    let mut cpp: *const *const libc::c_char = 0 as *const *const libc::c_char;
    let mut retval: errcode_t = 0;
    let mut skip_num: libc::c_int = 0 as libc::c_int;
    if iter.is_null() || (*iter).magic != -(1429577694 as libc::c_long) {
        return -(1429577694 as libc::c_long)
    }
    if !(*iter).file.is_null() &&
           (*(*iter).file).magic != -(1429577703 as libc::c_long) {
        return -(1429577703 as libc::c_long)
    }
    if !(*iter).file.is_null() &&
           (*(*(*iter).file).data).magic != -(1429577698 as libc::c_long) {
        return -(1429577698 as libc::c_long)
    }
    /*
     * If the file has changed, then the node pointer is invalid,
     * so we'll have search the file again looking for it.
     */
    if !(*iter).file.is_null() {
        k5_mutex_lock(&mut (*(*(*iter).file).data).lock);
    }
    if !(*iter).node.is_null() &&
           (*(*(*iter).file).data).upd_serial != (*iter).file_serial {
        (*iter).flags &= !(0x100 as libc::c_int);
        skip_num = (*iter).num;
        (*iter).node = 0 as *mut profile_node
    }
    if !(*iter).node.is_null() &&
           (*(*iter).node).magic != -(1429577727 as libc::c_long) {
        if !(*iter).file.is_null() {
            k5_mutex_unlock(&mut (*(*(*iter).file).data).lock);
        }
        return -(1429577727 as libc::c_long)
    }
    loop  {
        if (*iter).node.is_null() {
            if (*iter).file.is_null() ||
                   (*iter).flags & 0x100 as libc::c_int != 0 {
                if !(*iter).file.is_null() {
                    k5_mutex_unlock(&mut (*(*(*iter).file).data).lock);
                }
                profile_node_iterator_free(iter_p);
                if !ret_node.is_null() { *ret_node = 0 as *mut profile_node }
                if !ret_name.is_null() { *ret_name = 0 as *mut libc::c_char }
                if !ret_value.is_null() {
                    *ret_value = 0 as *mut libc::c_char
                }
                return 0 as libc::c_int as errcode_t
            }
            retval =
                profile_update_file_data_locked((*(*iter).file).data,
                                                0 as *mut *mut libc::c_char);
            if retval != 0 {
                k5_mutex_unlock(&mut (*(*(*iter).file).data).lock);
                if retval == 2 as libc::c_int as libc::c_long ||
                       retval == 13 as libc::c_int as libc::c_long {
                    /* XXX memory leak? */
                    (*iter).file = (*(*iter).file).next;
                    if !(*iter).file.is_null() {
                        k5_mutex_lock(&mut (*(*(*iter).file).data).lock);
                    }
                    skip_num = 0 as libc::c_int;
                    retval = 0 as libc::c_int as errcode_t;
                    continue ;
                } else { profile_node_iterator_free(iter_p); return retval }
            } else {
                (*iter).file_serial = (*(*(*iter).file).data).upd_serial;
                /*
         * Find the section to list if we are a LIST_SECTION,
         * or find the containing section if not.
         */
                section = (*(*(*iter).file).data).root;
                if !section.is_null() {
                } else {
                    __assert_fail(b"section != NULL\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"prof_tree.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  528 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 83],
                                                            &[libc::c_char; 83]>(b"errcode_t profile_node_iterator(void **, struct profile_node **, char **, char **)\x00")).as_ptr());
                }
                cpp = (*iter).names;
                while !(*cpp.offset((*iter).done_idx as isize)).is_null() {
                    p = (*section).first_child;
                    while !p.is_null() {
                        if strcmp((*p).name, *cpp) == 0 &&
                               (*p).value.is_null() && (*p).deleted() == 0 {
                            break ;
                        }
                        p = (*p).next
                    }
                    if p.is_null() {
                        section = 0 as *mut profile_node;
                        break ;
                    } else {
                        section = p;
                        if (*p).final_0() != 0 {
                            (*iter).flags |= 0x100 as libc::c_int
                        }
                        cpp = cpp.offset(1)
                    }
                }
                if section.is_null() {
                    k5_mutex_unlock(&mut (*(*(*iter).file).data).lock);
                    (*iter).file = (*(*iter).file).next;
                    if !(*iter).file.is_null() {
                        k5_mutex_lock(&mut (*(*(*iter).file).data).lock);
                    }
                    skip_num = 0 as libc::c_int;
                    continue ;
                } else {
                    (*iter).name = *cpp;
                    (*iter).node = (*section).first_child
                }
            }
        }
        /*
     * OK, now we know iter->node is set up correctly.  Let's do
     * the search.
     */
        p = (*iter).node;
        while !p.is_null() {
            if !(!(*iter).name.is_null() &&
                     strcmp((*p).name, (*iter).name) != 0) {
                if !((*iter).flags & 0x2 as libc::c_int != 0 &&
                         !(*p).value.is_null()) {
                    if !((*iter).flags & 0x4 as libc::c_int != 0 &&
                             (*p).value.is_null()) {
                        if skip_num > 0 as libc::c_int {
                            skip_num -= 1
                        } else if !((*p).deleted() != 0) { break ; }
                    }
                }
            }
            p = (*p).next
        }
        (*iter).num += 1;
        if p.is_null() {
            k5_mutex_unlock(&mut (*(*(*iter).file).data).lock);
            (*iter).file = (*(*iter).file).next;
            if !(*iter).file.is_null() {
                k5_mutex_lock(&mut (*(*(*iter).file).data).lock);
            }
            (*iter).node = 0 as *mut profile_node;
            skip_num = 0 as libc::c_int
        } else {
            k5_mutex_unlock(&mut (*(*(*iter).file).data).lock);
            (*iter).node = (*p).next;
            if (*iter).node.is_null() { (*iter).file = (*(*iter).file).next }
            if !ret_node.is_null() { *ret_node = p }
            if !ret_name.is_null() { *ret_name = (*p).name }
            if !ret_value.is_null() { *ret_value = (*p).value }
            return 0 as libc::c_int as errcode_t
        }
    };
}
/*
 * Remove a particular node.
 *
 * TYT, 2/25/99
 */
#[no_mangle]
#[c2rust::src_loc = "601:1"]
pub unsafe extern "C" fn profile_remove_node(mut node: *mut profile_node)
 -> errcode_t {
    if (*node).magic != -(1429577727 as libc::c_long) {
        return -(1429577727 as libc::c_long)
    } /* Can't remove the root! */
    if (*node).parent.is_null() { return -(1429577717 as libc::c_long) }
    (*node).set_deleted(1 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int as errcode_t;
}
/*
 * Set the value of a specific node containing a relation.
 *
 * TYT, 2/25/99
 */
#[no_mangle]
#[c2rust::src_loc = "618:1"]
pub unsafe extern "C" fn profile_set_relation_value(mut node:
                                                        *mut profile_node,
                                                    mut new_value:
                                                        *const libc::c_char)
 -> errcode_t {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*node).magic != -(1429577727 as libc::c_long) {
        return -(1429577727 as libc::c_long)
    }
    if (*node).value.is_null() { return -(1429577718 as libc::c_long) }
    cp = strdup(new_value);
    if cp.is_null() { return 12 as libc::c_int as errcode_t }
    free((*node).value as *mut libc::c_void);
    (*node).value = cp;
    return 0 as libc::c_int as errcode_t;
}
/*
 * prof-int.h
 */
/*
 * This is the structure which stores the profile information for a
 * particular configuration file.
 *
 * Locking strategy:
 * - filespec, fslen are fixed after creation
 * - refcount and next should only be tweaked with the global lock held
 * - other fields can be tweaked after grabbing the in-struct lock
 */
/* time tree was last updated from file */
/* fractional part of timestamp, if any */
/* r/w, dirty */
/* incremented when data changes */
/* Some separation between fields controlled by different
	   mutexes.  Theoretically, both could be accessed at the same
	   time from different threads on different CPUs with separate
	   caches.  Don't let the threads clobber each other's
	   changes.  One mutex controlling the whole thing would be
	   better, but sufficient separation might suffice.

	   This is icky.  I just hope it's adequate.

	   For next major release, fix this.  */
/* prf_file_t references */
/* Was: "profile_filespec_t filespec".  Now: flexible char
	   array ... except, we need to work in C89, so an array
	   length must be specified.  */
/*
 * The profile flags
 */
/*
 * This structure defines the high-level, user visible profile_t
 * object, which is used as a handle by users who need to query some
 * configuration file(s)
 */
/* If non-null, use vtable operations instead of native ones. */
/*
 * Used by the profile iterator in prof_get.c
 */
/*
 * Check if a filespec is last in a list (NULL on UNIX, invalid FSSpec on MacOS
 */
/* profile_parse.c */
/* prof_tree.c */
/*
 * Rename a specific node
 *
 * TYT 2/25/99
 */
#[no_mangle]
#[c2rust::src_loc = "643:1"]
pub unsafe extern "C" fn profile_rename_node(mut node: *mut profile_node,
                                             mut new_name:
                                                 *const libc::c_char)
 -> errcode_t {
    let mut new_string: *mut libc::c_char =
        0 as *mut libc::c_char; /* It's the same name, return */
    let mut p: *mut profile_node = 0 as *mut profile_node;
    let mut last: *mut profile_node = 0 as *mut profile_node;
    if (*node).magic != -(1429577727 as libc::c_long) {
        return -(1429577727 as libc::c_long)
    }
    if strcmp(new_name, (*node).name) == 0 as libc::c_int {
        return 0 as libc::c_int as errcode_t
    }
    /*
     * Make sure we can allocate memory for the new name, first!
     */
    new_string = strdup(new_name);
    if new_string.is_null() { return 12 as libc::c_int as errcode_t }
    /*
     * Find the place to where the new node should go.  We look
     * for the place *after* the last match of the node name,
     * since order matters.
     */
    p = (*(*node).parent).first_child;
    last = 0 as *mut profile_node;
    while !p.is_null() {
        if strcmp((*p).name, new_name) > 0 as libc::c_int { break ; }
        last = p;
        p = (*p).next
    }
    /*
     * If we need to move the node, do it now.
     */
    if p != node && last != node {
        /*
         * OK, let's detach the node
         */
        if !(*node).prev.is_null() {
            (*(*node).prev).next = (*node).next
        } else { (*(*node).parent).first_child = (*node).next }
        if !(*node).next.is_null() { (*(*node).next).prev = (*node).prev }
        /*
         * Now let's reattach it in the right place.
         */
        if !p.is_null() { (*p).prev = node }
        if !last.is_null() {
            (*last).next = node
        } else { (*(*node).parent).first_child = node }
        (*node).next = p;
        (*node).prev = last
    }
    free((*node).name as *mut libc::c_void);
    (*node).name = new_string;
    return 0 as libc::c_int as errcode_t;
}
