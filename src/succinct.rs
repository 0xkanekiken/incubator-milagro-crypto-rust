extern "C" {
    pub fn syscall_bls12381_add(p: *mut u32, q: *const u32);
}

#[inline]
pub fn bls12381_add(p: &mut [u32; 8], q: &[u32; 8]) {
    unsafe { syscall_bls12381_add(p.as_mut_ptr(), q.as_ptr()) }
}