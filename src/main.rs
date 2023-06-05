// does not work

use std::thread;
use std::time::Duration;

const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const BACKGROUND_ASCII_CODE: u8 = b' ';

fn calculate_x(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.sin() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos()
        + j * a.cos() * c.sin() + k * a.sin() * c.sin() + i * b.cos() * c.cos()
}

fn calculate_y(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.cos() * c.cos() + k * a.sin() * c.cos()
        - j * a.sin() * b.sin() * c.sin() + k * a.cos() * b.sin() * c.sin()
        - i * b.cos() * c.sin()
}

fn calculate_z(i: f32, j: f32, k: f32, a: f32, b: f32, _c: f32) -> f32 {
    k * a.cos() * b.cos() - j * a.sin() * b.cos() + i * b.sin()
}

fn calculate_for_surface(
    cube_x: f32,
    cube_y: f32,
    cube_z: f32,
    ch: u8,
    buffer: &mut [u8],
    z_buffer: &mut [f32],
    width: usize,
    height: usize,
    distance_from_cam: i32,
    horizontal_offset: f32,
    k1: f32,
    a: f32,
    b: f32,
    c: f32,
) {

    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b, c) + distance_from_cam as f32;

    let ooz = 1.0 / z;

    let xp = (width as f32 / 2.0 + horizontal_offset + k1 * ooz * x * 2.0) as usize;
    let yp = (height as f32 / 2.0 + k1 * ooz * y) as usize;

    let idx = xp + yp * width;
    if idx < width * height {
        if ooz > z_buffer[idx] {
            z_buffer[idx] = ooz;
            buffer[idx] = ch;
        }
    }
}

fn main() {
    let mut buffer = [BACKGROUND_ASCII_CODE; WIDTH * HEIGHT];
    let mut z_buffer = [0.0; WIDTH * HEIGHT];
    let distance_from_cam = 100;
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;

    loop {
        buffer.fill(BACKGROUND_ASCII_CODE);
        z_buffer.fill(0.0);

        let cube_width = 20;
        let horizontal_offset = -2 * cube_width;

        for cube_x in -cube_width..cube_width {
            for cube_y in -cube_width..cube_width {
                calculate_for_surface(
                    cube_x as f32, cube_y as f32, -cube_width as f32,
                    b'@', &mut buffer, &mut z_buffer, WIDTH, HEIGHT,
                    distance_from_cam, horizontal_offset as f32, 40.0,
                    a, b, c,
                );
                calculate_for_surface(
                    cube_width as f32, cube_y as f32, cube_x as f32,
                    b'$', &mut buffer, &mut z_buffer, WIDTH, HEIGHT,
                    distance_from_cam, horizontal_offset as f32, 40.0,
                    a, b, c,
                );
                calculate_for_surface(
                    -cube_width as f32, cube_y as f32, -cube_x as f32,
                    b'~', &mut buffer, &mut z_buffer, WIDTH, HEIGHT,
                    distance_from_cam, horizontal_offset as f32, 40.0,
                    a, b, c,
                );
                calculate_for_surface(
                    -cube_x as f32, cube_y as f32, cube_width as f32,
                    b'#', &mut buffer, &mut z_buffer, WIDTH, HEIGHT,
                    distance_from_cam, horizontal_offset as f32, 40.0,
                    a, b, c,
                );
                calculate_for_surface(
                    cube_x as f32, -cube_width as f32, -cube_y as f32,
                    b';', &mut buffer, &mut z_buffer, WIDTH, HEIGHT,
                    distance_from_cam, horizontal_offset as f32, 40.0,
                    a, b, c,
                );
                calculate_for_surface(
                    cube_x as f32, cube_width as f32, cube_y as f32,
                    b'+', &mut buffer, &mut z_buffer, WIDTH, HEIGHT,
                    distance_from_cam, horizontal_offset as f32, 40.0,
                    a, b, c,
                );
            }
        }

        print!("\x1b[2J\x1b[H");
        for (i, &ch) in buffer.iter().enumerate() {
            print!("{}", ch as char);
            if (i + 1) % WIDTH == 0 {
                println!();
            }
        }

        a += 0.05;
        b += 0.05;
        c += 0.01;

        thread::sleep(Duration::from_millis(16));
    }
}
