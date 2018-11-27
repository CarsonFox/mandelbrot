extern crate sdl2;

pub mod complex;
use complex::*;
pub mod position;
use position::*;

use std::{thread, time};
use std::sync::mpsc;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::keyboard::Keycode;

const THREADS: usize = 4;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const MAX_ITER: u32 = 1000;

/*
const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};*/

const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

fn main() {
    //setup window
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let window = video_ctx
        .window("Mandelbrot set", WIDTH as u32, HEIGHT as u32)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut events = ctx.event_pump().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let mut pos = Position::new(3.0, -0.5, 2.0, 0.0);

    draw_screen(&mut canvas, pos);

    'event_loop: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'event_loop;
                }
                Event::MouseButtonDown {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    x,
                    y,
                } => {
                    pos.set_center(x, y);
                    draw_screen(&mut canvas, pos);
                }
                Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => {
                    if keycode.is_some() {
                        match keycode.unwrap() {
                            Keycode::Minus => {
                                pos.zoom_out();
                                draw_screen(&mut canvas, pos);
                            }
                            Keycode::Equals => {
                                pos.zoom_in();
                                draw_screen(&mut canvas, pos);
                            }
                            Keycode::Escape => {
                                break 'event_loop;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn draw_screen(canvas: &mut WindowCanvas, pos: Position) {
    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let mut points = Vec::with_capacity(THREADS);
    for _ in 0..THREADS {
        points.push(Vec::new());
    }

    let mut i: usize = 0;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            points[i].push((x, y));
            i = (i + 1) % THREADS;
        }
    }

    let (tx, rx) = mpsc::channel();

    for vec in points {
        let tx = tx.clone();
        thread::spawn(move || {
            for (x, y) in vec {
                tx.send(calc_color(x, y, pos)).unwrap();
            }
        });
    }

    //If I don't drop tx here, it never goes out of scope
    //and rx blocks forever
    drop(tx);

    let sixteen_millis = time::Duration::from_millis(16);

    for (point, color) in rx {
        canvas.set_draw_color(color);
        canvas.draw_point(point).unwrap();
    }

    canvas.present();
}

fn calc_color(x: i32, y: i32, pos: Position) -> (Point, Color) {
    let point = Point::new(x, y);
    let c = pos.into_complex(x, y);

    let mut z = c;
    let mut iterations = 0;

    while !z.escapes() && iterations < MAX_ITER {
        z.iterate(&c);
        iterations += 1;
    }

    let color = match iterations {
        MAX_ITER => BLACK,
        0 => BLACK,
        _ => {
            let ratio = iterations as f64 * MAX_ITER as f64;
            let red: u8 = (255.0 * ratio) as u8;
            Color::RGB(red, 0, 0)
        }
    };

    (point, color)
}
