use crate::*;

#[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
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

pub unsafe fn blst_p2_add_or_double_affine(
    out: *mut blst_p2,
    a: *const blst_p2,
    b: *const blst_p2_affine,
) {
    // Handle infinity cases
    if blst_p2_is_inf(a) {
        // 0 + b = b, convert affine to projective
        blst_p2_from_affine(out, b);
        return;
    }
    if blst_p2_affine_is_inf(b) {
        // a + 0 = a
        *out = *a;
        return;
    }

    // Convert a from projective to affine
    let mut a_affine: blst_p2_affine = core::mem::zeroed();
    blst_p2_to_affine(&mut a_affine, a);

    // Convert a_affine from Montgomery to standard (4 Fp elements)
    let mut std_a: [u64; 24] = [0u64; 24];
    let a_fp = &a_affine as *const blst_p2_affine as *const blst_fp;
    let std_a_fp = std_a.as_mut_ptr() as *mut blst_fp;
    for i in 0..4 {
        blst_fp_from(std_a_fp.add(i), a_fp.add(i));
    }

    // Convert b from Montgomery to standard (4 Fp elements)
    let mut std_b: [u64; 24] = [0u64; 24];
    let b_fp = b as *const blst_fp;
    let std_b_fp = std_b.as_mut_ptr() as *mut blst_fp;
    for i in 0..4 {
        blst_fp_from(std_b_fp.add(i), b_fp.add(i));
    }

    #[cfg(all(not(all(target_os = "zkvm", target_vendor = "zisk")), zisk_hints))]
    {
        let p1_arr: &[u64; 24] = &*(std_a.as_ptr() as *const [u64; 24]);
        let p2_arr: &[u64; 24] = &*(std_b.as_ptr() as *const [u64; 24]);

        ziskos::hints::hint_add_twist_bls12_381(p1_arr, p2_arr);
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation (add_twist_bls12_381_c stores result in first argument)
        let is_infinity = zisk::add_twist_bls12_381_c(std_a.as_mut_ptr(), std_b.as_ptr());

        // If result is infinity, set out to infinity (z = 0)
        if is_infinity {
            *out = core::mem::zeroed();
            return;
        }

        // Convert result from standard to Montgomery (affine)
        let mut ret_affine: blst_p2_affine = core::mem::zeroed();
        let ret_affine_fp = &mut ret_affine as *mut blst_p2_affine as *mut blst_fp;
        let std_ret_fp = std_a.as_ptr() as *const blst_fp;
        for i in 0..4 {
            blst_fp_to(ret_affine_fp.add(i), std_ret_fp.add(i));
        }

        // Convert affine back to projective
        blst_p2_from_affine(out, &ret_affine);
    }
}

pub unsafe fn blst_fp12_mul(ret: *mut blst_fp12, a: *const blst_fp12, b: *const blst_fp12) {
    // Convert a from Montgomery to standard
    let mut std_a: [u64; 72] = [0u64; 72];
    let a_fp = a as *const blst_fp;
    let std_a_fp = std_a.as_mut_ptr() as *mut blst_fp;
    for i in 0..12 {
        blst_fp_from(std_a_fp.add(i), a_fp.add(i));
    }

    // Convert b from Montgomery to standard
    let mut std_b: [u64; 72] = [0u64; 72];
    let b_fp = b as *const blst_fp;
    let std_b_fp = std_b.as_mut_ptr() as *mut blst_fp;
    for i in 0..12 {
        blst_fp_from(std_b_fp.add(i), b_fp.add(i));
    }

    #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
    {
        #[cfg(zisk_hints)]
        {
            let a_arr: &[u64; 72] = &*(std_a.as_ptr() as *const [u64; 72]);
            let b_arr: &[u64; 72] = &*(std_b.as_ptr() as *const [u64; 72]);

            ziskos::hints::hint_mul_fp12_bls12_381(a_arr, b_arr);
        }

        let _ = ret; // avoid unused variable warning
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation
        let mut std_ret: [u64; 72] = [0u64; 72];
        zisk::mul_fp12_bls12_381_c(std_ret.as_mut_ptr(), std_a.as_ptr(), std_b.as_ptr());

        // Convert result back to Montgomery form
        let ret_fp = ret as *mut blst_fp;
        let std_ret_fp = std_ret.as_ptr() as *const blst_fp;
        for i in 0..12 {
            blst_fp_to(ret_fp.add(i), std_ret_fp.add(i));
        }
    }
}

