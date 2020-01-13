use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/stdlib.h:33"]
pub mod stdlib_h {
    #[c2rust::src_loc = "808:1"]
    pub type __compar_fn_t
        =
        Option<unsafe extern "C" fn(_: *const libc::c_void,
                                    _: *const libc::c_void) -> libc::c_int>;
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "830:1"]
        pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t,
                     __size: size_t, __compar: __compar_fn_t);
    }
}
#[c2rust::header_src = "/usr/include/bits/dirent.h:105"]
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:8"]
    pub struct dirent {
        pub d_ino: __ino_t,
        pub d_off: __off_t,
        pub d_reclen: libc::c_ushort,
        pub d_type: libc::c_uchar,
        pub d_name: [libc::c_char; 256],
    }
    use super::types_h::{__ino_t, __off_t};
}
#[c2rust::header_src = "/usr/include/dirent.h:105"]
pub mod include_dirent_h {
    #[c2rust::src_loc = "127:1"]
    pub type DIR = __dirstream;
    use super::dirent_h::dirent;
    extern "C" {
        #[c2rust::src_loc = "127:16"]
        pub type __dirstream;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn closedir(__dirp: *mut DIR) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "162:1"]
        pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn opendir(__name: *const libc::c_char) -> *mut DIR;
    }
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__ino_t, __off_t};
pub use self::stdlib_h::{__compar_fn_t, realloc, free, qsort};
pub use self::dirent_h::dirent;
pub use self::include_dirent_h::{DIR, __dirstream, closedir, readdir,
                                 opendir};
