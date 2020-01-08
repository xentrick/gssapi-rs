use crate::src::generic::util_errmap::mecherror;

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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mecherrmap__pair {
    pub l: crate::gssapi_h::OM_uint32,
    pub r: mecherror,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mecherrmap {
    pub a: crate::errmap_h::mecherrmap__pairarray,
    pub nextidx: isize,
}
pub type mecherrmap__pairarray = crate::errmap_h::mecherrmap__pairarray__header;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mecherrmap__pairarray__header {
    pub allocated: crate::stddef_h::size_t,
    pub elts: *mut crate::errmap_h::mecherrmap__pair,
}
/* end of t_array template */

/* start of t_bimap body template */

/* for use in cases where text substitutions may not work, like putting
"const" before a type that turns out to be "char *"  */
pub type mecherrmap__left_t = crate::gssapi_h::OM_uint32;
