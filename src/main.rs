extern crate num_complex;
use num_complex::Complex64 as Complex;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

fn main() {
    let width = 640;
    let height = 480;
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let window = video_ctx
        .window("Mandelbrot set", width, height)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = ctx.event_pump().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    println!("{:?}", gradient(WHITE, BLACK, 10));

    let _ = (5..1);

    'event_loop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'event_loop;
                }
                _ => {}
            }
        }
    }
}

fn gradient(begin: Color, end: Color, steps: u32) -> Vec<Color> {
    let mut colors = vec![begin];
    let begin = (begin.r as f32, begin.g as f32, begin.b as f32);
    let steps_f = steps as f32;
    let step_by = (
        (end.r as f32 - begin.0) / steps_f,
        (end.g as f32 - begin.1) / steps_f,
        (end.b as f32 - begin.2) / steps_f,
    );
    for i in 1..steps - 1 {
        let i = i as f32;
        colors.push(Color {
            r: (begin.0 + i * step_by.0) as u8,
            g: (begin.1 + i * step_by.1) as u8,
            b: (begin.1 + i * step_by.1) as u8,
            a: 255,
        });
    }
    colors.push(end);
    colors
}

fn count(c: Complex, max: u32) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..max {
        z = z * z + c;
        if z.norm() > 2.0 {
            return i;
        }
    }
    max
}
