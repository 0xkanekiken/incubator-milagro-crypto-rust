use alloc::vec::Vec;

extern "C" {
    pub fn syscall_bls12381_add(p: *mut u32, q: *const u32);
}

#[inline]
pub fn bls12381_add(p: &mut [u8; 96], q: &[u8; 96]) {
    unsafe { syscall_bls12381_add(p.as_mut_ptr() as *mut u32, q.as_ptr() as *const u32) }
}

 /// Convert a u8 array to a u32 array
 pub fn u8_to_u32(arr: Vec<u8>) -> Vec<u32> {
    let mut res = Vec::new();
    for i in 0..arr.len() / 4 {
        res.push(u32::from_le_bytes(arr[i * 4..(i + 1) * 4].try_into().unwrap()));
    }
    res
}