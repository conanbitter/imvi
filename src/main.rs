use phf::phf_set;
use sdl3::event::Event;
use sdl3::image::{LoadSurface, LoadTexture};
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{Canvas, Texture, TextureCreator};
use sdl3::surface::Surface;
use sdl3::video::{Window, WindowContext};
use sdl3::{Sdl, VideoSubsystem};
use std::fs;
use std::path::{Path, PathBuf};
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

struct FileEntry<'a> {
    pub filename: PathBuf,
    pub name: String,
    pub thumbnail_name: PathBuf,
    pub thumbnail: Option<Texture<'a>>,
}

struct App<'a> {
    sdl_context: Sdl,
    sdl_video: VideoSubsystem,
    window: Window,
    window_canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    current_texture: Option<Texture<'a>>,
    current_index: usize,
    files: Vec<FileEntry<'a>>,
    root: PathBuf,
}

impl<'a> App<'a> {
    fn init(path: &'static str) -> anyhow::Result<App<'a>> {
        let sdl_context = sdl3::init()?;
        let sdl_video = sdl_context.video()?;

        let window = sdl_video.window("imvi", 800, 600).position_centered().build()?;
        let mut window_canvas = window.clone().into_canvas();
        window_canvas.set_draw_color(Color::RGB(23, 36, 42));

        let root = PathBuf::from(path);
        let mut files: Vec<FileEntry> = vec![];

        for entry in fs::read_dir(&root)? {
            let path = entry?.path();
            if path.is_file()
                && let Some(ext) = path.extension()
                && let Some(ext) = ext.to_str()
                && EXTENTIONS.contains(ext)
            {
                let thumbnail_name = root.join("_preview").join(path.file_name().unwrap());
                files.push(FileEntry {
                    filename: path.clone(),
                    //thumbnail: Some(result.texture_creator.load_texture(&thumbnail_name).unwrap()),
                    thumbnail_name,
                    name: path.file_name().unwrap().to_str().unwrap().into(),
                    thumbnail: None,
                });
            }
        }

        Ok(App {
            sdl_context,
            sdl_video,
            window,
            texture_creator: window_canvas.texture_creator(),
            window_canvas,

            current_texture: None,
            current_index: 0,
            files,
            root: root.clone(),
        })
    }

    fn load_thumbnails(&'a mut self) {
        for file in &mut self.files {
            file.thumbnail = Some(self.texture_creator.load_texture(&file.thumbnail_name).unwrap());
        }
    }

    fn run(&mut self) -> anyhow::Result<()> {
        let mut event_pump = self.sdl_context.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::MouseWheel { y, .. } => {
                        if y < 0.0 {
                            self.next_index();
                        }
                        if y > 0.0 {
                            self.prev_index();
                        }
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => {
                        self.next_index();
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => {
                        self.prev_index();
                    }
                    _ => {}
                }
            }

            self.window_canvas.clear();
            if let Some(ref texture) = self.current_texture {
                self.window_canvas.copy(texture, None, None).unwrap();
            } else if let Some(ref texture) = self.files[self.current_index].thumbnail {
                self.window_canvas.copy(texture, None, None).unwrap();
            }
            self.window_canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    fn change_image(&mut self) -> anyhow::Result<()> {
        self.current_texture = None;
        self.window.set_title(
            format!(
                "[{}/{}] {} - imvi",
                self.current_index,
                self.files.len(),
                self.files[self.current_index].name
            )
            .as_str(),
        )?;
        //self.current_texture = self.files[self.current_index].thumbnail.as_ref();
        Ok(())
    }

    fn next_index(&mut self) -> anyhow::Result<()> {
        if self.current_index < self.files.len() - 1 {
            self.current_index += 1;
            self.change_image()?;
        }
        Ok(())
    }

    fn prev_index(&mut self) -> anyhow::Result<()> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.change_image()?;
        }
        Ok(())
    }

    //let image_surface = next_image(&files[current_image].filename);
}

fn main() {
    let mut app = App::init("./test_data").unwrap();
    app.load_thumbnails();
    app.run().unwrap();
}