pub unsafe fn blst_p1_add_or_double_affine(
    out: *mut blst_p1,
    a: *const blst_p1,
    b: *const blst_p1_affine,
) {
    // Handle infinity cases
    if blst_p1_is_inf(a) {
        // 0 + b = b, convert affine to projective
        blst_p1_from_affine(out, b);
        return;
    }
    if blst_p1_affine_is_inf(b) {
        // a + 0 = a
        *out = *a;
        return;
    }

    // Convert a from projective to affine
    let mut a_affine: blst_p1_affine = core::mem::zeroed();
    blst_p1_to_affine(&mut a_affine, a);

    // Convert a_affine from Montgomery to standard (2 Fp elements)
    let mut std_a: [u64; 12] = [0u64; 12];
    let a_fp = &a_affine as *const blst_p1_affine as *const blst_fp;
    let std_a_fp = std_a.as_mut_ptr() as *mut blst_fp;
    for i in 0..2 {
        blst_fp_from(std_a_fp.add(i), a_fp.add(i));
    }

    // Convert b from Montgomery to standard (2 Fp elements)
    let mut std_b: [u64; 12] = [0u64; 12];
    let b_fp = b as *const blst_fp;
    let std_b_fp = std_b.as_mut_ptr() as *mut blst_fp;
    for i in 0..2 {
        blst_fp_from(std_b_fp.add(i), b_fp.add(i));
    }

    #[cfg(all(not(all(target_os = "zkvm", target_vendor = "zisk")), zisk_hints))]
    {
        let p1_arr: &[u64; 12] = &*(std_a.as_ptr() as *const [u64; 12]);
        let p2_arr: &[u64; 12] = &*(std_b.as_ptr() as *const [u64; 12]);

        ziskos::hints::hint_add_bls12_381(p1_arr, p2_arr);
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation (add_bls12_381_c stores result in first argument)
        let is_infinity = zisk::add_bls12_381_c(std_a.as_mut_ptr(), std_b.as_ptr());

        // If result is infinity, set out to infinity (z = 0)
        if is_infinity {
            *out = core::mem::zeroed();
            return;
        }

        // Convert result from standard to Montgomery (affine)
        let mut ret_affine: blst_p1_affine = core::mem::zeroed();
        let ret_affine_fp = &mut ret_affine as *mut blst_p1_affine as *mut blst_fp;
        let std_ret_fp = std_a.as_ptr() as *const blst_fp;
        for i in 0..2 {
            blst_fp_to(ret_affine_fp.add(i), std_ret_fp.add(i));
        }

        // Convert affine back to projective
        blst_p1_from_affine(out, &ret_affine);
    }
}

pub unsafe fn blst_p1_uncompress(out: *mut blst_p1_affine, in_: *const byte) -> BLST_ERROR {
    #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
    {
        #[cfg(zisk_hints)]
        {
            let input_arr: &[u8; 48] = &*(in_ as *const [u8; 48]);

            ziskos::hints::hint_decompress_bls12_381(input_arr);
        }

        let _ = out; // avoid unused variable warning
        let _ = in_; // avoid unused variable warning

        // For non-Zisk targets function is used to generate the hint, the result will be ignored
        BLST_ERROR::BLST_SUCCESS
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation (returns 0=success, 1=infinity, 2=error)
        // Output is in standard form, 12 u64s (x, y) where x,y are Fp2 elements
        let mut std_point: [u64; 12] = [0u64; 12];

        let result = zisk::decompress_bls12_381_c(std_point.as_mut_ptr(), in_);

        if result == 2 {
            return BLST_ERROR::BLST_BAD_ENCODING;
        }

        if result == 1 {
            // Result is ifinity
            *out = core::mem::zeroed();
            return BLST_ERROR::BLST_SUCCESS;
        }

        // Convert from standard form to Montgomery form
        let out_fp = out as *mut blst_fp;
        let std_fp = std_point.as_ptr() as *const blst_fp;
        for i in 0..2 {
            blst_fp_to(out_fp.add(i), std_fp.add(i));
        }

        BLST_ERROR::BLST_SUCCESS
    }
}

