
use crate::{blst_fp12, blst_p1, blst_p1_affine, blst_p2, blst_p2_affine, byte, zisk};

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p2_add_or_double_affine(out: *mut blst_p2, a: *const blst_p2, b: *const blst_p2_affine) {
    zisk::blst_p2_add_or_double_affine(out, a, b);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_fp12_mul(ret: *mut blst_fp12, a: *const blst_fp12, b: *const blst_fp12) {
    zisk::blst_fp12_mul(ret, a, b);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p1_add_or_double_affine(out: *mut blst_p1, a: *const blst_p1, b: *const blst_p1_affine) {
    zisk::blst_p1_add_or_double_affine(out, a, b);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p1_uncompress(out: *mut blst_p1_affine, in_: *const byte) {
    let _ = zisk::blst_p1_uncompress(out, in_);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p1_affine_on_curve(p: *const blst_p1_affine) {
    let _ = zisk::blst_p1_affine_on_curve(p);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p1_affine_in_g1(p: *const blst_p1_affine) {
    let _ = zisk::blst_p1_affine_in_g1(p);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p1_mult(out: *mut blst_p1, p: *const blst_p1, scalar: *const byte, nbits: usize) {
    zisk::blst_p1_mult(out, p, scalar, nbits);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p2_uncompress(out: *mut blst_p2_affine, in_: *const byte) {
    let _ = zisk::blst_p2_uncompress(out, in_);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p2_affine_on_curve(p: *const blst_p2_affine) {
    let _ = zisk::blst_p2_affine_on_curve(p);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p2_affine_in_g2(p: *const blst_p2_affine) {
    let _ = zisk::blst_p2_affine_in_g2(p);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_p2_mult(out: *mut blst_p2, p: *const blst_p2, scalar: *const byte, nbits: usize) {
    zisk::blst_p2_mult(out, p, scalar, nbits);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_miller_loop(ret: *mut blst_fp12, Q: *const blst_p2_affine, P: *const blst_p1_affine) {
    zisk::blst_miller_loop(ret, Q, P);
}

#[no_mangle]
pub unsafe extern "C" fn hint_blst_final_exp(ret: *mut blst_fp12, f: *const blst_fp12) {
    zisk::blst_final_exp(ret, f);
}