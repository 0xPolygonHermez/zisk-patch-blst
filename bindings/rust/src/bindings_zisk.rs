// Same as bindings, but with patched functions

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BLST_ERROR {
    BLST_SUCCESS = 0,
    BLST_BAD_ENCODING = 1,
    BLST_POINT_NOT_ON_CURVE = 2,
    BLST_POINT_NOT_IN_GROUP = 3,
    BLST_AGGR_TYPE_MISMATCH = 4,
    BLST_VERIFY_FAIL = 5,
    BLST_PK_IS_INFINITY = 6,
    BLST_BAD_SCALAR = 7,
}
pub type byte = u8;
pub type limb_t = u64;
#[repr(C)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Zeroize)]
#[zeroize(drop)]
pub struct blst_scalar {
    pub b: [byte; 32usize],
}
#[test]
fn bindgen_test_layout_blst_scalar() {
    const UNINIT: ::core::mem::MaybeUninit<blst_scalar> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_scalar>(),
        32usize,
        concat!("Size of: ", stringify!(blst_scalar))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_scalar>(),
        1usize,
        concat!("Alignment of ", stringify!(blst_scalar))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_scalar),
            "::",
            stringify!(b)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct blst_fr {
    pub l: [limb_t; 4usize],
}
#[test]
fn bindgen_test_layout_blst_fr() {
    const UNINIT: ::core::mem::MaybeUninit<blst_fr> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_fr>(),
        32usize,
        concat!("Size of: ", stringify!(blst_fr))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_fr>(),
        8usize,
        concat!("Alignment of ", stringify!(blst_fr))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).l) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_fr),
            "::",
            stringify!(l)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct blst_fp {
    pub l: [limb_t; 6usize],
}
#[test]
fn bindgen_test_layout_blst_fp() {
    const UNINIT: ::core::mem::MaybeUninit<blst_fp> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_fp>(),
        48usize,
        concat!("Size of: ", stringify!(blst_fp))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_fp>(),
        8usize,
        concat!("Alignment of ", stringify!(blst_fp))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).l) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_fp),
            "::",
            stringify!(l)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct blst_fp2 {
    pub fp: [blst_fp; 2usize],
}
#[test]
fn bindgen_test_layout_blst_fp2() {
    const UNINIT: ::core::mem::MaybeUninit<blst_fp2> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_fp2>(),
        96usize,
        concat!("Size of: ", stringify!(blst_fp2))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_fp2>(),
        8usize,
        concat!("Alignment of ", stringify!(blst_fp2))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).fp) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_fp2),
            "::",
            stringify!(fp)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct blst_fp6 {
    pub fp2: [blst_fp2; 3usize],
}
#[test]
fn bindgen_test_layout_blst_fp6() {
    const UNINIT: ::core::mem::MaybeUninit<blst_fp6> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_fp6>(),
        288usize,
        concat!("Size of: ", stringify!(blst_fp6))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_fp6>(),
        8usize,
        concat!("Alignment of ", stringify!(blst_fp6))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).fp2) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_fp6),
            "::",
            stringify!(fp2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq)]
