
extern "C" {
    // Field operations
    pub fn mul_fp12_bls12_381_c(a_ptr: *const u64, b_ptr: *const u64, result_ptr: *mut u64);

    // Curve operations
    pub fn decompress_bls12_381_c(input_ptr: *const u8, result_ptr: *mut u64) -> u8;
    pub fn is_on_curve_bls12_381_c(p_ptr: *const u64) -> u8;
    pub fn is_on_subgroup_bls12_381_c(p_ptr: *const u64) -> u8;
    pub fn add_bls12_381_c(p1_ptr: *const u64, p2_ptr: *const u64, result_ptr: *mut u64) -> u8;
    pub fn scalar_mul_bls12_381_c(p_ptr: *const u64, k_ptr: *const u64, result_ptr: *mut u64) -> u8;

    // Twist operations
    pub fn decompress_twist_bls12_381_c(input_ptr: *const u8, result_ptr: *mut u64) -> u8;
    pub fn is_on_curve_twist_bls12_381_c(p_ptr: *const u64) -> u8;
    pub fn is_on_subgroup_twist_bls12_381_c(p_ptr: *const u64) -> u8;
    pub fn add_twist_bls12_381_c(p1_ptr: *const u64, p2_ptr: *const u64, result_ptr: *mut u64) -> u8;
    pub fn scalar_mul_twist_bls12_381_c(p_ptr: *const u64, k_ptr: *const u64, result_ptr: *mut u64) -> u8;

    // Pairing operations
    pub fn miller_loop_bls12_381_c(p_ptr: *const u64, q_ptr: *const u64, result_ptr: *mut u64);
    pub fn final_exp_bls12_381_c(f_ptr: *const u64, result_ptr: *mut u64);
}