pub unsafe fn blst_p1_affine_on_curve(p: *const blst_p1_affine) -> bool {
    // Check for point at infinity (all zeros is valid, return true)
    if blst_p1_affine_is_inf(p) {
        return true;
    }

    // Convert from Montgomery to standard form (2 Fp elements)
    let mut std_p: [u64; 12] = [0u64; 12];
    let p_fp = p as *const blst_fp;
    let std_p_fp = std_p.as_mut_ptr() as *mut blst_fp;
    for i in 0..2 {
        blst_fp_from(std_p_fp.add(i), p_fp.add(i));
    }

    #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
    {
        #[cfg(zisk_hints)]
        {
            let p_arr: &[u64; 12] = &*(std_p.as_ptr() as *const [u64; 12]);

            ziskos::hints::hint_is_on_curve_bls12_381(p_arr);
        }

        // For non-Zisk targets function is used to generate the hint, the result will be ignored
        return true;

    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation
        zisk::is_on_curve_bls12_381_c(std_p.as_ptr())
    }
}

pub unsafe fn blst_p1_affine_in_g1(p: *const blst_p1_affine) -> bool {
    // Check for point at infinity (infinity is in G1)
    if blst_p1_affine_is_inf(p) {
        return true;
    }

    // Convert from Montgomery to standard form (2 Fp elements)
    let mut std_p: [u64; 12] = [0u64; 12];
    let p_fp = p as *const blst_fp;
    let std_p_fp = std_p.as_mut_ptr() as *mut blst_fp;
    for i in 0..2 {
        blst_fp_from(std_p_fp.add(i), p_fp.add(i));
    }

    #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
    {
        #[cfg(zisk_hints)]
        {
            let p_arr: &[u64; 12] = &*(std_p.as_ptr() as *const [u64; 12]);

            ziskos::hints::hint_is_on_subgroup_bls12_381(p_arr);
        }

        // For non-Zisk targets function is used to generate the hint, the result will be ignored
        return true;
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation
        zisk::is_on_curve_bls12_381_c(std_p.as_ptr()) && zisk::is_on_subgroup_bls12_381_c(std_p.as_ptr())
    }
}

