extern "C" {
    pub fn syscall_bls12381_add(p: *mut u32, q: *const u32);
}

#[inline]
pub fn bls12381_add(p: &mut [u8; 32], q: &[u8; 32]) {
    unsafe { syscall_bls12381_add(p.as_mut_ptr() as *mut u32, q.as_ptr() as *const u32) }
}