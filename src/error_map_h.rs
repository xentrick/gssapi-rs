pub type gsserrmap = crate::error_map_h::gsserrmap__head;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsserrmap__head {
    pub first: *mut crate::error_map_h::gsserrmap__element,
}
/*
 * This file is generated, please don't edit it.
 * script: ../../../util/gen-map.pl
 * args:
 *	-oerror_map.new
 *	NAME=gsserrmap
 *	KEY=OM_uint32
 *	VALUE=char *
 *	COMPARE=compare_OM_uint32
 *	FREEVALUE=free_string
 * The rest of this file is copied from a template, with
 * substitutions.  See the template for copyright info.
 */

/*
 * map, generated from template
 * map name: gsserrmap
 * key: OM_uint32
 * value: char *
 * compare: compare_OM_uint32
 * copy_key: 0
 * free_key: 0
 * free_value: free_string
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsserrmap__element {
    pub key: crate::gssapi_h::OM_uint32,
    pub value: *mut i8,
    pub next: *mut crate::error_map_h::gsserrmap__element,
}
