//! Boundary-Tests für apw-kernel.

#[test]
fn smoke() {
    assert_eq!(apw_kernel::name(), "apw-kernel");
}