use self::string_h::{strdup, strcmp};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* util/support/dir_filenames.c - fetch filenames in a directory */
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
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn k5_free_filenames(mut fnames:
                                               *mut *mut libc::c_char) {
    let mut fn_0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    fn_0 = fnames;
    while !fn_0.is_null() && !(*fn_0).is_null() {
        free(*fn_0 as *mut libc::c_void);
        fn_0 = fn_0.offset(1)
    }
    free(fnames as *mut libc::c_void);
}
/* Resize the filename list and add a name. */
#[c2rust::src_loc = "46:1"]
unsafe extern "C" fn add_filename(mut fnames: *mut *mut *mut libc::c_char,
                                  mut n_fnames: *mut libc::c_int,
                                  mut name: *const libc::c_char)
 -> libc::c_int {
    let mut newlist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    newlist =
        realloc(*fnames as *mut libc::c_void,
                ((*n_fnames + 2 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                     as libc::c_ulong)) as
            *mut *mut libc::c_char;
    if newlist.is_null() { return 12 as libc::c_int }
    *fnames = newlist;
    let ref mut fresh0 = *newlist.offset(*n_fnames as isize);
    *fresh0 = strdup(name);
    if (*newlist.offset(*n_fnames as isize)).is_null() {
        return 12 as libc::c_int
    }
    *n_fnames += 1;
    let ref mut fresh1 = *newlist.offset(*n_fnames as isize);
    *fresh1 = 0 as *mut libc::c_char;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn compare_with_strcmp(mut a: *const libc::c_void,
                                         mut b: *const libc::c_void)
 -> libc::c_int {
    return strcmp(*(a as *mut *mut libc::c_char),
                  *(b as *mut *mut libc::c_char));
}
/* -*- mode: c; indent-tabs-mode: nil -*- */
/* include/k5-platform.h */
/*
 * Copyright 2003, 2004, 2005, 2007, 2008, 2009 Massachusetts Institute of Technology.
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
 */
/*
 * Some platform-dependent definitions to sync up the C support level.
 * Some to a C99-ish level, some related utility code.
 *
 * Currently:
 * + [u]int{8,16,32}_t types
 * + 64-bit types and load/store code
 * + SIZE_MAX
 * + shared library init/fini hooks
 * + consistent getpwnam/getpwuid interfaces
 * + va_copy fudged if not provided
 * + strlcpy/strlcat
 * + fnmatch
 * + [v]asprintf
 * + strerror_r
 * + mkstemp
 * + zap (support function and macro)
 * + constant time memory comparison
 * + path manipulation
 * + _, N_, dgettext, bindtextdomain (for localization)
 * + getopt_long
 * + secure_getenv
 * + fetching filenames from a directory
 */
/* This attribute prevents unused function warnings in gcc and clang. */
/* Initialization and finalization function support for libraries.

   At top level, before the functions are defined or even declared:
   MAKE_INIT_FUNCTION(init_fn);
   MAKE_FINI_FUNCTION(fini_fn);
   Then:
   int init_fn(void) { ... }
   void fini_fn(void) { if (INITIALIZER_RAN(init_fn)) ... }
   In code, in the same file:
   err = CALL_INIT_FUNCTION(init_fn);

   To trigger or verify the initializer invocation from another file,
   a helper function must be created.

   This model handles both the load-time execution (Windows) and
   delayed execution (pthread_once) approaches, and should be able to
   guarantee in both cases that the init function is run once, in one
   thread, before other stuff in the library is done; furthermore, the
   finalization code should only run if the initialization code did.
   (Maybe I could've made the "if INITIALIZER_RAN" test implicit, via
   another function hidden in macros, but this is hairy enough
   already.)

   The init_fn and fini_fn names should be chosen such that any
   exported names staring with those names, and optionally followed by
   additional characters, fits in with any namespace constraints on
   the library in question.


   There's also PROGRAM_EXITING() currently always defined as zero.
   If there's some trivial way to find out if the fini function is
   being called because the program that the library is linked into is
   exiting, we can just skip all the work because the resources are
   about to be freed up anyways.  Generally this is likely to be the
   same as distinguishing whether the library was loaded dynamically
   while the program was running, or loaded as part of program
   startup.  On most platforms, I don't think we can distinguish these
   cases easily, and it's probably not worth expending any significant
   effort.  (Note in particular that atexit() won't do, because if the
   library is explicitly loaded and unloaded, it would have to be able
   to deregister the atexit callback function.  Also, the system limit
   on atexit callbacks may be small.)


   Implementation outline:

   Windows: MAKE_FINI_FUNCTION creates a symbol with a magic name that
   is sought at library build time, and code is added to invoke the
   function when the library is unloaded.  MAKE_INIT_FUNCTION does
   likewise, but the function is invoked when the library is loaded,
   and an extra variable is declared to hold an error code and a "yes
   the initializer ran" flag.  CALL_INIT_FUNCTION blows up if the flag
   isn't set, otherwise returns the error code.

   UNIX: MAKE_INIT_FUNCTION creates and initializes a variable with a
   name derived from the function name, containing a k5_once_t
   (pthread_once_t or int), an error code, and a pointer to the
   function.  The function itself is declared static, but the
   associated variable has external linkage.  CALL_INIT_FUNCTION
   ensures thath the function is called exactly once (pthread_once or
   just check the flag) and returns the stored error code (or the
   pthread_once error).

   (That's the basic idea.  With some debugging assert() calls and
   such, it's a bit more complicated.  And we also need to handle
   doing the pthread test at run time on systems where that works, so
   we use the k5_once_t stuff instead.)

   UNIX, with compiler support: MAKE_FINI_FUNCTION declares the
   function as a destructor, and the run time linker support or
   whatever will cause it to be invoked when the library is unloaded,
   the program ends, etc.

   UNIX, with linker support: MAKE_FINI_FUNCTION creates a symbol with
   a magic name that is sought at library build time, and linker
   options are used to mark it as a finalization function for the
   library.  The symbol must be exported.

   UNIX, no library finalization support: The finalization function
   never runs, and we leak memory.  Tough.

   DELAY_INITIALIZER will be defined by the configure script if we
   want to use k5_once instead of load-time initialization.  That'll
   be the preferred method on most systems except Windows, where we
   have to initialize some mutexes.




   For maximum flexibility in defining the macros, the function name
   parameter should be a simple name, not even a macro defined as
   another name.  The function should have a unique name, and should
   conform to whatever namespace is used by the library in question.
   (We do have export lists, but (1) they're not used for all
   platforms, and (2) they're not used for static libraries.)

   If the macro expansion needs the function to have been declared, it
   must include a declaration.  If it is not necessary for the symbol
   name to be exported from the object file, the macro should declare
   it as "static".  Hence the signature must exactly match "void
   foo(void)".  (ANSI C allows a static declaration followed by a
   non-static one; the result is internal linkage.)  The macro
   expansion has to come before the function, because gcc apparently
   won't act on "__attribute__((constructor))" if it comes after the
   function definition.

   This is going to be compiler- and environment-specific, and may
   require some support at library build time, and/or "asm"
   statements.  But through macro expansion and auxiliary functions,
   we should be able to handle most things except #pragma.

   It's okay for this code to require that the library be built
   with the same compiler and compiler options throughout, but
   we shouldn't require that the library and application use the
   same compiler.

   For static libraries, we don't really care about cleanup too much,
   since it's all memory handling and mutex allocation which will all
   be cleaned up when the program exits.  Thus, it's okay if gcc-built
   static libraries don't play nicely with cc-built executables when
   it comes to static constructors, just as long as it doesn't cause
   linking to fail.

   For dynamic libraries on UNIX, we'll use pthread_once-type support
   to do delayed initialization, so if finalization can't be made to
   work, we'll only have memory leaks in a load/use/unload cycle.  If
   anyone (like, say, the OS vendor) complains about this, they can
   tell us how to get a shared library finalization function invoked
   automatically.

   Currently there's --disable-delayed-initialization for preventing
   the initialization from being delayed on UNIX, but that's mainly
   just for testing the linker options for initialization, and will
   probably be removed at some point.  */
/* Helper macros.  */
/* XXX Should test USE_LINKER_INIT_OPTION early, and if it's set,
   always provide a function by the expected name, even if we're
   delaying initialization.  */
/* Run the initialization code during program execution, at the latest
   possible moment.  This means multiple threads may be active.  */
/* Do it in macro form so we get the file/line of the invocation if
   the assertion fails.  */
/* forward declaration for use in initializer */
/* so ';' following macro use won't get error */
/* This should be called in finalization only, so we shouldn't have
   multiple active threads mucking around in our library at this
   point.  So ignore the once_t object and just look at the flag.

   XXX Could we have problems with memory coherence between processors
   if we don't invoke mutex/once routines?  Probably not, the
   application code should already be coordinating things such that
   the library code is not in use by this point, and memory
   synchronization will be needed there.  */
/* If we're using gcc, if the C++ support works, the compiler should
   build executables and shared libraries that support the use of
   static constructors and destructors.  The C compiler supports a
   function attribute that makes use of the same facility as C++.

   XXX How do we know if the C++ support actually works?  */
/* Read and write integer values as (unaligned) octet strings in
   specific byte orders.  Add per-platform optimizations as
   needed.  */
/* Check for BIG/LITTLE_ENDIAN macros.  If exactly one is defined, use
   it.  If both are defined, then BYTE_ORDER should be defined and
   match one of them.  Try those symbols, then try again with an
   underscore prefix.  */
/* Optimize for GCC on platforms with known byte orders.

   GCC's packed structures can be written to with any alignment; the
   compiler will use byte operations, unaligned-word operations, or
   normal memory ops as appropriate for the architecture.

   This assumes the availability of uint##_t types, which should work
   on most of our platforms except Windows, where we're not using
   GCC.  */
/* To do: Define SWAP16, SWAP32, SWAP64 macros to byte-swap values
   with the indicated numbers of bits.

   Linux: byteswap.h, bswap_16 etc.
   Solaris 10: none
   macOS: machine/endian.h or byte_order.h, NXSwap{Short,Int,LongLong}
   NetBSD: sys/bswap.h, bswap16 etc.  */
/* Note that on Windows at least this file can be included from C++
   source, so casts *from* void* are required.  */
/* Assume for simplicity that these swaps are identical.  */
/* Make the interfaces to getpwnam and getpwuid consistent.
   Model the wrappers on the POSIX thread-safe versions, but
   use the unsafe system versions if the safe ones don't exist
   or we can't figure out their interfaces.  */
/* int k5_getpwnam_r(const char *, blah blah) */
/* POSIX */
/* no getpwnam_r, or can't figure out #args or return type */
/* int k5_getpwuid_r(uid_t, blah blah) */
/* POSIX */
/* no getpwuid_r, or can't figure out #args or return type */
/* Ensure, if possible, that the indicated file descriptor won't be
   kept open if we exec another process (e.g., launching a ccapi
   server).  If we don't know how to do it... well, just go about our
   business.  Probably most callers won't check the return status
   anyways.  */
/* Macros make the Sun compiler happier, and all variants of this do a
   single evaluation of the argument, and fcntl and fileno should
   produce reasonable error messages on type mismatches, on any system
   with F_SETFD.  */
/* Since the original ANSI C spec left it undefined whether or
   how you could copy around a va_list, C 99 added va_copy.
   For old implementations, let's do our best to fake it.

   XXX Doesn't yet handle implementations with __va_copy (early draft)
   or GCC's __builtin_va_copy.  */
/* Do nothing.  */
/* Provide strlcpy/strlcat interfaces. */
/* Provide fnmatch interface. */
/* Provide [v]asprintf interfaces.  */
/* no vsnprintf */
/* have vasprintf and prototype? */
/* Return true if the snprintf return value RESULT reflects a buffer
   overflow for the buffer size SIZE.

   We cast the result to unsigned int for two reasons.  First, old
   implementations of snprintf (such as the one in Solaris 9 and
   prior) return -1 on a buffer overflow.  Casting the result to -1
   will convert that value to UINT_MAX, which should compare larger
   than any reasonable buffer size.  Second, comparing signed and
   unsigned integers will generate warnings with some compilers, and
   can have unpredictable results, particularly when the relative
   widths of the types is not known (size_t may be the same width as
   int or larger).
*/
/*
 * Attempt to zero memory in a way that compilers won't optimize out.
 *
 * This mechanism should work even for heap storage about to be freed,
 * or automatic storage right before we return from a function.
 *
 * Then, even if we leak uninitialized memory someplace, or UNIX
 * "core" files get created with world-read access, some of the most
 * sensitive data in the process memory will already be safely wiped.
 *
 * We're not going so far -- yet -- as to try to protect key data that
 * may have been written into swap space....
 */
/*
 * Return 0 if the n-byte memory regions p1 and p2 are equal, and nonzero if
 * they are not.  The function is intended to take the same amount of time
 * regardless of how many bytes of p1 and p2 are equal.
 */
/*
 * Split a path into parent directory and basename.  Either output parameter
 * may be NULL if the caller doesn't need it.  parent_out will be empty if path
 * has no basename.  basename_out will be empty if path ends with a path
 * separator.  Returns 0 on success or ENOMEM on allocation failure.
 */
/*
 * Compose two path components, inserting the platform-appropriate path
 * separator if needed.  If path2 is an absolute path, path1 will be discarded
 * and path_out will be a copy of path2.  Returns 0 on success or ENOMEM on
 * allocation failure.
 */
/* Return 1 if path is absolute, 0 if it is relative. */
/*
 * Localization macros.  If we have gettext, define _ appropriately for
 * translating a string.  If we do not have gettext, define _ and
 * bindtextdomain as no-ops.  N_ is always a no-op; it marks a string for
 * extraction to pot files but does not translate it.
 */
/* HAVE_GETOPT */
/* HAVE_GETOPT_LONG */
/* Set *fnames_out to a null-terminated list of filenames within dirname,
 * sorted according to strcmp().  Return 0 on success, or ENOENT/ENOMEM. */
/* _WIN32 */
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub unsafe extern "C" fn k5_dir_filenames(mut dirname: *const libc::c_char,
                                          mut fnames_out:
                                              *mut *mut *mut libc::c_char)
 -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut ent: *mut dirent = 0 as *mut dirent;
    let mut fnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n_fnames: libc::c_int = 0 as libc::c_int;
    *fnames_out = 0 as *mut *mut libc::c_char;
    dir = opendir(dirname);
    if dir.is_null() { return 2 as libc::c_int }
    loop  {
        ent = readdir(dir);
        if ent.is_null() { break ; }
        if add_filename(&mut fnames, &mut n_fnames,
                        (*ent).d_name.as_mut_ptr()) != 0 as libc::c_int {
            k5_free_filenames(fnames);
            closedir(dir);
            return 12 as libc::c_int
        }
    }
    closedir(dir);
    qsort(fnames as *mut libc::c_void, n_fnames as size_t,
          ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
          Some(compare_with_strcmp as
                   unsafe extern "C" fn(_: *const libc::c_void,
                                        _: *const libc::c_void)
                       -> libc::c_int));
    *fnames_out = fnames;
    return 0 as libc::c_int;
}
/* not _WIN32 */
