pub fn test_function_pointer() {
    let pointer = foo as *const ();
    let function = unsafe { std::mem::transmute::<_, fn(i32) -> i32>(pointer) };
    assert_eq!(function(5), 10);
}

pub fn test_lifetime() {
    let mut r;
    {
        let s = String::from("abc");
        r = R { data: &s };
        r = unsafe { extend_lifetime(r) };
    }
    println!("{}", r.data);
}

pub fn test_str_to_u8() {
    let s = "abcd";
    let v = unsafe { std::mem::transmute::<&str, &[u8]>(s) };
    println!("{:?}", v);
}

fn foo(mut i: i32) -> i32 {
    i *= 2;
    i
}

struct R<'a> {
    data: &'a String,
}

unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    std::mem::transmute::<R<'b>, R<'static>>(r)
}
