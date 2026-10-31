use raylib::ffi::GetRandomValue;

pub fn get_random_value() -> i32 {
    unsafe { 2 * GetRandomValue(0, 1) - 1 }
}