pub unsafe fn blst_p1_mult(
    out: *mut blst_p1,
    p: *const blst_p1,
    scalar: *const byte,
    nbits: usize,
) {
    // If p is infinity (z = 0), result is infinity
    if blst_p1_is_inf(p) {
        // Set out to infinity (z = 0)
        *out = core::mem::zeroed();
        return;
    }

    // Check scalar value
    let nbytes = (nbits + 7) / 8;
    let mut scalar_is_zero = true;
    let mut scalar_is_one = *scalar == 1;
    for i in 0..nbytes.min(48) {
        let byte_val = *scalar.add(i);
        if byte_val != 0 {
            scalar_is_zero = false;
        }
        if i == 0 {
            if byte_val != 1 {
                scalar_is_one = false;
            }
        } else if byte_val != 0 {
            scalar_is_one = false;
        }
    }

    // If scalar is zero, result is infinity
    if scalar_is_zero {
        // Set out to infinity (z = 0)
        *out = core::mem::zeroed();
        return;
    }

    // If scalar is one, result is p
    if scalar_is_one {
        *out = *p;
        return;
    }

    // Otherwise, perform multiplication using Zisk implementation

    // Convert projective point to affine first
    let mut p_affine: blst_p1_affine = core::mem::zeroed();
    blst_p1_to_affine(&mut p_affine, p);

    // Convert affine point from Montgomery to standard (2 Fp elements)
    let mut std_p: [u64; 12] = [0u64; 12];
    let p_fp = &p_affine as *const blst_p1_affine as *const blst_fp;
    let std_p_fp = std_p.as_mut_ptr() as *mut blst_fp;
    for i in 0..2 {
        blst_fp_from(std_p_fp.add(i), p_fp.add(i));
    }

    // Convert scalar bytes to [u64; 6] (little-endian)
    // blst scalars are little-endian byte arrays
    let mut k: [u64; 6] = [0u64; 6];
    let k_bytes = k.as_mut_ptr() as *mut u8;
    for i in 0..nbytes.min(48) {
        *k_bytes.add(i) = *scalar.add(i);
    }

    #[cfg(all(not(all(target_os = "zkvm", target_vendor = "zisk")), zisk_hints))]
    {
        let p_arr: &[u64; 12] = &*(std_p.as_ptr() as *const [u64; 12]);
        let k_arr: &[u64; 6] = &k;

        ziskos::hints::hint_scalar_mul_bls12_381(p_arr, k_arr);
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation
        let mut std_ret: [u64; 12] = [0u64; 12];
        zisk::scalar_mul_bls12_381_c(std_ret.as_mut_ptr(), std_p.as_ptr(), k.as_ptr());

        // Convert result from standard to Montgomery (affine)
        let mut ret_affine: blst_p1_affine = core::mem::zeroed();
        let ret_affine_fp = &mut ret_affine as *mut blst_p1_affine as *mut blst_fp;
        let std_ret_fp = std_ret.as_ptr() as *const blst_fp;
        for i in 0..2 {
            blst_fp_to(ret_affine_fp.add(i), std_ret_fp.add(i));
        }

        // Convert affine back to projective
        blst_p1_from_affine(out, &ret_affine);
    }
}

pub unsafe fn blst_p2_uncompress(out: *mut blst_p2_affine, in_: *const byte) -> BLST_ERROR {
    #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
    {
        #[cfg(zisk_hints)]
        {
            let input_arr: &[u8; 96] = &*(in_ as *const [u8; 96]);

            ziskos::hints::hint_decompress_twist_bls12_381(input_arr);
        }

        let _ = out; // avoid unused variable warning
        let _ = in_; // avoid unused variable warning

        // For non-Zisk targets function is used to generate the hint, the result will be ignored
        BLST_ERROR::BLST_SUCCESS
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation (returns 0=success, 1=infinity, 2=error)
        // Output is in standard form, 24 u64s (x, y) where x,y are Fp2 elements
        let mut std_point: [u64; 24] = [0u64; 24];
        let result = zisk::decompress_twist_bls12_381_c(std_point.as_mut_ptr(), in_);

        if result == 2 {
            return BLST_ERROR::BLST_BAD_ENCODING;
        }

        if result == 1 {
            // Result is infinity
            *out = core::mem::zeroed();
            return BLST_ERROR::BLST_SUCCESS;
        }

        // Convert from standard form to Montgomery form (4 Fp elements)
        let out_fp = out as *mut blst_fp;
        let std_fp = std_point.as_ptr() as *const blst_fp;
        for i in 0..4 {
            blst_fp_to(out_fp.add(i), std_fp.add(i));
        }

        BLST_ERROR::BLST_SUCCESS
    }
}

pub unsafe fn blst_p2_affine_on_curve(p: *const blst_p2_affine) -> bool {
    // Check for point at infinity (all zeros is valid, return true)
    if blst_p2_affine_is_inf(p) {
        return true;
    }

    // Convert from Montgomery to standard form (4 Fp elements)
    let mut std_p: [u64; 24] = [0u64; 24];
    let p_fp = p as *const blst_fp;
    let std_p_fp = std_p.as_mut_ptr() as *mut blst_fp;
    for i in 0..4 {
        blst_fp_from(std_p_fp.add(i), p_fp.add(i));
    }

    #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
    {
        #[cfg(zisk_hints)]
        {
            let p_arr: &[u64; 24] = &*(std_p.as_ptr() as *const [u64; 24]);

            ziskos::hints::hint_is_on_curve_twist_bls12_381(p_arr);
        }

        // For non-Zisk targets function is used to generate the hint, the result will be ignored
        return true;
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation
        zisk::is_on_curve_twist_bls12_381_c(std_p.as_ptr())
    }
}

