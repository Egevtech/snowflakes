use glfw::{Action, Context, Key};
use std::os::raw::c_void;

// Тут должно было быть кое-что, но его не будет
// А переделывать структуру мне лень
struct Vec2<T>{
    x: T,
    y: T
}

type Size = Vec2<u32>;

const WINDOW_SIZE: Size = Vec2{
    x: 600,
    y: 600,
};

const WINDOW_TITLE: &str = "Snowflakes";

fn main() {
    use glfw::fail_on_errors;

    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));

    let (mut window, events) = glfw
        .create_window(WINDOW_SIZE.x, WINDOW_SIZE.y, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|f| window.get_proc_address(f).unwrap() as *const c_void );

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 0.5);
    }

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{event:?}");

            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => {}
            }
        }
    }
}
