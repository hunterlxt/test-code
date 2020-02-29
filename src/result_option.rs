pub fn test_chain_result() {
    let mut b1 = Box::new(10 as i32);
    *b1 += 10;
    let mut b2 = b1.clone();
    *b2 += 100;
    let ptr: *mut std::ffi::c_void = unsafe { std::mem::transmute(b1) };
    let content: &i32 = unsafe { std::mem::transmute(ptr) };
    assert_eq!(*content, 10);
}

#[derive(Debug)]
struct Data {
    data: i32,
}
