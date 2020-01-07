
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[cfg(test)]
mod tests {

    #[test]
    fn api_use(){
        use super::*;
        unsafe {
            let result=glfwInit();
            assert!(result != 0);
        }
    }
}