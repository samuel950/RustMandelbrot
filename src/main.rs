use sdl2::{pixels::Color, rect::Point, render::Canvas};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsytem = sdl_context.video().unwrap();
    let window = video_subsytem
        .window("Mandelbrot", 1200, 1200)
        .build()
        .unwrap();
    let mut canvas: Canvas<sdl2::video::Window> =
        window.into_canvas().present_vsync().build().unwrap();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    draw(1200.0, 1200.0, &mut canvas);
    canvas.present();
    loop {}
}

fn draw(height: f64, width: f64, mut canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let maxiter: i32 = 100;
    let minreal = -2.0;
    let maxreal = 1.0;
    let miniq = -1.2;
    let maxiq = miniq + (maxreal - minreal) * height / width;
    let real_velocity = (maxreal - minreal) / (width - 1.0);
    let iq_velocity = (maxiq - miniq) / (height - 1.0);
    for y in 0..height as i32 {
        let ciq = maxiq - y as f64 * iq_velocity;
        for x in 0..width as i32 {
            let creal = minreal + (x as f64 * real_velocity);
            let mut zreal = creal;
            let mut ziq = ciq;
            let mut inside = true;
            for n in 0..maxiter {
                let zreal2 = zreal * zreal;
                let ziq2 = ziq * ziq;
                if zreal2 + ziq2 > 4.0 {
                    inside = false;
                    put_pixel(x, y, n, false, &mut canvas);
                    break;
                }
                ziq = (2.0 * zreal * ziq) + ciq;
                zreal = zreal2 - ziq2 + creal;
            } //end for n
            if inside {
                put_pixel(x, y, 0, true, &mut canvas);
            }
        } //end for x
    } //end for y
}
fn put_pixel(
    x: i32,
    y: i32,
    n: i32,
    inside: bool,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
    let rlow = 0.0;
    let glow = 0.0;
    let blow = 0.0;
    let rmed = 1.0;
    let gmed = 0.0;
    let bmed = 0.0;
    let rhigh = 1.0;
    let ghigh = 200.0 / 255.0;
    let bhigh = 0.0;

    let max = 100.0;
    let p: Point = sdl2::rect::Point::new(x, y);
    if inside {
        canvas.set_draw_color(Color::BLACK);
        canvas.draw_point(p).unwrap();
    } else {
        let mut prop: f64 = 0.0;
        let t = max / 2.0;
        if n < (max / 2.0) as i32 {
            prop = n as f64 / (t - 1.0);
            let rval = rlow + prop * (rmed - rlow);
            let gval = glow + prop * (gmed - glow);
            let bval = blow + prop * (bmed - blow);
            let r: u8 = (rval * 255.0) as u8;
            let g: u8 = (gval * 255.0) as u8;
            let b: u8 = (bval * 255.0) as u8;
            canvas.set_draw_color(Color::RGB(r, g, b));
            canvas.draw_point(p).unwrap();
        } else {
            prop = (n as f64 - t) / (max - 1.0 - t);
            let rval = rmed + prop * (rhigh - rmed);
            let gval = gmed + prop * (ghigh - gmed);
            let bval = bmed + prop * (bhigh - bmed);
            let r: u8 = (rval * 255.0) as u8;
            let g: u8 = (gval * 255.0) as u8;
            let b: u8 = (bval * 255.0) as u8;
            canvas.set_draw_color(Color::RGB(r, g, b));
            canvas.draw_point(p).unwrap();
        }
    }
}
