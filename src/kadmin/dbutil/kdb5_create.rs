use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:56"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:56"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:56"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:56"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:56"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:56"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:56"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /*
 * end wordsize.h
 */
    /*
 * begin "base-defs.h"
 */
    /*
 * Basic definitions for Kerberos V5 library
 */
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
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
    /* Originally used to recognize AFS and default salts.  No longer used. */
    #[c2rust::src_loc = "225:1"]
    pub type krb5_pointer = *mut libc::c_void;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    /* *< Service and other unique instance (krbtgt) */
    /* *< Service with host name as instance
                                      (telnet, rcommands) */
    /* *< Service with host as remaining components */
    /* *< Unique ID */
    /* *< PKINIT */
    /* *< Name in form of SMTP email name */
    /* *< Windows 2000 UPN */
    /* *< Well-known (special) principal */
    /* *< First component of
                                                NT_WELLKNOWN principals */
    /* *< Windows 2000 UPN and SID */
    /* *< NT 4 style name */
    /* *< NT 4 style name and SID */
    /* * Constant version of krb5_principal_data */
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    /* *
 * Hook function for inspecting or overriding KDC replies.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  code            Status of KDC communication
 * @param [in]  realm           The realm the reply was received from
 * @param [in]  message         The message sent to the realm's KDC
 * @param [in]  reply           The reply received from the KDC
 * @param [out] new_reply_out   Optional replacement reply
 *
 * If @a code is zero, @a reply contains the reply received from the KDC.  The
 * hook function may return an error code to simulate an error, may synthesize
 * a different reply by setting @a new_reply_out, or may simply return
 * successfully to do nothing.
 *
 * If @a code is non-zero, KDC communication failed and @a reply should be
 * ignored.  The hook function may return @a code or a different error code, or
 * may synthesize a reply by setting @a new_reply_out and return successfully.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_reply_out, to ensure that it can be freed correctly by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    /* *
 * Hook function for inspecting or modifying messages sent to KDCs.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  realm           The realm the message will be sent to
 * @param [in]  message         The original message to be sent to the KDC
 * @param [out] new_message_out Optional replacement message to be sent
 * @param [out] reply_out       Optional synthetic reply
 *
 * If the hook function returns an error code, the KDC communication will be
 * aborted and the error code will be returned to the library operation which
 * initiated the communication.
 *
 * If the hook function sets @a reply_out, @a message will not be sent to the
 * KDC, and the given reply will used instead.
 *
 * If the hook function sets @a new_message_out, the given message will be sent
 * to the KDC in place of @a message.
 *
 * If the hook function returns successfully without setting either output,
 * @a message will be sent to the KDC normally.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_message_out or @a reply_out, to ensure that it can be freed correctly
 * by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    #[c2rust::src_loc = "8475:1"]
    pub type krb5_pre_send_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "8391:1"]
    pub type krb5_trace_callback
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *const krb5_trace_info,
                                    _: *mut libc::c_void) -> ()>;
    /* *
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* This file is generated, please don't edit it directly.  */
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* General definitions for Kerberos version 5. */
/*
 * Copyright 1989, 1990, 1995, 2001, 2003, 2007, 2011 by the Massachusetts
 * Institute of Technology.  All Rights Reserved.
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
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
        /* * @defgroup KRB5_H krb5 library API
 * @{
 */
        /* By default, do not expose deprecated interfaces. */
        /* !KRB5_CONFIG__ */
        /* for *_MAX */
        /* from profile.h */
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        /* *
 * Generate an enctype-specific random encryption key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type of the generated key
 * @param [out] k5_random_key   An allocated and initialized keyblock
 *
 * Use krb5_free_keyblock_contents() to free @a k5_random_key when
 * no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "745:1"]
        pub fn krb5_c_make_random_key(context: krb5_context,
                                      enctype: krb5_enctype,
                                      k5_random_key: *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Collect entropy from the OS if possible.
 *
 * @param [in]  context         Library context
 * @param [in]  strong          Strongest available source of entropy
 * @param [out] success         1 if OS provides entropy, 0 otherwise
 *
 * If @a strong is non-zero, this function attempts to use the strongest
 * available source of entropy.  Setting this flag may cause the function to
 * block on some operating systems.  Good uses include seeding the PRNG for
 * kadmind and realm setup.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "817:1"]
        pub fn krb5_c_random_os_entropy(context: krb5_context,
                                        strong: libc::c_int,
                                        success: *mut libc::c_int)
         -> krb5_error_code;
        /* * @deprecated Replaced by krb5_c_* API family. */
        #[no_mangle]
        #[c2rust::src_loc = "821:1"]
        pub fn krb5_c_random_seed(context: krb5_context, data: *mut krb5_data)
         -> krb5_error_code;
        /* *
 * Convert a string (such a password) to a key.
 *
 * @param [in]  context         Library context
 * @param [in]  enctype         Encryption type
 * @param [in]  string          String to be converted
 * @param [in]  salt            Salt value
 * @param [out] key             Generated key
 *
 * This function converts @a string to a @a key of encryption type @a enctype,
 * using the specified @a salt.  The newly created @a key must be released by
 * calling krb5_free_keyblock_contents() when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "839:1"]
        pub fn krb5_c_string_to_key(context: krb5_context,
                                    enctype: krb5_enctype,
                                    string: *const krb5_data,
                                    salt: *const krb5_data,
                                    key: *mut krb5_keyblock)
         -> krb5_error_code;
        /* *
 * Copy a principal.
 *
 * @param [in]  context         Library context
 * @param [in]  inprinc         Principal to be copied
 * @param [out] outprinc        Copy of @a inprinc
 *
 * This function creates a new principal structure with the contents of @a
 * inprinc.  Use krb5_free_principal() to free @a outprinc when it is no longer
 * needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
         -> krb5_error_code;
        /* *
 * Convert a principal name into the default salt for that principal.
 *
 * @param [in]  context         Library context
 * @param [in]  pr              Principal name
 * @param [out] ret             Default salt for @a pr to be filled in
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4313:1"]
        pub fn krb5_principal2salt(context: krb5_context,
                                   pr: krb5_const_principal,
                                   ret: *mut krb5_data) -> krb5_error_code;
        /* *
 * Free the contents of a krb5_keyblock structure.
 *
 * @param [in] context          Library context
 * @param [in] key              Keyblock to be freed
 *
 * This function frees the contents of @a key, but not the structure itself.
 */
        #[no_mangle]
        #[c2rust::src_loc = "4721:1"]
        pub fn krb5_free_keyblock_contents(context: krb5_context,
                                           key: *mut krb5_keyblock);
        /* *
 * Retrieve the current time with context specific time offset adjustment.
 *
 * @param [in]  context         Library context
 * @param [out] timeret         Timestamp to fill in
 *
 * This function retrieves the system time of day with the context specific
 * time offset adjustment.
 *
 * @retval
 * 0  Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "4837:1"]
        pub fn krb5_timeofday(context: krb5_context,
                              timeret: *mut krb5_timestamp)
         -> krb5_error_code;
        /*
 * end "func-proto.h"
 */
        /*
 * begin stuff from libos.h
 */
        /* *
 * @brief Read a password from keyboard input.
 *
 * @param [in]     context      Library context
 * @param [in]     prompt       First user prompt when reading password
 * @param [in]     prompt2      Second user prompt (NULL to prompt only once)
 * @param [out]    return_pwd   Returned password
 * @param [in,out] size_return  On input, maximum size of password; on output,
 *                              size of password read
 *
 * This function reads a password from keyboard input and stores it in @a
 * return_pwd.  @a size_return should be set by the caller to the amount of
 * storage space available in @a return_pwd; on successful return, it will be
 * set to the length of the password read.
 *
 * @a prompt is printed to the terminal, followed by ": ", and then a password
 * is read from the keyboard.
 *
 * If @a prompt2 is NULL, the password is read only once.  Otherwise, @a
 * prompt2 is printed to the terminal and a second password is read.  If the
 * two passwords entered are not identical, KRB5_LIBOS_BADPWDMATCH is returned.
 *
 * Echoing is turned off when the password is read.
 *
 * @retval
 *  0   Success
 * @return
 * Error in reading or verifying the password
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "6096:1"]
        pub fn krb5_read_password(context: krb5_context,
                                  prompt: *const libc::c_char,
                                  prompt2: *const libc::c_char,
                                  return_pwd: *mut libc::c_char,
                                  size_return: *mut libc::c_uint)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:56"]
pub mod k5_int_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 1989,1990,1991,1992,1993,1994,1995,2000,2001,
 * 2003,2006,2007,2008,2009 by the Massachusetts Institute of Technology,
 * Cambridge, MA, USA.  All Rights Reserved.
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
/*
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
    /*
 * This prototype for k5-int.h (Krb5 internals include file)
 * includes the user-visible definitions from krb5.h and then
 * includes other definitions that are not user-visible but are
 * required for compiling Kerberos internal routines.
 *
 * John Gilmore, Cygnus Support, Sat Jan 21 22:45:52 PST 1995
 */
    /* KRB5_GENERAL__ */
    /*
 * Begin "k5-config.h"
 */
    /*
 * Machine-type definitions: PC Clone 386 running Microloss Windows
 */
    /* From autoconf.h */
    /* HAVE_SYS_TYPES_H */
    /* HAVE_SYS_TYPES_H */
    /* KRB5_SYSTYPES__ */
    /* one day */
    /* one week */
    /* Thu Jan  1 00:00:00 2038 UTC */
    /*
 * Windows requires a different api interface to each function. Here
 * just define it as NULL.
 */
    /* #define KRB5_OLD_CRYPTO is done in krb5.h */
    /* KRB5_CONFIG__ */
    /*
 * End "k5-config.h"
 */
    /*
 * After loading the configuration definitions, load the Kerberos definitions.
 */
    /* Get mutex support; currently used only for the replay cache.  */
    /* Get error info support.  */
    /* Get string buffer support. */
    /* Define tracing macros. */
    /* Profile variables.  Constants are named KRB5_CONF_STRING, where STRING
 * matches the variable name.  Keep these alphabetized. */
    /* Cache configuration variables */
    /* Error codes used in KRB_ERROR protocol messages.
   Return values of library routines are based on a different error table
   (which allows non-ambiguous error codes between subsystems) */
    /* KDC errors */
    /* No error */
    /* Client's entry in DB expired */
    /* Server's entry in DB expired */
    /* Requested pvno not supported */
    /* C's key encrypted in old master */
    /* S's key encrypted in old master */
    /* Client not found in Kerberos DB */
    /* Server not found in Kerberos DB */
    /* Multiple entries in Kerberos DB */
    /* The C or S has a null key */
    /* Tkt ineligible for postdating */
    /* Requested starttime > endtime */
    /* KDC policy rejects request */
    /* KDC can't do requested opt. */
    /* No support for encryption type */
    /* No support for checksum type */
    /* No support for padata type */
    /* No support for transited type */
    /* C's creds have been revoked */
    /* S's creds have been revoked */
    /* TGT has been revoked */
    /* C not yet valid */
    /* S not yet valid */
    /* Password has expired */
    /* Preauthentication failed */
    /* Additional preauthentication */
                                           /* required */
    /* Requested server and */
                                           /* ticket don't match*/
    /* Server principal valid for */
                                           /*   user2user only */
    /* KDC policy rejected transited */
                                           /*   path */
    /* A service is not
                                            * available that is
                                            * required to process the
                                            * request */
    /* Application errors */
    /* Decrypt integrity check failed */
    /* Ticket expired */
    /* Ticket not yet valid */
    /* Request is a replay */
    /* The ticket isn't for us */
    /* Ticket/authenticator don't match */
    /* Clock skew too great */
    /* Incorrect net address */
    /* Protocol version mismatch */
    /* Invalid message type */
    /* Message stream modified */
    /* Message out of order */
    /* Key version is not available */
    /* Service key not available */
    /* Mutual authentication failed */
    /* Incorrect message direction */
    /* Alternative authentication */
                                        /* method required */
    /* Incorrect sequence numnber */
                                        /* in message */
    /* Inappropriate type of */
                                        /* checksum in message */
    /* Policy rejects transited path */
    /* Response too big for UDP, */
                                        /*   retry with TCP */
    /* other errors */
    /* Generic error (description */
                                        /* in e-text) */
    /* Field is too long for impl. */
    /* PKINIT server-reported errors */
    /* client cert not trusted */
    /* client signature verify failed */
    /* invalid Diffie-Hellman parameters */
    /* client cert not verifiable to */
                                                   /* trusted root cert */
    /* client cert had invalid signature */
    /* client cert was revoked */
    /* client cert revoked, reason unknown */
    /* mismatch between client cert and */
                                                   /* principal name */
    /* bad extended key use */
    /* bad digest algorithm in client cert */
    /* missing paChecksum in PA-PK-AS-REQ */
    /* bad digest algorithm in SignedData */
    /* The IAKERB proxy could
                                                      not find a KDC */
    /* The KDC did not respond
                                                      to the IAKERB proxy */
    /* RFC 6113 */
    /* RFC 6113 */
    /* err table base max offset for protocol err codes */
    /*
 * A null-terminated array of this structure is returned by the KDC as
 * the data part of the ETYPE_INFO preauth type.  It informs the
 * client which encryption types are supported.
 * The  same data structure is used by both etype-info and etype-info2
 * but s2kparams must be null when encoding etype-info.
 */
    /*
 *  This is essentially -1 without sign extension which can screw up
 *  comparisons on 64 bit machines. If the length is this value, then
 *  the salt data is not present. This is to distinguish between not
 *  being set and being of 0 length.
 */
    /* RFC 4537 */
    /* sam_type values -- informational only */
    /*  Enigma Logic */
    /*  Digital Pathways */
    /*  S/key where  KDC has key 0 */
    /*  Traditional S/Key */
    /*  Security Dynamics */
    /*  CRYPTOCard */
    /* XXX need to figure out who has which numbers assigned */
    /*  ActivCard decimal mode */
    /*  ActivCard hex mode */
    /*  Digital Pathways hex mode */
    /* experimental */
    /* testing */
    /* special */
    /* Array of checksums */
    /* information */
    /* KRB5_SAM_* values */
    /* informational */
    /* KRB5_SAM_* values */
    /* copied */
    /* krb5_enc_sam_response_enc */
    /*
 * Keep the pkinit definitions in a separate file so that the plugin
 * only has to include k5-int-pkinit.h rather than k5-int.h
 */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* Plain text of an encrypted PA-FX-COOKIE value produced by the KDC. */
    /* In PAC options, indicates Resource-Based Constrained Delegation support. */
    /* struct stat, stat() */
    /* MAXPATHLEN */
    /* prototypes for file-related
                                           syscalls; flags for open &
                                           friends */
    /* libos.spec */
    /* Internal structure of an opaque key identifier */
    /*
     * Cache of data private to the cipher implementation, which we
     * don't want to have to recompute for every operation.  This may
     * include key schedules, iteration counts, etc.
     *
     * The cipher implementation is responsible for setting this up
     * whenever needed, and the enc_provider key_cleanup method must
     * then be provided to dispose of it.
     */
    /* Write the SHA-256 hash of in (containing n elements) to out. */
    /* Convenience function: zap and free ptr if it is non-NULL. */
    /* Convenience function: zap and free zero-terminated str if it is non-NULL. */
    /* Convenience function: zap and free krb5_data pointer if it is non-NULL. */
    /*
 * End "los-proto.h"
 */
    /*
 * Flags for the os_flags field
 *
 * KRB5_OS_TOFFSET_VALID means that the time offset fields are valid.
 * The intention is that this facility to correct the system clocks so
 * that they reflect the "real" time, for systems where for some
 * reason we can't set the system clock.  Instead we calculate the
 * offset between the system time and real time, and store the offset
 * in the os context so that we can correct the system clock as necessary.
 *
 * KRB5_OS_TOFFSET_TIME means that the time offset fields should be
 * returned as the time by the krb5 time routines.  This should only
 * be used for testing purposes (obviously!)
 */
    /* lock mode flags */
    /*
 * Begin "preauth.h"
 *
 * (Originally written by Glen Machin at Sandia Labs.)
 */
