use phf::phf_set;
use sdl3::event::Event;
use sdl3::image::LoadSurface;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::Texture;
use sdl3::surface::Surface;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

static EXTENTIONS: phf::Set<&'static str> = phf_set!(
    "cur",
    "ico",
    "bmp",
    "pnm",
    "xpm",
    "xcf",
    "pcx",
    "gif",
    "jpg" | "jpeg",
    "tif" | "tiff",
    "png",
    "tga",
    "lbm",
    "xv",
    "webp",
);

fn next_image(file: &PathBuf) -> Surface<'_> {
    Surface::from_file(file).unwrap()
}

fn main() {
    let mut files: Vec<PathBuf> = vec![];

    let path = "./test_data";
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file()
            && let Some(ext) = path.extension()
            && let Some(ext) = ext.to_str()
            && EXTENTIONS.contains(ext)
        {
            files.push(path.clone());
            //println!("{}", path.display());
        }
    }

    let mut current_image: usize = 0;

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("imvi", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let image_surface = Surface::from_file(&files[0]).unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = Texture::from_surface(&image_surface, &texture_creator).unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseWheel { y, .. } => {
                    if y < 0.0 && current_image < files.len() - 1 {
                        current_image += 1;
                    }
                    if y > 0.0 && current_image > 0 {
                        current_image -= 1;
                    }
                    let image_surface = next_image(&files[current_image]);
                    texture = Texture::from_surface(&image_surface, &texture_creator).unwrap();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if current_image < files.len() - 1 {
                        current_image += 1;
                        let image_surface = next_image(&files[current_image]);
                        texture = Texture::from_surface(&image_surface, &texture_creator).unwrap();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if current_image > 0 {
                        current_image -= 1;
                        let image_surface = next_image(&files[current_image]);
                        texture = Texture::from_surface(&image_surface, &texture_creator).unwrap();
                    }
                }
                _ => {}
            }
        }
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
