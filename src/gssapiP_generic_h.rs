pub type g_set_elt = *mut _g_set_elt;
use crate::src::generic::util_set::_g_set_elt;
pub type g_seqnum_state = *mut g_seqnum_state_st;
pub use crate::src::generic::util_seqstate::g_seqnum_state_st;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct g_set {
    pub mutex: crate::k5_thread_h::k5_mutex_t,
    pub data: *mut libc::c_void,
}