/*
 * Sandia National Laboratories also makes no representations about the
 * suitability of the modifications, or additions to this software for
 * any purpose.  It is provided "as is" without express or implied warranty.
 */
    /* check logon hour restrictions */
    /* sign with usage 27 instead of 26 */
    /* padata from req_body is used*/
    /* Bits 0-15 are critical in FAST options (RFC 6113 section 7.3). */
    /*
 * AD-CAMMAC's other-verifiers field is a sequence of Verifier, which is an
 * extensible choice with only one selection, Verifier-MAC.  For the time being
 * we will represent this field directly as an array of krb5_verifier_mac.
 * That will have to change if other selections are added.
 */
    /* Does not return a copy; original padata sequence responsible for freeing*/
    /* Allocate a pa-data object with uninitialized contents of size len.  If len
 * is 0, set the contents field to NULL. */
    /* Free a single pa-data object. */
    /* Without copying, add single element *pa to *list, reallocating as necessary.
 * If *list is NULL, allocate a new list.  Set *pa to NULL on success. */
    /* Without copying, add a pa-data element of type pa_type to *list with the
 * contents in data.  Set *data to empty_data() on success. */
    /* Add an empty pa-data element of type pa_type to *list. */
    /* KRB5_PREAUTH__ */
    /*
 * End "preauth.h"
 */
    /* #include "krb5/wordsize.h" -- comes in through base-defs.h. */
    /* ** Plugin framework ***/
    /*
 * This framework can be used to create pluggable interfaces.  Not all existing
 * pluggable interface use this framework, but new ones should.  A new
 * pluggable interface entails:
 *
 * - An interface ID definition in the list of #defines below.
 *
 * - A name in the interface_names array in lib/krb5/krb/plugins.c.
 *
 * - An installed public header file in include/krb5.  The public header should
 *   include <krb5/plugin.h> and should declare a vtable structure for each
 *   supported major version of the interface.
 *
 * - A consumer API implementation, located within the code unit which makes
 *   use of the pluggable interface.  The consumer API should consist of:
 *
 *   . An interface-specific handle type which contains a vtable structure for
 *     the module (or a union of several such structures, if there are multiple
 *     supported major versions) and, optionally, resource data bound to the
 *     handle.
 *
 *   . An interface-specific loader function which creates a handle or list of
 *     handles.  A list of handles would be created if the interface is a
 *     one-to-many interface where the consumer wants to consult all available
 *     modules; a single handle would be created for an interface where the
 *     consumer wants to consult a specific module.  The loader function should
 *     use k5_plugin_load or k5_plugin_load_all to produce one or a list of
 *     vtable initializer functions, and should use those functions to fill in
 *     the vtable structure for the module (if necessary, trying each supported
 *     major version starting from the most recent).  The loader function can
 *     also bind resource data into the handle based on caller arguments, if
 *     appropriate.
 *
 *   . For each plugin method, a wrapper function which accepts a krb5_context,
 *     a plugin handle, and the method arguments.  Wrapper functions should
 *     invoke the method function contained in the handle's vtable.
 *
 * - Possibly, built-in implementations of the interface, also located within
 *   the code unit which makes use of the interface.  Built-in implementations
 *   must be registered with k5_plugin_register before the first call to
 *   k5_plugin_load or k5_plugin_load_all.
 *
 * A pluggable interface should have one or more currently supported major
 * versions, starting at 1.  Each major version should have a current minor
 * version, also starting at 1.  If new methods are added to a vtable, the
 * minor version should be incremented and the vtable stucture should document
 * where each minor vtable version ends.  If method signatures for a vtable are
 * changed, the major version should be incremented.
 *
 * Plugin module implementations (either built-in or dynamically loaded) should
 * define a function named <interfacename>_<modulename>_initvt, matching the
 * signature of krb5_plugin_initvt_fn as declared in include/krb5/plugin.h.
 * The initvt function should check the given maj_ver argument against its own
 * supported major versions, cast the vtable pointer to the appropriate
 * interface-specific vtable type, and fill in the vtable methods, stopping as
 * appropriate for the given min_ver.  Memory for the vtable structure is
 * allocated by the caller, not by the module.
 *
 * Dynamic plugin modules are registered with the framework through the
 * [plugins] section of the profile, as described in the admin documentation
 * and krb5.conf man page.
 */
    /* Holds krb5_context information about each pluggable interface. */
    /* A list of plugin interface IDs.  Make sure to increment
 * PLUGIN_NUM_INTERFACES when a new interface is added, and add an entry to the
 * interface_names table in lib/krb5/krb/plugin.c. */
    /* Retrieve the plugin module of type interface_id and name modname,
 * storing the result into module. */
    /* Retrieve all plugin modules of type interface_id, storing the result
 * into modules.  Free the result with k5_plugin_free_handles. */
    /* Release a module list allocated by k5_plugin_load_all. */
    /* Register a plugin module of type interface_id and name modname. */
    /*
 * Register a plugin module which is part of the krb5 tree but is built as a
 * dynamic plugin.  Look for the module in modsubdir relative to the
 * context->base_plugin_dir.
 */
    /* Destroy the module state within context; used by krb5_free_context. */
    /* private, in kdb5.h */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1208:8"]
    pub struct _krb5_context {
        pub magic: krb5_magic,
        pub in_tkt_etypes: *mut krb5_enctype,
        pub tgs_etypes: *mut krb5_enctype,
        pub os_context: _krb5_os_context,
        pub default_realm: *mut libc::c_char,
        pub profile: profile_t,
        pub dal_handle: *mut kdb5_dal_handle,
        pub clockskew: krb5_deltat,
        pub kdc_default_options: krb5_flags,
        pub library_options: krb5_flags,
        pub profile_secure: krb5_boolean,
        pub fcc_default_format: libc::c_int,
        pub prompt_types: *mut krb5_prompt_type,
        pub udp_pref_limit: libc::c_int,
        pub use_conf_ktypes: krb5_boolean,
        pub libkrb5_plugins: plugin_dir_handle,
        pub preauth_context: krb5_preauth_context,
        pub ccselect_handles: *mut *mut ccselect_module_handle,
        pub localauth_handles: *mut *mut localauth_module_handle,
        pub hostrealm_handles: *mut *mut hostrealm_module_handle,
        pub tls: *mut k5_tls_vtable_st,
        pub err: errinfo,
        pub err_fmt: *mut libc::c_char,
        pub kdblog_context: *mut _kdb_log_context,
        pub allow_weak_crypto: krb5_boolean,
        pub ignore_acceptor_hostname: krb5_boolean,
        pub enforce_ok_as_delegate: krb5_boolean,
        pub dns_canonicalize_hostname: dns_canonhost,
        pub trace_callback: krb5_trace_callback,
        pub trace_callback_data: *mut libc::c_void,
        pub kdc_send_hook: krb5_pre_send_fn,
        pub kdc_send_hook_data: *mut libc::c_void,
        pub kdc_recv_hook: krb5_post_recv_fn,
        pub kdc_recv_hook_data: *mut libc::c_void,
        pub plugins: [plugin_interface; 13],
        pub plugin_base_dir: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1137:8"]
    pub struct plugin_interface {
        pub modules: *mut *mut plugin_mapping,
        pub configured: krb5_boolean,
    }
    #[c2rust::src_loc = "1194:1"]
    pub type dns_canonhost = libc::c_uint;
    #[c2rust::src_loc = "1197:5"]
    pub const CANONHOST_FALLBACK: dns_canonhost = 2;
    #[c2rust::src_loc = "1196:5"]
    pub const CANONHOST_TRUE: dns_canonhost = 1;
    #[c2rust::src_loc = "1195:5"]
    pub const CANONHOST_FALSE: dns_canonhost = 0;
    #[c2rust::src_loc = "1203:1"]
    pub type krb5_preauth_context = *mut krb5_preauth_context_st;
    #[c2rust::src_loc = "1201:1"]
    pub type kdb5_dal_handle = _kdb5_dal_handle;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "702:16"]
    pub struct _krb5_os_context {
        pub magic: krb5_magic,
        pub time_offset: krb5_int32,
        pub usec_offset: krb5_int32,
        pub os_flags: krb5_int32,
        pub default_ccname: *mut libc::c_char,
    }
    #[inline]
    #[c2rust::src_loc = "656:1"]
    pub unsafe extern "C" fn zapfree(mut ptr: *mut libc::c_void,
                                     mut len: size_t) {
        if !ptr.is_null() { explicit_bzero(ptr, len); free(ptr); };
    }
    #[inline]
    #[c2rust::src_loc = "2251:1"]
    pub unsafe extern "C" fn make_data(mut data: *mut libc::c_void,
                                       mut len: libc::c_uint) -> krb5_data {
        let mut d: krb5_data =
            krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
        d.magic = -(1760647422 as libc::c_long) as krb5_magic;
        d.data = data as *mut libc::c_char;
        d.length = len;
        return d;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb_log_h::_kdb_log_context;
    use super::stddef_h::size_t;
    use super::string_h::explicit_bzero;
    use super::stdlib_h::free;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1206:8"]
        pub type hostrealm_module_handle;
        #[c2rust::src_loc = "1205:8"]
        pub type localauth_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:61"]
pub mod kdb_log_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:16"]
    pub struct _kdb_log_context {
        pub iproprole: iprop_role,
        pub ulog: *mut kdb_hlog_t,
        pub ulogentries: uint32_t,
        pub ulogfd: libc::c_int,
    }
    #[c2rust::src_loc = "81:1"]
    pub type kdb_hlog_t = kdb_hlog;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct kdb_hlog {
        pub kdb_hmagic: uint32_t,
        pub db_version_num: uint16_t,
        pub kdb_num: uint32_t,
        pub kdb_first_time: kdbe_time_t,
        pub kdb_last_time: kdbe_time_t,
        pub kdb_first_sno: kdb_sno_t,
        pub kdb_last_sno: kdb_sno_t,
        pub kdb_state: uint16_t,
        pub kdb_block: uint16_t,
    }
    #[c2rust::src_loc = "102:1"]
    pub type kdb_log_context = _kdb_log_context;
    use super::iprop_hdr_h::iprop_role;
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t};
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
        /* #pragma ident        "@(#)kdb_log.h  1.3     04/02/23 SMI" */
        /*
 * DB macros
 */
        /*
 * Current DB version #
 */
        /*
 * DB log states
 */
        /*
 * DB log constants
 */
        /*
 * Default ulog file attributes
 */
        /* in seconds */
        /*
 * Max size of update entry + update header
 * We make this large since resizing can be costly.
 */
        /* Default size of principal record */
        /* 256 MB log file */
        /*
 * Prototype declarations
 */
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn ulog_map(context: krb5_context, logname: *const libc::c_char,
                        entries: uint32_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "63:1"]
        pub fn ulog_init_header(context: krb5_context) -> krb5_error_code;
    }
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:61"]
pub mod iprop_h {
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    use super::stdint_uintn_h::uint32_t;
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:61"]
pub mod iprop_hdr_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* #pragma ident        "@(#)iprop_hdr.h        1.1     04/02/20 SMI" */
    /*
 * This file has some defines common to the iprop client and
 * server routines.
 */
    /*
 * Maximum size for each ulog entry is 2KB and maximum
 * possible attribute-value pairs for each ulog entry is 20
 */
    /* Backoff for a maximum for 5 mts */
    #[c2rust::src_loc = "32:1"]
    pub type iprop_role = libc::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const IPROP_REPLICA: iprop_role = 2;
    #[c2rust::src_loc = "34:5"]
    pub const IPROP_MASTER: iprop_role = 1;
    #[c2rust::src_loc = "33:5"]
    pub const IPROP_NULL: iprop_role = 0;
    /* !_IPROP_HDR_H */
    /*
 * Full resync dump versioning
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:56"]
pub mod k5_err_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-err.h */
/*
 * Copyright 2006, 2007 Massachusetts Institute of Technology.
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
 *
 * Error-message handling
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct errinfo {
        pub code: libc::c_long,
        pub msg: *mut libc::c_char,
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:56"]
pub mod k5_plugin_h {
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
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:56"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:56"]
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
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:57"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    /* NOT saved */
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:16"]
    pub struct _krb5_db_entry_new {
        pub magic: krb5_magic,
        pub len: krb5_ui_2,
        pub mask: krb5_ui_4,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_tl_data: krb5_int16,
        pub n_key_data: krb5_int16,
        pub e_length: krb5_ui_2,
        pub e_data: *mut krb5_octet,
        pub princ: krb5_principal,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    /* Length, data */
    /*
 * A principal database entry.  Extensions to this structure currently use the
 * tl_data list.  The e_data and e_length fields are not used by any calling
 * code except kdb5_util dump and load, which marshal and unmarshal the array
 * in the dump record.  KDB modules may use these fields internally as long as
 * they set e_length appropriately (non-zero if the data should be marshalled
 * across dump and load, zero if not) and handle null e_data values in
 * caller-constructed principal entries.
 */
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "282:16"]
    pub struct _krb5_actkvno_node {
        pub next: *mut _krb5_actkvno_node,
        pub act_kvno: krb5_kvno,
        pub act_time: krb5_timestamp,
    }
    /* NOT saved */
    /* members currently changed/set */
    /* When the client expires */
    /* When its passwd expires */
    /* Last successful passwd */
    /* Last failed passwd attempt */
    /* # of failed passwd attempt */
    /* Length of extra data */
    /* Extra data to be saved */
    /* Length, data */
    /* Linked list */
    /* key_data must be sorted by kvno in descending order. */
    /* Array */
    /* SECURID */
    /* String attributes may not always be represented in tl-data.  kadmin clients
 * must use the get_strings and set_string RPCs. */
    /* NDR encoded validation info */
    /* ASN.1 encoded ServerReferralInfo */
    /* ASN.1 encoded PA-SVR-REFERRAL-DATA */
    /* Each entry is a permitted SPN */
    /* LM OWF */
    /* <I>IssuerDN<S>SubjectDN */
    /* Timestamp of admin unlock */
    /* version number for KRB5_TL_ACTKVNO data */
    /* version number for KRB5_TL_MKEY_AUX data */
    #[c2rust::src_loc = "282:1"]
    pub type krb5_actkvno_node = _krb5_actkvno_node;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_data,
                        krb5_magic, krb5_ui_4, krb5_flags, krb5_deltat,
                        krb5_timestamp, krb5_kvno, krb5_principal,
                        krb5_enctype, krb5_int32, krb5_context,
                        krb5_error_code, krb5_principal_data, krb5_keyblock,
                        krb5_const_principal};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn krb5_db_create(kcontext: krb5_context,
                              db_args: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn krb5_db_put_principal(kcontext: krb5_context,
                                     entry: *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "394:1"]
        pub fn krb5_db_store_master_key(kcontext: krb5_context,
                                        keyfile: *mut libc::c_char,
                                        mname: krb5_principal,
                                        kvno: krb5_kvno,
                                        key: *mut krb5_keyblock,
                                        master_pwd: *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn krb5_db_setup_mkey_name(context: krb5_context,
                                       keyname: *const libc::c_char,
                                       realm: *const libc::c_char,
                                       fullname: *mut *mut libc::c_char,
                                       principal: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn krb5_dbe_encrypt_key_data(context: krb5_context,
                                         mkey: *const krb5_keyblock,
                                         dbkey: *const krb5_keyblock,
                                         keysalt: *const krb5_keysalt,
                                         keyver: libc::c_int,
                                         key_data: *mut krb5_key_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "502:1"]
        pub fn krb5_dbe_update_mkvno(context: krb5_context,
                                     entry: *mut krb5_db_entry,
                                     mkvno: krb5_kvno) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "517:1"]
        pub fn krb5_dbe_update_actkvno(context: krb5_context,
                                       entry: *mut krb5_db_entry,
                                       actkvno_list: *const krb5_actkvno_node)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "537:1"]
        pub fn krb5_dbe_create_key_data(context: krb5_context,
                                        entry: *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "542:1"]
        pub fn krb5_dbe_update_mod_princ_data(context: krb5_context,
                                              entry: *mut krb5_db_entry,
                                              mod_date: krb5_timestamp,
                                              mod_princ: krb5_const_principal)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:58"]
pub mod admin_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src = "/usr/include/unistd.h:56"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "825:1"]
        pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:56"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn getopt(___argc: libc::c_int, ___argv: *const *mut libc::c_char,
                      __shortopts: *const libc::c_char) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:56"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:56"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:56"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/string.h:56"]