pub unsafe fn blst_p2_affine_in_g2(p: *const blst_p2_affine) -> bool {
    // Check for point at infinity (infinity is in G2)
    if blst_p2_affine_is_inf(p) {
        return true;
    }

    // Convert from Montgomery to standard form (4 Fp elements)
    let mut std_p: [u64; 24] = [0u64; 24];
    let p_fp = p as *const blst_fp;
    let std_p_fp = std_p.as_mut_ptr() as *mut blst_fp;
    for i in 0..4 {
        blst_fp_from(std_p_fp.add(i), p_fp.add(i));
    }

    #[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
    {
        #[cfg(zisk_hints)]
        {
            let p_arr: &[u64; 24] = &*(std_p.as_ptr() as *const [u64; 24]);

            ziskos::hints::hint_is_on_subgroup_twist_bls12_381(p_arr);
        }

        // For non-Zisk targets function is used to generate the hint, the result will be ignored
        return true;
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation
        zisk::is_on_curve_twist_bls12_381_c(std_p.as_ptr()) && zisk::is_on_subgroup_twist_bls12_381_c(std_p.as_ptr())
    }
}

pub unsafe fn blst_p2_mult(
    out: *mut blst_p2,
    p: *const blst_p2,
    scalar: *const byte,
    nbits: usize,
) {
    // If p is infinity (z = 0), result is infinity
    if blst_p2_is_inf(p) {
        // Set out to infinity (z = 0)
        *out = core::mem::zeroed();
        return;
    }

    // Check scalar value
    let nbytes = (nbits + 7) / 8;
    let mut scalar_is_zero = true;
    let mut scalar_is_one = *scalar == 1;
    for i in 0..nbytes.min(48) {
        let byte_val = *scalar.add(i);
        if byte_val != 0 {
            scalar_is_zero = false;
        }
        if i == 0 {
            if byte_val != 1 {
                scalar_is_one = false;
            }
        } else if byte_val != 0 {
            scalar_is_one = false;
        }
    }

    // If scalar is zero, result is infinity
    if scalar_is_zero {
        // Set out to infinity (z = 0)
        *out = core::mem::zeroed();
        return;
    }

    // If scalar is one, result is p
    if scalar_is_one {
        *out = *p;
        return;
    }

    // Otherwise, perform multiplication using Zisk implementation

    // Convert projective point to affine first
    let mut p_affine: blst_p2_affine = core::mem::zeroed();
    blst_p2_to_affine(&mut p_affine, p);

    // Convert affine point from Montgomery to standard (4 Fp elements for G2)
    let mut std_p: [u64; 24] = [0u64; 24];
    let p_fp = &p_affine as *const blst_p2_affine as *const blst_fp;
    let std_p_fp = std_p.as_mut_ptr() as *mut blst_fp;
    for i in 0..4 {
        blst_fp_from(std_p_fp.add(i), p_fp.add(i));
    }

    // Convert scalar bytes to [u64; 6] (little-endian)
    let mut k: [u64; 6] = [0u64; 6];
    let k_bytes = k.as_mut_ptr() as *mut u8;
    for i in 0..nbytes.min(48) {
        *k_bytes.add(i) = *scalar.add(i);
    }

    #[cfg(all(not(all(target_os = "zkvm", target_vendor = "zisk")), zisk_hints))]
    {
        let p_arr: &[u64; 24] = &*(std_p.as_ptr() as *const [u64; 24]);
        let k_arr: &[u64; 6] = &k;

        ziskos::hints::hint_scalar_mul_twist_bls12_381(p_arr, k_arr);
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation
        let mut std_ret: [u64; 24] = [0u64; 24];
        zisk::scalar_mul_twist_bls12_381_c(std_ret.as_mut_ptr(), std_p.as_ptr(), k.as_ptr());

        // Convert result from standard to Montgomery (affine)
        let mut ret_affine: blst_p2_affine = core::mem::zeroed();
        let ret_affine_fp = &mut ret_affine as *mut blst_p2_affine as *mut blst_fp;
        let std_ret_fp = std_ret.as_ptr() as *const blst_fp;
        for i in 0..4 {
            blst_fp_to(ret_affine_fp.add(i), std_ret_fp.add(i));
        }

        // Convert affine back to projective
        blst_p2_from_affine(out, &ret_affine);
    }
}

