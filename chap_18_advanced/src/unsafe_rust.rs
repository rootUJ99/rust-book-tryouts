pub unsafe fn mighty_fn() -> i32{
    let num = 7;
    let r1 = &num as *const i32;
    *r1
}
