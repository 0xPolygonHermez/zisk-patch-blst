
extern "C" {
    pub fn mul_fp12_bls12_381_c(ret: *mut u64, a: *const u64, b: *const u64);

    pub fn decompress_bls12_381_c(ret: *mut u64, input: *const u8) -> u8;
    pub fn is_on_curve_bls12_381_c(p: *const u64) -> bool;
    pub fn is_on_subgroup_bls12_381_c(p: *const u64) -> bool;
    pub fn add_bls12_381_c(p1: *mut u64, p2: *const u64) -> bool;
    pub fn scalar_mul_bls12_381_c(ret: *mut u64, p: *const u64, k: *const u64);

    pub fn decompress_twist_bls12_381_c(ret: *mut u64, input: *const u8) -> u8;
    pub fn is_on_curve_twist_bls12_381_c(p: *const u64) -> bool;
    pub fn is_on_subgroup_twist_bls12_381_c(p: *const u64) -> bool;

    pub fn add_twist_bls12_381_c(p1: *mut u64, p2: *const u64) -> bool;
    pub fn scalar_mul_twist_bls12_381_c(ret: *mut u64, p: *const u64, k: *const u64);

    pub fn miller_loop_bls12_381_c( ret: *mut u64, q: *const u64, p: *const u64);
    pub fn final_exp_bls12_381_c(f: *mut u64);
}