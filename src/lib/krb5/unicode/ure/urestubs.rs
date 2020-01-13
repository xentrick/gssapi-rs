use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:42"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:42"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:42"]
pub mod krb5_h {
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    use super::stdint_uintn_h::uint32_t;
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/unicode/ure/ure.h:44"]
pub mod ure_h {
    /*
 * Copyright 1998-2008 The OpenLDAP Foundation.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted only as authorized by the OpenLDAP
 * Public License.
 *
 * A copy of this license is available in file LICENSE in the
 * top-level directory of the distribution or, alternatively, at
 * <http://www.OpenLDAP.org/license.html>.
 */
/* Copyright 1997, 1998, 1999 Computing Research Labs,
 * New Mexico State University
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COMPUTING RESEARCH LAB OR NEW MEXICO STATE UNIVERSITY BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT
 * OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR
 * THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
    /*
 * This work is part of OpenLDAP Software <http://www.openldap.org/>.
 * $OpenLDAP: pkg/ldap/libraries/liblunicode/ure/ure.h,v 1.15 2008/01/07 23:20:05 kurt Exp $
 * $Id: ure.h,v 1.2 1999/09/21 15:47:44 mleisher Exp $
 */
    /*
 * Set of character class flags.
 */
    /*
 * Error codes.
 */
    /*
 * Options that can be combined for searching.
 */
    #[c2rust::src_loc = "104:1"]
    pub type ucs4_t = krb5_ui_4;
    use super::krb5_h::krb5_ui_4;
    /* _h_ure */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/krb5/unicode/ucdata/ucdata.h:46"]
pub mod ucdata_h {
    use super::krb5_h::krb5_ui_4;
    extern "C" {
        /* Other Neutrals             */
        /*
 * Implementation specific character properties.
 */
        /* Composite                  */
        /* Non-Breaking               */
        /* Symmetric                  */
        /* Hex Digit                  */
        /* Quote Mark                 */
        /* Mirroring                  */
        /* Space, other               */
        /* Defined                    */
        /*
 * Added for UnicodeData-2.1.3.
 */
        /* Punctuation, Initial       */
        /* Punctuation, Final         */
        /*
 * This is the primary function for testing to see if a character has some set
 * of properties.  The macros that test for various character properties all
 * call this function with some set of masks.
 */
        #[no_mangle]
        #[c2rust::src_loc = "123:1"]
        pub fn ucisprop(code: krb5_ui_4, mask1: krb5_ui_4, mask2: krb5_ui_4)
         -> libc::c_int;
        /*
 * Directionality macros.
 */
        /*
 * Other macros inspired by John Cowan.
 */
        /*
 * Other miscellaneous character property macros.
 */
        /* *************************************************************************
 *
 * Functions for case conversion.
 *
 **************************************************************************/
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn uctoupper(code: krb5_ui_4) -> krb5_ui_4;
    }
    /* _h_ucdata */
}
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::krb5_h::krb5_ui_4;
pub use self::ure_h::ucs4_t;
use self::ucdata_h::{ucisprop, uctoupper};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "62:15"]
pub struct ucmaskmap {
    pub mask1: libc::c_ulong,
    pub mask2: libc::c_ulong,
}
/* ************************************************************************
 *
 * Prototypes for stub functions used for URE.  These need to be rewritten to
 * use the Unicode support available on the system.
 *
 *************************************************************************/
/*
 * Copyright 1998-2008 The OpenLDAP Foundation.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted only as authorized by the OpenLDAP
 * Public License.
 *
 * A copy of this license is available in file LICENSE in the
 * top-level directory of the distribution or, alternatively, at
 * <https://www.OpenLDAP.org/license.html>.
 */
/*
 * Copyright 1997, 1998, 1999 Computing Research Labs,
 * New Mexico State University
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COMPUTING RESEARCH LAB OR NEW MEXICO STATE UNIVERSITY BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT
 * OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR
 * THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
/*
 * This work is part of OpenLDAP Software <https://www.openldap.org/>.
 * $OpenLDAP: pkg/ldap/libraries/liblunicode/ure/urestubs.c,v 1.16 2008/01/07 23:20:05 kurt Exp $
 * $Id: urestubs.c,v 1.2 1999/09/21 15:47:44 mleisher Exp $"
 */
/*
 * This file contains stub routines needed by the URE package to test
 * character properties and other Unicode implementation specific details.
 */
/*
 * This routine should return the lower case equivalent for the character or,
 * if there is no lower case quivalent, the character itself.
 */
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub unsafe extern "C" fn _ure_tolower(mut c: ucs4_t) -> ucs4_t {
    return uctoupper(c);
}
#[c2rust::src_loc = "65:3"]
static mut masks: [ucmaskmap; 32] =
    [{
         let mut init =
             ucmaskmap{mask1: 0x1 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x2 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x8 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1:
                           (0x10 as libc::c_int | 0x20 as libc::c_int) as
                               libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x40 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x80 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x100 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x200 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x1000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x4000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x8000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x10000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x20000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x40000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x100000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x200000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x80000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x800000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x1000000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x2000000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x8000000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x10000000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0x20000000 as libc::c_int as libc::c_ulong,
                       mask2: 0 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0 as libc::c_int as libc::c_ulong,
                       mask2: 0x40000000 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0 as libc::c_int as libc::c_ulong,
                       mask2: 0x80000000 as libc::c_uint as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0 as libc::c_int as libc::c_ulong,
                       mask2: 0x1 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0 as libc::c_int as libc::c_ulong,
                       mask2: 0x2 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0 as libc::c_int as libc::c_ulong,
                       mask2: 0x4 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0 as libc::c_int as libc::c_ulong,
                       mask2: 0x8 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0 as libc::c_int as libc::c_ulong,
                       mask2: 0x10 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0 as libc::c_int as libc::c_ulong,
                       mask2: 0x20 as libc::c_int as libc::c_ulong,};
         init
     },
     {
         let mut init =
             ucmaskmap{mask1: 0 as libc::c_int as libc::c_ulong,
                       mask2: 0x40 as libc::c_int as libc::c_ulong,};
         init
     }];
/*
 * This routine takes a set of URE character property flags (see ure.h) along
 * with a character and tests to see if the character has one or more of those
 * properties.
 */
#[no_mangle]
#[c2rust::src_loc = "111:1"]
pub unsafe extern "C" fn _ure_matches_properties(mut props: libc::c_ulong,
                                                 mut c: ucs4_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut mask1: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut mask2: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if props & ((1 as libc::c_int) << i) as libc::c_ulong != 0 {
            mask1 |= masks[i as usize].mask1;
            mask2 |= masks[i as usize].mask2
        }
        i += 1
    }
    return ucisprop(mask1 as krb5_ui_4, mask2 as krb5_ui_4, c);
}
