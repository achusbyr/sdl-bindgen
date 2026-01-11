extern crate sdl_sys_bindgen;

fn main() {
    if unsafe { sdl_sys_bindgen::SDL_Init(sdl_sys_bindgen::SDL_INIT_VIDEO) } {
        println!("Hello World");
        unsafe { sdl_sys_bindgen::SDL_Quit() };
    } else {
        println!("Error: {:#?}", unsafe {
            std::ffi::CStr::from_ptr(sdl_sys_bindgen::SDL_GetError())
        });
    }
}