pub mod string_h {
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "435:1"]
        pub fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/adm_proto.h:60"]
pub mod adm_proto_h {
    use super::kdb_h::krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int32, krb5_boolean, krb5_error_code,
                        krb5_pointer};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "84:1"]
        pub fn krb5_keysalt_iterate(_: *mut krb5_key_salt_tuple,
                                    _: krb5_int32, _: krb5_boolean,
                                    _:
                                        Option<unsafe extern "C" fn(_:
                                                                        *mut krb5_key_salt_tuple,
                                                                    _:
                                                                        krb5_pointer)
                                                   -> krb5_error_code>,
                                    _: krb5_pointer) -> krb5_error_code;
    }
    /* KRB5_ADM_PROTO_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/kadmin/dbutil/kdb5_util.h:61"]
pub mod kdb5_util_h {
    use super::admin_h::kadm5_config_params;
    use super::krb5_h::{krb5_keyblock, krb5_magic, krb5_enctype, krb5_octet,
                        krb5_principal_data, krb5_principal, krb5_context,
                        krb5_deltat, krb5_timestamp, krb5_flags, krb5_int32,
                        krb5_kvno};
    use super::k5_int_h::_krb5_context;
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/dbutil/kdb5_util.h */
/*
 * Copyright 1992, 2008, 2009 by the Massachusetts Institute of Technology.
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
        #[no_mangle]
        #[c2rust::src_loc = "92:1"]
        pub fn kadm5_create(params: *mut kadm5_config_params) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "32:14"]
        pub static mut progname: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "34:22"]
        pub static mut master_keyblock: krb5_keyblock;
        #[no_mangle]
        #[c2rust::src_loc = "35:23"]
        pub static mut master_princ: krb5_principal;
        #[no_mangle]
        #[c2rust::src_loc = "38:12"]
        pub static mut exit_status: libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "39:21"]
        pub static mut util_context: krb5_context;
        #[no_mangle]
        #[c2rust::src_loc = "40:28"]
        pub static mut global_params: kadm5_config_params;
        #[no_mangle]
        #[c2rust::src_loc = "43:15"]
        pub static mut db5util_db_args: *mut *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "49:1"]
        pub fn usage();
    }
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_pointer, krb5_principal_data, krb5_principal,
                       krb5_const_principal, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, _krb5_keyblock,
                       krb5_keyblock, _profile_t, krb5_c_make_random_key,
                       krb5_c_random_os_entropy, krb5_c_random_seed,
                       krb5_c_string_to_key, krb5_copy_principal,
                       krb5_principal2salt, krb5_free_keyblock_contents,
                       krb5_timeofday, krb5_read_password};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, zapfree, make_data, plugin_mapping,
                         k5_tls_vtable_st, hostrealm_module_handle,
                         localauth_module_handle, ccselect_module_handle,
                         krb5_preauth_context_st, _kdb5_dal_handle};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog,
                          kdb_log_context, ulog_map, ulog_init_header};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, com_err};
pub use self::kdb_h::{_krb5_key_data, _krb5_tl_data, krb5_tl_data,
                      krb5_key_data, _krb5_keysalt, krb5_keysalt,
                      _krb5_db_entry_new, krb5_db_entry,
                      __krb5_key_salt_tuple, krb5_key_salt_tuple,
                      _krb5_actkvno_node, krb5_actkvno_node, krb5_db_create,
                      krb5_db_free_principal, krb5_db_put_principal,
                      krb5_db_store_master_key, krb5_db_setup_mkey_name,
                      krb5_dbe_encrypt_key_data, krb5_dbe_update_mkvno,
                      krb5_dbe_update_actkvno, krb5_dbe_create_key_data,
                      krb5_dbe_update_mod_princ_data};
pub use self::admin_h::{_kadm5_config_params, kadm5_config_params};
use self::unistd_h::unlink;
use self::getopt_core_h::getopt;
use self::libintl_h::dgettext;
use self::stdio_h::{printf, fflush, stdout};
use self::stdlib_h::{free, calloc, malloc};
use self::string_h::{explicit_bzero, strlen, memset};
use self::adm_proto_h::krb5_keysalt_iterate;
use self::kdb5_util_h::{kadm5_create, progname, master_keyblock, master_princ,
                        exit_status, util_context, global_params,
                        db5util_db_args, usage};
extern "C" {
    /*
 * Steps in creating a database:
 *
 * 1) use the db calls to open/create a new database
 *
 * 2) get a realm name for the new db
 *
 * 3) get a master password for the new db; convert to an encryption key.
 *
 * 4) create various required entries in the database
 *
 * 5) close & exit
 */
    #[no_mangle]
    #[c2rust::src_loc = "107:14"]
    pub static mut mkey_fullname: *mut libc::c_char;
    #[no_mangle]
    #[c2rust::src_loc = "135:14"]
    pub static mut mkey_password: *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "69:8"]