pub struct blst_fp12 {
    pub fp6: [blst_fp6; 2usize],
}
#[test]
fn bindgen_test_layout_blst_fp12() {
    const UNINIT: ::core::mem::MaybeUninit<blst_fp12> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_fp12>(),
        576usize,
        concat!("Size of: ", stringify!(blst_fp12))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_fp12>(),
        8usize,
        concat!("Alignment of ", stringify!(blst_fp12))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).fp6) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_fp12),
            "::",
            stringify!(fp6)
        )
    );
}
extern "C" {
    pub fn blst_scalar_from_uint32(out: *mut blst_scalar, a: *const u32);
}
extern "C" {
    pub fn blst_uint32_from_scalar(out: *mut u32, a: *const blst_scalar);
}
extern "C" {
    pub fn blst_scalar_from_uint64(out: *mut blst_scalar, a: *const u64);
}
extern "C" {
    pub fn blst_uint64_from_scalar(out: *mut u64, a: *const blst_scalar);
}
extern "C" {
    pub fn blst_scalar_from_bendian(out: *mut blst_scalar, a: *const byte);
}
extern "C" {
    pub fn blst_bendian_from_scalar(out: *mut byte, a: *const blst_scalar);
}
extern "C" {
    pub fn blst_scalar_from_lendian(out: *mut blst_scalar, a: *const byte);
}
extern "C" {
    pub fn blst_lendian_from_scalar(out: *mut byte, a: *const blst_scalar);
}
extern "C" {
    pub fn blst_scalar_fr_check(a: *const blst_scalar) -> bool;
}
extern "C" {
    pub fn blst_sk_check(a: *const blst_scalar) -> bool;
}
extern "C" {
    pub fn blst_sk_add_n_check(
        out: *mut blst_scalar,
        a: *const blst_scalar,
        b: *const blst_scalar,
    ) -> bool;
}
extern "C" {
    pub fn blst_sk_sub_n_check(
        out: *mut blst_scalar,
        a: *const blst_scalar,
        b: *const blst_scalar,
    ) -> bool;
}
extern "C" {
    pub fn blst_sk_mul_n_check(
        out: *mut blst_scalar,
        a: *const blst_scalar,
        b: *const blst_scalar,
    ) -> bool;
}
extern "C" {
    pub fn blst_sk_inverse(out: *mut blst_scalar, a: *const blst_scalar);
}
extern "C" {
    pub fn blst_scalar_from_le_bytes(out: *mut blst_scalar, in_: *const byte, len: usize) -> bool;
}
extern "C" {
    pub fn blst_scalar_from_be_bytes(out: *mut blst_scalar, in_: *const byte, len: usize) -> bool;
}
extern "C" {
    pub fn blst_fr_add(ret: *mut blst_fr, a: *const blst_fr, b: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_sub(ret: *mut blst_fr, a: *const blst_fr, b: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_mul_by_3(ret: *mut blst_fr, a: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_lshift(ret: *mut blst_fr, a: *const blst_fr, count: usize);
}
extern "C" {
    pub fn blst_fr_rshift(ret: *mut blst_fr, a: *const blst_fr, count: usize);
}
extern "C" {
    pub fn blst_fr_mul(ret: *mut blst_fr, a: *const blst_fr, b: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_sqr(ret: *mut blst_fr, a: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_cneg(ret: *mut blst_fr, a: *const blst_fr, flag: bool);
}
extern "C" {
    pub fn blst_fr_eucl_inverse(ret: *mut blst_fr, a: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_inverse(ret: *mut blst_fr, a: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_from_uint64(ret: *mut blst_fr, a: *const u64);
}
extern "C" {
    pub fn blst_uint64_from_fr(ret: *mut u64, a: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_from_scalar(ret: *mut blst_fr, a: *const blst_scalar);
}
extern "C" {
    pub fn blst_scalar_from_fr(ret: *mut blst_scalar, a: *const blst_fr);
}
extern "C" {
    pub fn blst_fp_add(ret: *mut blst_fp, a: *const blst_fp, b: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_sub(ret: *mut blst_fp, a: *const blst_fp, b: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_mul_by_3(ret: *mut blst_fp, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_mul_by_8(ret: *mut blst_fp, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_lshift(ret: *mut blst_fp, a: *const blst_fp, count: usize);
}
extern "C" {
    pub fn blst_fp_mul(ret: *mut blst_fp, a: *const blst_fp, b: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_sqr(ret: *mut blst_fp, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_cneg(ret: *mut blst_fp, a: *const blst_fp, flag: bool);
}
extern "C" {
    pub fn blst_fp_eucl_inverse(ret: *mut blst_fp, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_inverse(ret: *mut blst_fp, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_sqrt(ret: *mut blst_fp, a: *const blst_fp) -> bool;
}
extern "C" {
    pub fn blst_fp_from_uint32(ret: *mut blst_fp, a: *const u32);
}
extern "C" {
    pub fn blst_uint32_from_fp(ret: *mut u32, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_from_uint64(ret: *mut blst_fp, a: *const u64);
}
extern "C" {
    pub fn blst_uint64_from_fp(ret: *mut u64, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_from_bendian(ret: *mut blst_fp, a: *const byte);
}
extern "C" {
    pub fn blst_bendian_from_fp(ret: *mut byte, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_from_lendian(ret: *mut blst_fp, a: *const byte);
}
extern "C" {
    pub fn blst_lendian_from_fp(ret: *mut byte, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp2_add(ret: *mut blst_fp2, a: *const blst_fp2, b: *const blst_fp2);
}
extern "C" {
    pub fn blst_fp2_sub(ret: *mut blst_fp2, a: *const blst_fp2, b: *const blst_fp2);
}
extern "C" {
    pub fn blst_fp2_mul_by_3(ret: *mut blst_fp2, a: *const blst_fp2);
}
extern "C" {
    pub fn blst_fp2_mul_by_8(ret: *mut blst_fp2, a: *const blst_fp2);
}
extern "C" {
    pub fn blst_fp2_lshift(ret: *mut blst_fp2, a: *const blst_fp2, count: usize);
}
extern "C" {
    pub fn blst_fp2_mul(ret: *mut blst_fp2, a: *const blst_fp2, b: *const blst_fp2);
}
extern "C" {
    pub fn blst_fp2_sqr(ret: *mut blst_fp2, a: *const blst_fp2);
}
extern "C" {
    pub fn blst_fp2_cneg(ret: *mut blst_fp2, a: *const blst_fp2, flag: bool);
}
extern "C" {
    pub fn blst_fp2_eucl_inverse(ret: *mut blst_fp2, a: *const blst_fp2);
}
extern "C" {
    pub fn blst_fp2_inverse(ret: *mut blst_fp2, a: *const blst_fp2);
}
extern "C" {
    pub fn blst_fp2_sqrt(ret: *mut blst_fp2, a: *const blst_fp2) -> bool;
}
extern "C" {
    pub fn blst_fp12_sqr(ret: *mut blst_fp12, a: *const blst_fp12);
}
extern "C" {
    pub fn blst_fp12_cyclotomic_sqr(ret: *mut blst_fp12, a: *const blst_fp12);
}
// extern "C" {
//     pub fn blst_fp12_mul(ret: *mut blst_fp12, a: *const blst_fp12, b: *const blst_fp12);
// }
pub unsafe extern "C" fn blst_fp12_mul(ret: *mut blst_fp12, a: *const blst_fp12, b: *const blst_fp12) {
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
extern "C" {
    pub fn blst_fp12_mul_by_xy00z0(
        ret: *mut blst_fp12,
        a: *const blst_fp12,
        xy00z0: *const blst_fp6,
    );
}
extern "C" {
    pub fn blst_fp12_conjugate(a: *mut blst_fp12);
}
extern "C" {
    pub fn blst_fp12_inverse(ret: *mut blst_fp12, a: *const blst_fp12);
}
extern "C" {
    pub fn blst_fp12_frobenius_map(ret: *mut blst_fp12, a: *const blst_fp12, n: usize);
}
extern "C" {
    pub fn blst_fp12_is_equal(a: *const blst_fp12, b: *const blst_fp12) -> bool;
}
extern "C" {
    pub fn blst_fp12_is_one(a: *const blst_fp12) -> bool;
}
extern "C" {
    pub fn blst_fp12_in_group(a: *const blst_fp12) -> bool;
}
extern "C" {
    pub fn blst_fp12_one() -> *const blst_fp12;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq)]
pub struct blst_p1 {
    pub x: blst_fp,
    pub y: blst_fp,
    pub z: blst_fp,
}
#[test]
fn bindgen_test_layout_blst_p1() {
    const UNINIT: ::core::mem::MaybeUninit<blst_p1> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_p1>(),
        144usize,
        concat!("Size of: ", stringify!(blst_p1))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_p1>(),
        8usize,
        concat!("Alignment of ", stringify!(blst_p1))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p1),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p1),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p1),
            "::",
            stringify!(z)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq)]
pub struct blst_p1_affine {
    pub x: blst_fp,
    pub y: blst_fp,
}
#[test]
fn bindgen_test_layout_blst_p1_affine() {
    const UNINIT: ::core::mem::MaybeUninit<blst_p1_affine> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_p1_affine>(),
        96usize,
        concat!("Size of: ", stringify!(blst_p1_affine))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_p1_affine>(),
        8usize,
        concat!("Alignment of ", stringify!(blst_p1_affine))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p1_affine),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p1_affine),
            "::",
            stringify!(y)
        )
    );
}
extern "C" {
    pub fn blst_p1_add(out: *mut blst_p1, a: *const blst_p1, b: *const blst_p1);
}
extern "C" {
    pub fn blst_p1_add_or_double(out: *mut blst_p1, a: *const blst_p1, b: *const blst_p1);
}
extern "C" {
    pub fn blst_p1_add_affine(out: *mut blst_p1, a: *const blst_p1, b: *const blst_p1_affine);
}
// extern "C" {
//     pub fn blst_p1_add_or_double_affine(
//         out: *mut blst_p1,
//         a: *const blst_p1,
//         b: *const blst_p1_affine,
//     );
// }
pub unsafe extern "C" fn blst_p1_add_or_double_affine(
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
extern "C" {
    pub fn blst_p1_double(out: *mut blst_p1, a: *const blst_p1);
}
// extern "C" {
//     pub fn blst_p1_mult(out: *mut blst_p1, p: *const blst_p1, scalar: *const byte, nbits: usize);
// }
pub unsafe extern "C" fn blst_p1_mult(
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
extern "C" {
    pub fn blst_p1_cneg(p: *mut blst_p1, cbit: bool);
}
extern "C" {
    pub fn blst_p1_to_affine(out: *mut blst_p1_affine, in_: *const blst_p1);
}
extern "C" {
    pub fn blst_p1_from_affine(out: *mut blst_p1, in_: *const blst_p1_affine);
}
extern "C" {
    pub fn blst_p1_on_curve(p: *const blst_p1) -> bool;
}
extern "C" {
    pub fn blst_p1_in_g1(p: *const blst_p1) -> bool;
}
extern "C" {
    pub fn blst_p1_is_equal(a: *const blst_p1, b: *const blst_p1) -> bool;
}
extern "C" {
    pub fn blst_p1_is_inf(a: *const blst_p1) -> bool;
}
extern "C" {
    pub fn blst_p1_generator() -> *const blst_p1;
}
// extern "C" {
//     pub fn blst_p1_affine_on_curve(p: *const blst_p1_affine) -> bool;
// }
pub unsafe extern "C" fn blst_p1_affine_on_curve(p: *const blst_p1_affine) -> bool {
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
    
    // Call Zisk implementation
    zisk::is_on_curve_bls12_381_c(std_p.as_ptr())
}
// extern "C" {
//     pub fn blst_p1_affine_in_g1(p: *const blst_p1_affine) -> bool;
// }
pub unsafe extern "C" fn blst_p1_affine_in_g1(p: *const blst_p1_affine) -> bool {
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
    
    // Call Zisk implementation
    zisk::is_on_curve_bls12_381_c(std_p.as_ptr()) && zisk::is_on_subgroup_bls12_381_c(std_p.as_ptr())
}
extern "C" {
    pub fn blst_p1_affine_is_equal(a: *const blst_p1_affine, b: *const blst_p1_affine) -> bool;
}
extern "C" {
    pub fn blst_p1_affine_is_inf(a: *const blst_p1_affine) -> bool;
}
extern "C" {
    pub fn blst_p1_affine_generator() -> *const blst_p1_affine;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq)]
pub struct blst_p2 {
    pub x: blst_fp2,
    pub y: blst_fp2,
    pub z: blst_fp2,
}
#[test]
fn bindgen_test_layout_blst_p2() {
    const UNINIT: ::core::mem::MaybeUninit<blst_p2> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_p2>(),
        288usize,
        concat!("Size of: ", stringify!(blst_p2))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_p2>(),
        8usize,
        concat!("Alignment of ", stringify!(blst_p2))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p2),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p2),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p2),
            "::",
            stringify!(z)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Eq)]
pub struct blst_p2_affine {
    pub x: blst_fp2,
    pub y: blst_fp2,
}
#[test]
fn bindgen_test_layout_blst_p2_affine() {
    const UNINIT: ::core::mem::MaybeUninit<blst_p2_affine> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<blst_p2_affine>(),
        192usize,
        concat!("Size of: ", stringify!(blst_p2_affine))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_p2_affine>(),
        8usize,
        concat!("Alignment of ", stringify!(blst_p2_affine))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p2_affine),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(blst_p2_affine),
            "::",
            stringify!(y)
        )
    );
}
extern "C" {
    pub fn blst_p2_add(out: *mut blst_p2, a: *const blst_p2, b: *const blst_p2);
}
extern "C" {
    pub fn blst_p2_add_or_double(out: *mut blst_p2, a: *const blst_p2, b: *const blst_p2);
}
extern "C" {
    pub fn blst_p2_add_affine(out: *mut blst_p2, a: *const blst_p2, b: *const blst_p2_affine);
}
// extern "C" {
//     pub fn blst_p2_add_or_double_affine(
//         out: *mut blst_p2,
//         a: *const blst_p2,
//         b: *const blst_p2_affine,
//     );
// }
pub unsafe extern "C" fn blst_p2_add_or_double_affine(
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
extern "C" {
    pub fn blst_p2_double(out: *mut blst_p2, a: *const blst_p2);
}
// extern "C" {
//     pub fn blst_p2_mult(out: *mut blst_p2, p: *const blst_p2, scalar: *const byte, nbits: usize);
// }
pub unsafe extern "C" fn blst_p2_mult(
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
extern "C" {
    pub fn blst_p2_cneg(p: *mut blst_p2, cbit: bool);
}
extern "C" {
    pub fn blst_p2_to_affine(out: *mut blst_p2_affine, in_: *const blst_p2);
}
extern "C" {
    pub fn blst_p2_from_affine(out: *mut blst_p2, in_: *const blst_p2_affine);
}
extern "C" {
    pub fn blst_p2_on_curve(p: *const blst_p2) -> bool;
}
extern "C" {
    pub fn blst_p2_in_g2(p: *const blst_p2) -> bool;
}
extern "C" {
    pub fn blst_p2_is_equal(a: *const blst_p2, b: *const blst_p2) -> bool;
}
extern "C" {
    pub fn blst_p2_is_inf(a: *const blst_p2) -> bool;
}
extern "C" {
    pub fn blst_p2_generator() -> *const blst_p2;
}
// extern "C" {
//     pub fn blst_p2_affine_on_curve(p: *const blst_p2_affine) -> bool;
// }
pub unsafe extern "C" fn blst_p2_affine_on_curve(p: *const blst_p2_affine) -> bool {
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
    
    // Call Zisk implementation
    zisk::is_on_curve_twist_bls12_381_c(std_p.as_ptr())
}
// extern "C" {
//     pub fn blst_p2_affine_in_g2(p: *const blst_p2_affine) -> bool;
// }
pub unsafe extern "C" fn blst_p2_affine_in_g2(p: *const blst_p2_affine) -> bool {
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
    
    // Call Zisk implementation
    zisk::is_on_curve_twist_bls12_381_c(std_p.as_ptr()) && zisk::is_on_subgroup_twist_bls12_381_c(std_p.as_ptr())
}
extern "C" {
    pub fn blst_p2_affine_is_equal(a: *const blst_p2_affine, b: *const blst_p2_affine) -> bool;
}
extern "C" {
    pub fn blst_p2_affine_is_inf(a: *const blst_p2_affine) -> bool;
}
extern "C" {
    pub fn blst_p2_affine_generator() -> *const blst_p2_affine;
}
extern "C" {
    pub fn blst_p1s_to_affine(
        dst: *mut blst_p1_affine,
        points: *const *const blst_p1,
        npoints: usize,
    );
}
extern "C" {
    pub fn blst_p1s_add(ret: *mut blst_p1, points: *const *const blst_p1_affine, npoints: usize);
}
extern "C" {
    pub fn blst_p1s_mult_wbits_precompute_sizeof(wbits: usize, npoints: usize) -> usize;
}
extern "C" {
    pub fn blst_p1s_mult_wbits_precompute(
        table: *mut blst_p1_affine,
        wbits: usize,
        points: *const *const blst_p1_affine,
        npoints: usize,
    );
}
extern "C" {
    pub fn blst_p1s_mult_wbits_scratch_sizeof(npoints: usize) -> usize;
}
extern "C" {
    pub fn blst_p1s_mult_wbits(
        ret: *mut blst_p1,
        table: *const blst_p1_affine,
        wbits: usize,
        npoints: usize,
        scalars: *const *const byte,
        nbits: usize,
        scratch: *mut limb_t,
    );
}
extern "C" {
    pub fn blst_p1s_mult_pippenger_scratch_sizeof(npoints: usize) -> usize;
}
extern "C" {
    pub fn blst_p1s_mult_pippenger(
        ret: *mut blst_p1,
        points: *const *const blst_p1_affine,
        npoints: usize,
        scalars: *const *const byte,
        nbits: usize,
        scratch: *mut limb_t,
    );
}
extern "C" {
    pub fn blst_p1s_tile_pippenger(
        ret: *mut blst_p1,
        points: *const *const blst_p1_affine,
        npoints: usize,
        scalars: *const *const byte,
        nbits: usize,
        scratch: *mut limb_t,
        bit0: usize,
        window: usize,
    );
}
extern "C" {
    pub fn blst_p2s_to_affine(
        dst: *mut blst_p2_affine,
        points: *const *const blst_p2,
        npoints: usize,
    );
}
extern "C" {
    pub fn blst_p2s_add(ret: *mut blst_p2, points: *const *const blst_p2_affine, npoints: usize);
}
extern "C" {
    pub fn blst_p2s_mult_wbits_precompute_sizeof(wbits: usize, npoints: usize) -> usize;
}
extern "C" {
    pub fn blst_p2s_mult_wbits_precompute(
        table: *mut blst_p2_affine,
        wbits: usize,
        points: *const *const blst_p2_affine,
        npoints: usize,
    );
}
extern "C" {
    pub fn blst_p2s_mult_wbits_scratch_sizeof(npoints: usize) -> usize;
}
extern "C" {
    pub fn blst_p2s_mult_wbits(
        ret: *mut blst_p2,
        table: *const blst_p2_affine,
        wbits: usize,
        npoints: usize,
        scalars: *const *const byte,
        nbits: usize,
        scratch: *mut limb_t,
    );
}
extern "C" {
    pub fn blst_p2s_mult_pippenger_scratch_sizeof(npoints: usize) -> usize;
}
extern "C" {
    pub fn blst_p2s_mult_pippenger(
        ret: *mut blst_p2,
        points: *const *const blst_p2_affine,
        npoints: usize,
        scalars: *const *const byte,
        nbits: usize,
        scratch: *mut limb_t,
    );
}
extern "C" {
    pub fn blst_p2s_tile_pippenger(
        ret: *mut blst_p2,
        points: *const *const blst_p2_affine,
        npoints: usize,
        scalars: *const *const byte,
        nbits: usize,
        scratch: *mut limb_t,
        bit0: usize,
        window: usize,
    );
}
// TODO: This should be patched if used intensively as specified https://eips.ethereum.org/assets/eip-2537/field_to_curve
extern "C" {
    pub fn blst_map_to_g1(out: *mut blst_p1, u: *const blst_fp, v: *const blst_fp);
}
// TODO: This should be patched if used intensively as specified https://eips.ethereum.org/assets/eip-2537/field_to_curve
extern "C" {
    pub fn blst_map_to_g2(out: *mut blst_p2, u: *const blst_fp2, v: *const blst_fp2);
}
extern "C" {
    pub fn blst_encode_to_g1(
        out: *mut blst_p1,
        msg: *const byte,
        msg_len: usize,
        DST: *const byte,
        DST_len: usize,
        aug: *const byte,
        aug_len: usize,
    );
}
extern "C" {
    pub fn blst_hash_to_g1(
        out: *mut blst_p1,
        msg: *const byte,
        msg_len: usize,
        DST: *const byte,
        DST_len: usize,
        aug: *const byte,
        aug_len: usize,
    );
}
extern "C" {
    pub fn blst_encode_to_g2(
        out: *mut blst_p2,
        msg: *const byte,
        msg_len: usize,
        DST: *const byte,
        DST_len: usize,
        aug: *const byte,
        aug_len: usize,
    );
}
extern "C" {
    pub fn blst_hash_to_g2(
        out: *mut blst_p2,
        msg: *const byte,
        msg_len: usize,
        DST: *const byte,
        DST_len: usize,
        aug: *const byte,
        aug_len: usize,
    );
}
extern "C" {
    pub fn blst_p1_serialize(out: *mut byte, in_: *const blst_p1);
}
extern "C" {
    pub fn blst_p1_compress(out: *mut byte, in_: *const blst_p1);
}
extern "C" {
    pub fn blst_p1_affine_serialize(out: *mut byte, in_: *const blst_p1_affine);
}
extern "C" {
    pub fn blst_p1_affine_compress(out: *mut byte, in_: *const blst_p1_affine);
}
// extern "C" {
//     pub fn blst_p1_uncompress(out: *mut blst_p1_affine, in_: *const byte) -> BLST_ERROR;
// }
pub unsafe extern "C" fn blst_p1_uncompress(out: *mut blst_p1_affine, in_: *const byte) -> BLST_ERROR {
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
extern "C" {
    pub fn blst_p1_deserialize(out: *mut blst_p1_affine, in_: *const byte) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_p2_serialize(out: *mut byte, in_: *const blst_p2);
}
extern "C" {
    pub fn blst_p2_compress(out: *mut byte, in_: *const blst_p2);
}
extern "C" {
    pub fn blst_p2_affine_serialize(out: *mut byte, in_: *const blst_p2_affine);
}
extern "C" {
    pub fn blst_p2_affine_compress(out: *mut byte, in_: *const blst_p2_affine);
}
// extern "C" {
//     pub fn blst_p2_uncompress(out: *mut blst_p2_affine, in_: *const byte) -> BLST_ERROR;
// }
pub unsafe extern "C" fn blst_p2_uncompress(out: *mut blst_p2_affine, in_: *const byte) -> BLST_ERROR {
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
extern "C" {
    pub fn blst_p2_deserialize(out: *mut blst_p2_affine, in_: *const byte) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_keygen(
        out_SK: *mut blst_scalar,
        IKM: *const byte,
        IKM_len: usize,
        info: *const byte,
        info_len: usize,
    );
}
extern "C" {
    pub fn blst_sk_to_pk_in_g1(out_pk: *mut blst_p1, SK: *const blst_scalar);
}
extern "C" {
    pub fn blst_sign_pk_in_g1(out_sig: *mut blst_p2, hash: *const blst_p2, SK: *const blst_scalar);
}
extern "C" {
    pub fn blst_sk_to_pk_in_g2(out_pk: *mut blst_p2, SK: *const blst_scalar);
}
extern "C" {
    pub fn blst_sign_pk_in_g2(out_sig: *mut blst_p1, hash: *const blst_p1, SK: *const blst_scalar);
}
// extern "C" {
//     pub fn blst_miller_loop(
//         ret: *mut blst_fp12,
//         Q: *const blst_p2_affine,
//         P: *const blst_p1_affine,
//     );
// }
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
extern "C" {
    pub fn blst_miller_loop_n(
        ret: *mut blst_fp12,
        Qs: *const *const blst_p2_affine,
        Ps: *const *const blst_p1_affine,
        n: usize,
    );
}
// extern "C" {
//     pub fn blst_final_exp(ret: *mut blst_fp12, f: *const blst_fp12);
// }
pub unsafe extern "C" fn blst_final_exp(ret: *mut blst_fp12, f: *const blst_fp12) {
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

    // Call ZisK implementation
    zisk::final_exp_bls12_381_c(std_input.as_mut_ptr());

    // Convert result back to Montgomery form
    let ret_fp = ret as *mut blst_fp;
    let std_fp = std_input.as_ptr() as *const blst_fp;
    
    for i in 0..12 {
        blst_fp_to(ret_fp.add(i), std_fp.add(i));
    }
}
extern "C" {
    pub fn blst_precompute_lines(Qlines: *mut blst_fp6, Q: *const blst_p2_affine);
}
extern "C" {
    pub fn blst_miller_loop_lines(
        ret: *mut blst_fp12,
        Qlines: *const blst_fp6,
        P: *const blst_p1_affine,
    );
}
extern "C" {
    pub fn blst_fp12_finalverify(gt1: *const blst_fp12, gt2: *const blst_fp12) -> bool;
}
#[repr(C)]
#[repr(align(1))]
#[derive(Debug, Default)]
pub struct blst_pairing {
    pub _bindgen_opaque_blob: [u8; 0usize],
}
#[test]
fn bindgen_test_layout_blst_pairing() {
    assert_eq!(
        ::core::mem::size_of::<blst_pairing>(),
        0usize,
        concat!("Size of: ", stringify!(blst_pairing))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_pairing>(),
        1usize,
        concat!("Alignment of ", stringify!(blst_pairing))
    );
}
extern "C" {
    pub fn blst_pairing_sizeof() -> usize;
}
extern "C" {
    pub fn blst_pairing_init(
        new_ctx: *mut blst_pairing,
        hash_or_encode: bool,
        DST: *const byte,
        DST_len: usize,
    );
}
extern "C" {
    pub fn blst_pairing_get_dst(ctx: *const blst_pairing) -> *const byte;
}
extern "C" {
    pub fn blst_pairing_commit(ctx: *mut blst_pairing);
}
extern "C" {
    pub fn blst_pairing_aggregate_pk_in_g2(
        ctx: *mut blst_pairing,
        PK: *const blst_p2_affine,
        signature: *const blst_p1_affine,
        msg: *const byte,
        msg_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_pairing_chk_n_aggr_pk_in_g2(
        ctx: *mut blst_pairing,
        PK: *const blst_p2_affine,
        pk_grpchk: bool,
        signature: *const blst_p1_affine,
        sig_grpchk: bool,
        msg: *const byte,
        msg_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_pairing_mul_n_aggregate_pk_in_g2(
        ctx: *mut blst_pairing,
        PK: *const blst_p2_affine,
        sig: *const blst_p1_affine,
        scalar: *const byte,
        nbits: usize,
        msg: *const byte,
        msg_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_pairing_chk_n_mul_n_aggr_pk_in_g2(
        ctx: *mut blst_pairing,
        PK: *const blst_p2_affine,
        pk_grpchk: bool,
        sig: *const blst_p1_affine,
        sig_grpchk: bool,
        scalar: *const byte,
        nbits: usize,
        msg: *const byte,
        msg_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_pairing_aggregate_pk_in_g1(
        ctx: *mut blst_pairing,
        PK: *const blst_p1_affine,
        signature: *const blst_p2_affine,
        msg: *const byte,
        msg_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_pairing_chk_n_aggr_pk_in_g1(
        ctx: *mut blst_pairing,
        PK: *const blst_p1_affine,
        pk_grpchk: bool,
        signature: *const blst_p2_affine,
        sig_grpchk: bool,
        msg: *const byte,
        msg_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_pairing_mul_n_aggregate_pk_in_g1(
        ctx: *mut blst_pairing,
        PK: *const blst_p1_affine,
        sig: *const blst_p2_affine,
        scalar: *const byte,
        nbits: usize,
        msg: *const byte,
        msg_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_pairing_chk_n_mul_n_aggr_pk_in_g1(
        ctx: *mut blst_pairing,
        PK: *const blst_p1_affine,
        pk_grpchk: bool,
        sig: *const blst_p2_affine,
        sig_grpchk: bool,
        scalar: *const byte,
        nbits: usize,
        msg: *const byte,
        msg_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_pairing_merge(ctx: *mut blst_pairing, ctx1: *const blst_pairing) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_pairing_finalverify(ctx: *const blst_pairing, gtsig: *const blst_fp12) -> bool;
}
extern "C" {
    pub fn blst_aggregate_in_g1(
        out: *mut blst_p1,
        in_: *const blst_p1,
        zwire: *const byte,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_aggregate_in_g2(
        out: *mut blst_p2,
        in_: *const blst_p2,
        zwire: *const byte,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_aggregated_in_g1(out: *mut blst_fp12, signature: *const blst_p1_affine);
}
extern "C" {
    pub fn blst_aggregated_in_g2(out: *mut blst_fp12, signature: *const blst_p2_affine);
}
extern "C" {
    pub fn blst_core_verify_pk_in_g1(
        pk: *const blst_p1_affine,
        signature: *const blst_p2_affine,
        hash_or_encode: bool,
        msg: *const byte,
        msg_len: usize,
        DST: *const byte,
        DST_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub fn blst_core_verify_pk_in_g2(
        pk: *const blst_p2_affine,
        signature: *const blst_p1_affine,
        hash_or_encode: bool,
        msg: *const byte,
        msg_len: usize,
        DST: *const byte,
        DST_len: usize,
        aug: *const byte,
        aug_len: usize,
    ) -> BLST_ERROR;
}
extern "C" {
    pub static BLS12_381_G1: blst_p1_affine;
}
extern "C" {
    pub static BLS12_381_NEG_G1: blst_p1_affine;
}
extern "C" {
    pub static BLS12_381_G2: blst_p2_affine;
}
extern "C" {
    pub static BLS12_381_NEG_G2: blst_p2_affine;
}
extern "C" {
    pub fn blst_fr_ct_bfly(x0: *mut blst_fr, x1: *mut blst_fr, twiddle: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_gs_bfly(x0: *mut blst_fr, x1: *mut blst_fr, twiddle: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_to(ret: *mut blst_fr, a: *const blst_fr);
}
extern "C" {
    pub fn blst_fr_from(ret: *mut blst_fr, a: *const blst_fr);
}
extern "C" {
    pub fn blst_fp_to(ret: *mut blst_fp, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_from(ret: *mut blst_fp, a: *const blst_fp);
}
extern "C" {
    pub fn blst_fp_is_square(a: *const blst_fp) -> bool;
}
extern "C" {
    pub fn blst_fp2_is_square(a: *const blst_fp2) -> bool;
}
extern "C" {
    pub fn blst_p1_from_jacobian(out: *mut blst_p1, in_: *const blst_p1);
}
extern "C" {
    pub fn blst_p2_from_jacobian(out: *mut blst_p2, in_: *const blst_p2);
}
extern "C" {
    pub fn blst_sk_to_pk2_in_g1(
        out: *mut byte,
        out_pk: *mut blst_p1_affine,
        SK: *const blst_scalar,
    );
}
extern "C" {
    pub fn blst_sign_pk2_in_g1(
        out: *mut byte,
        out_sig: *mut blst_p2_affine,
        hash: *const blst_p2,
        SK: *const blst_scalar,
    );
}
extern "C" {
    pub fn blst_sk_to_pk2_in_g2(
        out: *mut byte,
        out_pk: *mut blst_p2_affine,
        SK: *const blst_scalar,
    );
}
extern "C" {
    pub fn blst_sign_pk2_in_g2(
        out: *mut byte,
        out_sig: *mut blst_p1_affine,
        hash: *const blst_p1,
        SK: *const blst_scalar,
    );
}
#[repr(C)]
#[repr(align(1))]
#[derive(Debug, Default)]
pub struct blst_uniq {
    pub _bindgen_opaque_blob: [u8; 0usize],
}
#[test]
fn bindgen_test_layout_blst_uniq() {
    assert_eq!(
        ::core::mem::size_of::<blst_uniq>(),
        0usize,
        concat!("Size of: ", stringify!(blst_uniq))
    );
    assert_eq!(
        ::core::mem::align_of::<blst_uniq>(),
        1usize,
        concat!("Alignment of ", stringify!(blst_uniq))
    );
}
extern "C" {
    pub fn blst_uniq_sizeof(n_nodes: usize) -> usize;
}
extern "C" {
    pub fn blst_uniq_init(tree: *mut blst_uniq);
}
extern "C" {
    pub fn blst_uniq_test(tree: *mut blst_uniq, msg: *const byte, len: usize) -> bool;
}
extern "C" {
    pub fn blst_expand_message_xmd(
        out: *mut byte,
        out_len: usize,
        msg: *const byte,
        msg_len: usize,
        DST: *const byte,
        DST_len: usize,
    );
}
extern "C" {
    pub fn blst_p1_unchecked_mult(
        out: *mut blst_p1,
        p: *const blst_p1,
        scalar: *const byte,
        nbits: usize,
    );
}
extern "C" {
    pub fn blst_p2_unchecked_mult(
        out: *mut blst_p2,
        p: *const blst_p2,
        scalar: *const byte,
        nbits: usize,
    );
}
extern "C" {
    pub fn blst_pairing_raw_aggregate(
        ctx: *mut blst_pairing,
        q: *const blst_p2_affine,
        p: *const blst_p1_affine,
    );
}
extern "C" {
    pub fn blst_pairing_as_fp12(ctx: *mut blst_pairing) -> *mut blst_fp12;
}
extern "C" {
    pub fn blst_bendian_from_fp12(out: *mut byte, a: *const blst_fp12);
}
extern "C" {
    pub fn blst_keygen_v3(
        out_SK: *mut blst_scalar,
        IKM: *const byte,
        IKM_len: usize,
        info: *const byte,
        info_len: usize,
    );
}
extern "C" {
    pub fn blst_keygen_v4_5(
        out_SK: *mut blst_scalar,
        IKM: *const byte,
        IKM_len: usize,
        salt: *const byte,
        salt_len: usize,
        info: *const byte,
        info_len: usize,
    );
}
extern "C" {
    pub fn blst_keygen_v5(
        out_SK: *mut blst_scalar,
        IKM: *const byte,
        IKM_len: usize,
        salt: *const byte,
        salt_len: usize,
        info: *const byte,
        info_len: usize,
    );
}
extern "C" {
    pub fn blst_derive_master_eip2333(out_SK: *mut blst_scalar, IKM: *const byte, IKM_len: usize);
}
extern "C" {
    pub fn blst_derive_child_eip2333(
        out_SK: *mut blst_scalar,
        SK: *const blst_scalar,
        child_index: u32,
    );
}
extern "C" {
    pub fn blst_scalar_from_hexascii(out: *mut blst_scalar, hex: *const byte);
}
extern "C" {
    pub fn blst_fr_from_hexascii(ret: *mut blst_fr, hex: *const byte);
}
extern "C" {
    pub fn blst_fp_from_hexascii(ret: *mut blst_fp, hex: *const byte);
}
extern "C" {
    pub fn blst_p1_sizeof() -> usize;
}
extern "C" {
    pub fn blst_p1_affine_sizeof() -> usize;
}
extern "C" {
    pub fn blst_p2_sizeof() -> usize;
}
extern "C" {
    pub fn blst_p2_affine_sizeof() -> usize;
}
extern "C" {
    pub fn blst_fp12_sizeof() -> usize;
}
extern "C" {
    pub fn blst_sha256(out: *mut byte, msg: *const byte, msg_len: usize);
}
#[test]
fn bindgen_test_normal_types() {
    // from "Rust for Rustaceans" by Jon Gjengset
    fn is_normal<T: Sized + Send + Sync + Unpin>() {}
    is_normal::<BLST_ERROR>();
    is_normal::<blst_scalar>();
    is_normal::<blst_fr>();
    is_normal::<blst_fp>();
    is_normal::<blst_fp2>();
    is_normal::<blst_fp6>();
    is_normal::<blst_fp12>();
    is_normal::<blst_p1>();
    is_normal::<blst_p1_affine>();
    is_normal::<blst_p2>();
    is_normal::<blst_p2_affine>();
    is_normal::<blst_pairing>();
    is_normal::<blst_uniq>();
}
