/* _GSSAPIP_GENERIC_H_ */
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::uint32_t;
/* buffer */
/* #pragma ident	"@(#)g_rel_oid_set.c	1.12	97/11/11 SMI" */
/*
 * Copyright 1996 by Sun Microsystems, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of Sun Microsystems not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. Sun Microsystems makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * SUN MICROSYSTEMS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL SUN MICROSYSTEMS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 *  glue routine for gss_release_oid_set
 */
#[no_mangle]

pub unsafe extern "C" fn gss_release_oid_set(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut set: *mut crate::gssapi_h::gss_OID_set,
) -> crate::gssapi_h::OM_uint32 {
    return crate::src::generic::rel_oid_set::generic_gss_release_oid_set(minor_status, set);
}
