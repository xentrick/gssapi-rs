use ::libc;
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    }
}
use self::string_h::strdup;
/*
 * Copyright 2011 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
/* Stub implementation of module loading for MIT krb5 bundled libverto. */
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn module_symbol_is_present(mut modname:
                                                      *const libc::c_char,
                                                  mut symbname:
                                                      *const libc::c_char)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn module_get_filename_for_symbol(mut addr:
                                                            *mut libc::c_void,
                                                        mut filename:
                                                            *mut *mut libc::c_char)
 -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "41:1"]
pub unsafe extern "C" fn module_close(mut dll: *mut libc::c_void) { }
#[no_mangle]
#[c2rust::src_loc = "46:1"]
pub unsafe extern "C" fn module_load(mut filename: *const libc::c_char,
                                     mut symbname: *const libc::c_char,
                                     mut shouldload:
                                         Option<unsafe extern "C" fn(_:
                                                                         *mut libc::c_void,
                                                                     _:
                                                                         *mut libc::c_void,
                                                                     _:
                                                                         *mut *mut libc::c_char)
                                                    -> libc::c_int>,
                                     mut misc: *mut libc::c_void,
                                     mut dll: *mut *mut libc::c_void,
                                     mut symb: *mut *mut libc::c_void)
 -> *mut libc::c_char {
    if !dll.is_null() { *dll = 0 as *mut libc::c_void }
    if !symb.is_null() { *symb = 0 as *mut libc::c_void }
    return strdup(b"module loading disabled\x00" as *const u8 as
                      *const libc::c_char);
}
