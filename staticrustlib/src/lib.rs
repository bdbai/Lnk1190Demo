#![feature(linkage)]

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn my_func() { }