pub struct realm_info {
    pub max_life: krb5_deltat,
    pub max_rlife: krb5_deltat,
    pub expiration: krb5_timestamp,
    pub flags: krb5_flags,
    pub key: *mut krb5_keyblock,
    pub nkslist: krb5_int32,
    pub kslist: *mut krb5_key_salt_tuple,
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* kadmin/dbutil/kdb5_create.c - Create a KDC database */
/*
 * Copyright 1990,1991,2001, 2002, 2008 by the Massachusetts Institute of
 * Technology.  All Rights Reserved.
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
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
#[c2rust::src_loc = "63:1"]
pub type ap_op = libc::c_uint;
/* special handling for tgt key */
/* use master key as new key */
#[c2rust::src_loc = "66:5"]
pub const TGT_KEY: ap_op = 2;
/* setup null keys */
#[c2rust::src_loc = "65:5"]
pub const MASTER_KEY: ap_op = 1;
#[c2rust::src_loc = "64:5"]
pub const NULL_KEY: ap_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "79:8"]
pub struct iterate_args {
    pub ctx: krb5_context,
    pub rblock: *mut realm_info,
    pub dbentp: *mut krb5_db_entry,
}
#[no_mangle]
#[c2rust::src_loc = "77:3"]
pub static mut rblock: realm_info =
    realm_info{max_life: 0,
               max_rlife: 0,
               expiration: 0,
               flags: 0,
               key: 0 as *const krb5_keyblock as *mut krb5_keyblock,
               nkslist: 0,
               kslist:
                   0 as *const krb5_key_salt_tuple as
                       *mut krb5_key_salt_tuple,};