pub unsafe extern "C" fn blst_miller_loop(
    ret: *mut blst_fp12,
    Q: *const blst_p2_affine,
    P: *const blst_p1_affine,
) {
    // If P or Q is the point at infinity, result is 1
    if blst_p1_affine_is_inf(P) || blst_p2_affine_is_inf(Q) {
        let one = blst_fp12_one();
        *ret = *one;
        return;
    }

    // Convert P from Montgomery to standard
    let mut std_p: [u64; 12] = [0u64; 12];
    let p_fp = P as *const blst_fp;
    let std_p_fp = std_p.as_mut_ptr() as *mut blst_fp;
    for i in 0..2 {
        blst_fp_from(std_p_fp.add(i), p_fp.add(i));
    }

    // Convert Q from Montgomery to standard
    let mut std_q: [u64; 24] = [0u64; 24];
    let q_fp = Q as *const blst_fp;
    let std_q_fp = std_q.as_mut_ptr() as *mut blst_fp;
    for i in 0..4 {
        blst_fp_from(std_q_fp.add(i), q_fp.add(i));
    }

    #[cfg(all(not(all(target_os = "zkvm", target_vendor = "zisk")), zisk_hints))]
    {
        let p_arr: &[u64; 12] = &*(std_p.as_ptr() as *const [u64; 12]);
        let q_arr: &[u64; 24] = &*(std_q.as_ptr() as *const [u64; 24]);

        ziskos::hints::hint_miller_loop_bls12_381(p_arr, q_arr);
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call Zisk implementation
        let mut std_ret: [u64; 72] = [0u64; 72];
        zisk::miller_loop_bls12_381_c(std_ret.as_mut_ptr(), std_q.as_ptr(), std_p.as_ptr());

        // Convert result back to Montgomery form
        let ret_fp = ret as *mut blst_fp;
        let std_ret_fp = std_ret.as_ptr() as *const blst_fp;
        for i in 0..12 {
            blst_fp_to(ret_fp.add(i), std_ret_fp.add(i));
        }
    }
}

pub unsafe fn blst_final_exp(ret: *mut blst_fp12, f: *const blst_fp12) {
    // If f is 1, result is 1
    if blst_fp12_is_one(f) {
        let one = blst_fp12_one();
        *ret = *one;
        return;
    }

    // Convert from Montgomery to standard form
    let mut std_input: [u64; 72] = [0u64; 72];
    let f_ptr = f as *const blst_fp;
    let std_ptr = std_input.as_mut_ptr() as *mut blst_fp;

    for i in 0..12 {
        blst_fp_from(std_ptr.add(i), f_ptr.add(i));
    }

    #[cfg(all(not(all(target_os = "zkvm", target_vendor = "zisk")), zisk_hints))]
    {
        let f_arr: &[u64; 72] = &*(std_input.as_ptr() as *const [u64; 72]);

        ziskos::hints::hint_final_exp_bls12_381(f_arr);
    }

    #[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
    {
        // Call ZisK implementation
        zisk::final_exp_bls12_381_c(std_input.as_mut_ptr());

        // Convert result back to Montgomery form
        let ret_fp = ret as *mut blst_fp;
        let std_fp = std_input.as_ptr() as *const blst_fp;

        for i in 0..12 {
            blst_fp_to(ret_fp.add(i), std_fp.add(i));
        }
    }
}