#[no_mangle]
#[c2rust::src_loc = "108:11"]
pub static mut master_salt: krb5_data =
    krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
#[no_mangle]
#[c2rust::src_loc = "110:11"]
pub static mut tgt_princ_entries: [krb5_data; 2] =
    [{
         let mut init =
             _krb5_data{magic: 0 as libc::c_int,
                        length: 6 as libc::c_int as libc::c_uint,
                        data:
                            b"krbtgt\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,};
         init
     },
     {
         let mut init =
             _krb5_data{magic: 0 as libc::c_int,
                        length: 0 as libc::c_int as libc::c_uint,
                        data: 0 as *const libc::c_char as *mut libc::c_char,};
         init
     }];
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "114:11"]
pub static mut db_creator_entries: [krb5_data; 1] =
    [krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,}; 1];
/* XXX knows about contents of krb5_principal, and that tgt names
   are of form TGT/REALM@REALM */
#[no_mangle]
#[c2rust::src_loc = "119:21"]
pub static mut tgt_princ: krb5_principal_data =
    unsafe {
        {
            let mut init =
                krb5_principal_data{magic: 0 as libc::c_int,
                                    realm:
                                        {
                                            let mut init =
                                                _krb5_data{magic:
                                                               0 as
                                                                   libc::c_int,
                                                           length:
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint,
                                                           data:
                                                               0 as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,};
                                            init
                                        },
                                    data:
                                        tgt_princ_entries.as_ptr() as *mut _,
                                    length: 2 as libc::c_int,
                                    type_0: 2 as libc::c_int,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "127:21"]
pub static mut db_create_princ: krb5_principal_data =
    unsafe {
        {
            let mut init =
                krb5_principal_data{magic: 0 as libc::c_int,
                                    realm:
                                        {
                                            let mut init =
                                                _krb5_data{magic:
                                                               0 as
                                                                   libc::c_int,
                                                           length:
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_uint,
                                                           data:
                                                               0 as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char,};
                                            init
                                        },
                                    data:
                                        db_creator_entries.as_ptr() as *mut _,
                                    length: 1 as libc::c_int,
                                    type_0: 2 as libc::c_int,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "142:1"]
pub unsafe extern "C" fn kdb5_create(mut argc: libc::c_int,
                                     mut argv: *mut *mut libc::c_char) {
    let mut optchar: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    let mut pw_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pw_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut do_stash: libc::c_int = 0 as libc::c_int;
    let mut pwd: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut seed: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut mkey_kvno: krb5_kvno = 0;
    let mut strong_random: libc::c_int = 1 as libc::c_int;
    loop  {
        optchar =
            getopt(argc, argv as *const *mut libc::c_char,
                   b"sW\x00" as *const u8 as *const libc::c_char);
        if !(optchar != -(1 as libc::c_int)) { break ; }
        match optchar {
            115 => { do_stash += 1 }
            87 => { strong_random = 0 as libc::c_int }
            63 | _ => { usage(); return }
        }
    }
    rblock.max_life = global_params.max_life;
    rblock.max_rlife = global_params.max_rlife;
    rblock.expiration = global_params.expiration;
    rblock.flags = global_params.flags;
    rblock.nkslist = global_params.num_keysalts;
    rblock.kslist = global_params.keysalts;
    log_ctx = (*util_context).kdblog_context;
    printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                    b"Loading random data\n\x00" as *const u8 as
                        *const libc::c_char));
    retval =
        krb5_c_random_os_entropy(util_context, strong_random,
                                 0 as *mut libc::c_int);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Loading random data\x00" as *const u8 as
                             *const libc::c_char));
        exit_status += 1;
        return
    }
    /* assemble & parse the master key name */
    retval =
        krb5_db_setup_mkey_name(util_context, global_params.mkey_name,
                                global_params.realm, &mut mkey_fullname,
                                &mut master_princ);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while setting up master key name\x00" as *const u8
                             as *const libc::c_char));
        exit_status += 1;
        return
    }
    db_create_princ.realm.data = global_params.realm;
    db_create_princ.realm.length =
        strlen(global_params.realm) as libc::c_uint;
    tgt_princ.realm.data = global_params.realm;
    tgt_princ.realm.length = strlen(global_params.realm) as libc::c_uint;
    let ref mut fresh0 =
        (*if (1 as libc::c_int) < tgt_princ.length {
              tgt_princ.data.offset(1 as libc::c_int as isize)
          } else { 0 as *mut krb5_data }).data;
    *fresh0 = global_params.realm;
    (*if (1 as libc::c_int) < tgt_princ.length {
          tgt_princ.data.offset(1 as libc::c_int as isize)
      } else { 0 as *mut krb5_data }).length =
        strlen(global_params.realm) as libc::c_uint;
    printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                    b"Initializing database \'%s\' for realm \'%s\',\nmaster key name \'%s\'\n\x00"
                        as *const u8 as *const libc::c_char),
           global_params.dbname, global_params.realm, mkey_fullname);
    if mkey_password.is_null() {
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"You will be prompted for the database Master Password.\n\x00"
                            as *const u8 as *const libc::c_char));
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"It is important that you NOT FORGET this password.\n\x00"
                            as *const u8 as *const libc::c_char));
        fflush(stdout);
        pw_size = 1024 as libc::c_int as libc::c_uint;
        pw_str = malloc(pw_size as libc::c_ulong) as *mut libc::c_char;
        if pw_str.is_null() {
            com_err(progname, 12 as libc::c_int as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while creating new master key\x00" as *const u8
                                 as *const libc::c_char));
            exit_status += 1;
            return
        }
        retval =
            krb5_read_password(util_context,
                               b"Enter KDC database master key\x00" as
                                   *const u8 as *const libc::c_char,
                               b"Re-enter KDC database master key to verify\x00"
                                   as *const u8 as *const libc::c_char,
                               pw_str, &mut pw_size);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while reading master key from keyboard\x00" as
                                 *const u8 as *const libc::c_char));
            exit_status += 1;
            return
        }
        mkey_password = pw_str
    }
    pwd.data = mkey_password;
    pwd.length = strlen(mkey_password) as libc::c_uint;
    retval =
        krb5_principal2salt(util_context,
                            master_princ as krb5_const_principal,
                            &mut master_salt);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while calculating master key salt\x00" as *const u8
                             as *const libc::c_char));
        exit_status += 1;
        return
    }
    retval =
        krb5_c_string_to_key(util_context, master_keyblock.enctype, &mut pwd,
                             &mut master_salt, &mut master_keyblock);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while transforming master key from password\x00" as
                             *const u8 as *const libc::c_char));
        exit_status += 1;
        return
    }
    rblock.key = &mut master_keyblock;
    seed =
        make_data(master_keyblock.contents as *mut libc::c_void,
                  master_keyblock.length);
    retval = krb5_c_random_seed(util_context, &mut seed);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing random key generator\x00" as
                             *const u8 as *const libc::c_char));
        exit_status += 1;
        return
    }
    retval = krb5_db_create(util_context, db5util_db_args);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while creating database \'%s\'\x00" as *const u8 as
                             *const libc::c_char), global_params.dbname);
        exit_status += 1;
        return
    }
    /*     if ((retval = krb5_db_fini(util_context))) { */
/*         com_err(progname, retval, "while closing current database"); */
/*         exit_status++; return; */
/*     } */
/*     if ((retval = krb5_db_open(util_context, db5util_db_args, KRB5_KDB_OPEN_RW))) { */
/*      com_err(progname, retval, "while initializing the database '%s'", */
/*              global_params.dbname); */
/*      exit_status++; return; */
/*     } */
    if !log_ctx.is_null() && (*log_ctx).iproprole as libc::c_uint != 0 {
        retval =
            ulog_map(util_context, global_params.iprop_logfile,
                     global_params.iprop_ulogsize);
        if retval != 0 {
            com_err(*argv.offset(0 as libc::c_int as isize),
                    retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while creating update log\x00" as *const u8 as
                                 *const libc::c_char));
            exit_status += 1;
            return
        }
        /*
         * We're reinitializing the update log in case one already
         * existed, but this should never happen.
         */
        retval = ulog_init_header(util_context);
        if retval != 0 {
            com_err(*argv.offset(0 as libc::c_int as isize),
                    retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while initializing update log\x00" as *const u8
                                 as *const libc::c_char));
            exit_status += 1;
            return
        }
        /*
         * Since we're creating a new db we shouldn't worry about
         * adding the initial principals since any replica might as
         * well do full resyncs from this newly created db.
         */
        (*log_ctx).iproprole = IPROP_NULL
    }
    retval =
        add_principal(util_context, master_princ, MASTER_KEY, &mut rblock);
    if retval != 0 ||
           {
               retval =
                   add_principal(util_context, &mut tgt_princ, TGT_KEY,
                                 &mut rblock);
               (retval) != 0
           } {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while adding entries to the database\x00" as
                             *const u8 as *const libc::c_char));
        exit_status += 1;
        return
    }
    /*
     * Always stash the master key so kadm5_create does not prompt for
     * it; delete the file below if it was not requested.  DO NOT EXIT
     * BEFORE DELETING THE KEYFILE if do_stash is not set.
     */
    /*
     * Determine the kvno to use, it must be that used to create the master key
     * princ.
     */
    if global_params.mask & 0x20000000 as libc::c_int as libc::c_long != 0
       { /* Default */
        mkey_kvno = global_params.kvno
    } else { mkey_kvno = 1 as libc::c_int as krb5_kvno } /* user specified */
    retval =
        krb5_db_store_master_key(util_context, global_params.stash_file,
                                 master_princ, mkey_kvno,
                                 &mut master_keyblock, mkey_password);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while storing key\x00" as *const u8 as
                             *const libc::c_char));
        printf(dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Warning: couldn\'t stash master key.\n\x00" as
                            *const u8 as *const libc::c_char));
    }
    /* clean up */
    zapfree(pw_str as *mut libc::c_void, pw_size as size_t);
    free(master_salt.data as *mut libc::c_void);
    if kadm5_create(&mut global_params) != 0 {
        if do_stash == 0 { unlink(global_params.stash_file); }
        exit_status += 1;
        return
    }
    if do_stash == 0 { unlink(global_params.stash_file); };
}
#[c2rust::src_loc = "347:1"]
unsafe extern "C" fn tgt_keysalt_iterate(mut ksent: *mut krb5_key_salt_tuple,
                                         mut ptr: krb5_pointer)
 -> krb5_error_code {
    let mut context: krb5_context = 0 as *mut _krb5_context;
    let mut kret: krb5_error_code = 0;
    let mut iargs: *mut iterate_args = 0 as *mut iterate_args;
    let mut key: krb5_keyblock =
        krb5_keyblock{magic: 0,
                      enctype: 0,
                      length: 0,
                      contents: 0 as *const krb5_octet as *mut krb5_octet,};
    let mut ind: krb5_int32 = 0;
    let mut pwd: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    iargs = ptr as *mut iterate_args;
    kret = 0 as libc::c_int;
    context = (*iargs).ctx;
    /*
     * Convert the master key password into a key for this particular
     * encryption system.
     */
    pwd.data = mkey_password; /* Default */
    pwd.length = strlen(mkey_password) as libc::c_uint; /* user specified */
    kret = krb5_c_random_seed(context, &mut pwd);
    if kret != 0 { return kret }
    kret = krb5_dbe_create_key_data((*iargs).ctx, (*iargs).dbentp);
    if kret == 0 {
        ind = (*(*iargs).dbentp).n_key_data as libc::c_int - 1 as libc::c_int;
        kret = krb5_c_make_random_key(context, (*ksent).ks_enctype, &mut key);
        if kret == 0 {
            kret =
                krb5_dbe_encrypt_key_data(context, (*(*iargs).rblock).key,
                                          &mut key, 0 as *const krb5_keysalt,
                                          1 as libc::c_int,
                                          &mut *(*(*iargs).dbentp).key_data.offset(ind
                                                                                       as
                                                                                       isize));
            krb5_free_keyblock_contents(context, &mut key);
        }
    }
    return kret;
}
#[c2rust::src_loc = "388:1"]
unsafe extern "C" fn add_principal(mut context: krb5_context,
                                   mut princ: krb5_principal, mut op: ap_op,
                                   mut pblock: *mut realm_info)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: krb5_error_code = 0;
    let mut entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut mkey_kvno: krb5_kvno = 0;
    let mut now: krb5_timestamp = 0;
    let mut iargs: iterate_args =
        iterate_args{ctx: 0 as *mut _krb5_context,
                     rblock: 0 as *mut realm_info,
                     dbentp: 0 as *mut krb5_db_entry,};
    let mut actkvno: krb5_actkvno_node =
        krb5_actkvno_node{next: 0 as *mut _krb5_actkvno_node,
                          act_kvno: 0,
                          act_time: 0,};
    entry =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<krb5_db_entry>() as libc::c_ulong) as
            *mut krb5_db_entry;
    if entry.is_null() { return 12 as libc::c_int }
    (*entry).len = 38 as libc::c_int as krb5_ui_2;
    (*entry).attributes = (*pblock).flags;
    (*entry).max_life = (*pblock).max_life;
    (*entry).max_renewable_life = (*pblock).max_rlife;
    (*entry).expiration = (*pblock).expiration;
    retval =
        krb5_copy_principal(context, princ as krb5_const_principal,
                            &mut (*entry).princ);
    if !(retval != 0) {
        retval = krb5_timeofday(context, &mut now);
        if !(retval != 0) {
            retval =
                krb5_dbe_update_mod_princ_data(context, entry, now,
                                               &mut db_create_princ as
                                                   *mut krb5_principal_data as
                                                   krb5_const_principal);
            if !(retval != 0) {
                match op as libc::c_uint {
                    1 => {
                        (*entry).key_data =
                            malloc(::std::mem::size_of::<krb5_key_data>() as
                                       libc::c_ulong) as *mut krb5_key_data;
                        if (*entry).key_data.is_null() {
                            current_block = 1898215128172248753;
                        } else {
                            memset((*entry).key_data as *mut libc::c_void,
                                   0 as libc::c_int,
                                   ::std::mem::size_of::<krb5_key_data>() as
                                       libc::c_ulong);
                            (*entry).n_key_data =
                                1 as libc::c_int as krb5_int16;
                            if global_params.mask &
                                   0x20000000 as libc::c_int as libc::c_long
                                   != 0 {
                                mkey_kvno = global_params.kvno
                            } else {
                                mkey_kvno = 1 as libc::c_int as krb5_kvno
                            }
                            (*entry).attributes |= 0x40 as libc::c_int;
                            retval =
                                krb5_dbe_encrypt_key_data(context,
                                                          (*pblock).key,
                                                          &mut master_keyblock,
                                                          0 as
                                                              *const krb5_keysalt,
                                                          mkey_kvno as
                                                              libc::c_int,
                                                          (*entry).key_data);
                            if retval != 0 { return retval }
                            /*
         * There should always be at least one "active" mkey so creating the
         * KRB5_TL_ACTKVNO entry now so the initial mkey is active.
         */
                            actkvno.next = 0 as *mut _krb5_actkvno_node;
                            actkvno.act_kvno = mkey_kvno;
                            /* earliest possible time in case system clock is set back */
                            actkvno.act_time = 0 as libc::c_int;
                            retval =
                                krb5_dbe_update_actkvno(context, entry,
                                                        &mut actkvno);
                            if retval != 0 { return retval }
                            /* so getprinc shows the right kvno */
                            retval =
                                krb5_dbe_update_mkvno(context, entry,
                                                      mkey_kvno);
                            if retval != 0 { return retval }
                            current_block = 2873832966593178012;
                        }
                    }
                    2 => {
                        iargs.ctx = context;
                        iargs.rblock = pblock;
                        iargs.dbentp = entry;
                        /*
         * Iterate through the key/salt list, ignoring salt types.
         */
                        retval =
                            krb5_keysalt_iterate((*pblock).kslist,
                                                 (*pblock).nkslist,
                                                 1 as libc::c_int as
                                                     krb5_boolean,
                                                 ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                                    ->
                                                                                        krb5_error_code>,
                                                                         Option<unsafe extern "C" fn(_:
                                                                                                         *mut krb5_key_salt_tuple,
                                                                                                     _:
                                                                                                         krb5_pointer)
                                                                                    ->
                                                                                        krb5_error_code>>(Some(::std::mem::transmute::<unsafe extern "C" fn(_:
                                                                                                                                                                *mut krb5_key_salt_tuple,
                                                                                                                                                            _:
                                                                                                                                                                krb5_pointer)
                                                                                                                                           ->
                                                                                                                                               krb5_error_code,
                                                                                                                                       unsafe extern "C" fn()
                                                                                                                                           ->
                                                                                                                                               krb5_error_code>(tgt_keysalt_iterate))),
                                                 &mut iargs as
                                                     *mut iterate_args as
                                                     krb5_pointer);
                        if retval != 0 { return retval }
                        current_block = 2873832966593178012;
                    }
                    0 => { return 95 as libc::c_int }
                    _ => { current_block = 2873832966593178012; }
                }
                match current_block {
                    1898215128172248753 => { }
                    _ => {
                        (*entry).mask =
                            (0x20000 as libc::c_int | 0x1 as libc::c_int |
                                 0x10 as libc::c_int | 0x20 as libc::c_int |
                                 0x2000 as libc::c_int |
                                 0x40000 as libc::c_int | 0x2 as libc::c_int)
                                as krb5_ui_4;
                        (*entry).attributes |= 0x800000 as libc::c_int;
                        retval = krb5_db_put_principal(context, entry)
                    }
                }
            }
        }
    }
    krb5_db_free_principal(context, entry);
    return retval;
}
unsafe extern "C" fn run_static_initializers() {
    db_creator_entries =
        [{
             let mut init =
                 _krb5_data{magic: 0 as libc::c_int,
                            length:
                                (::std::mem::size_of::<[libc::c_char; 12]>()
                                     as
                                     libc::c_ulong).wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as libc::c_uint,
                            data:
                                b"db_creation\x00" as *const u8 as
                                    *const libc::c_char as
                                    *mut libc::c_char,};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
