use std::{fs, path::PathBuf};

use phf::phf_set;
use sdl3::{
    image::LoadSurface,
    render::{Texture, TextureCreator},
    surface::Surface,
    video::WindowContext,
};

use crate::images::Image;
use crate::window::Rect;
use crate::window::Window;

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

pub struct FileEntry {
    pub filename: PathBuf,
    pub name: String,
    thumbnail_file: PathBuf,
    pub thumbnail: Image,
    pub tile_rect: Rect,
}

pub struct ImageContainer {
    pub root: PathBuf,
    pub files: Vec<FileEntry>,
    pub index: usize,
    image: Image,
}

impl ImageContainer {
    pub fn load(root: &PathBuf) -> anyhow::Result<ImageContainer> {
        let mut files: Vec<FileEntry> = vec![];
        for entry in fs::read_dir(root)? {
            let path = entry?.path();
            if path.is_file()
                && let Some(ext) = path.extension()
                && let Some(ext) = ext.to_str()
                && EXTENTIONS.contains(ext)
            {
                let thumbnail_file = root.join("_preview").join(path.file_name().unwrap());
                files.push(FileEntry {
                    filename: path.clone(),
                    name: path.file_name().unwrap().to_str().unwrap().into(),
                    thumbnail_file,
                    thumbnail: Image::default(),
                    tile_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
                });
            }
        }
        Ok(ImageContainer {
            root: root.clone(),
            files,
            index: 0,
            image: Image::default(),
        })
    }

    pub fn load_thumbnails(&mut self, window: &Window) -> anyhow::Result<()> {
        for file in &mut self.files {
            let image_surface = Surface::from_file(&file.thumbnail_file)?;
            file.thumbnail.load(image_surface, &window.texture_creator)?;
        }
        Ok(())
    }

    pub fn get_texture(&self) -> Option<&Texture> {
        if let Some(ref texture) = self.image.image {
            Some(texture)
        } else if let Some(ref texture) = self.files[self.index].thumbnail.image {
            Some(texture)
        } else {
            None
        }
    }

    pub fn get_image(&self) -> &Image {
        if self.image.image.is_some() {
            &self.image
        } else {
            &self.files[self.index].thumbnail
        }
    }

    pub fn next(&mut self) -> bool {
        if self.index < self.files.len() - 1 {
            self.index += 1;
            true
        } else {
            false
        }
    }

    pub fn prev(&mut self) -> bool {
        if self.index > 0 {
            self.index -= 1;
            true
        } else {
            false
        }
    }

    pub fn update_image(&mut self, index: usize, surface: Surface, window: &Window) -> anyhow::Result<()> {
        if index != self.index {
            return Ok(());
        }
        self.image.load(surface, &window.texture_creator)
    }

    pub fn update_thumbnail(
        &mut self,
        index: usize,
        surface: Surface,
        creator: &TextureCreator<WindowContext>,
    ) -> anyhow::Result<()> {
        self.files[index].thumbnail.load(surface, creator)
    }

    pub fn change_image(&mut self, window: &Window) -> anyhow::Result<()> {
        /*self.window.set_title(
            format!(
                "[{}/{}] {} - imvi",
                self.current_index,
                self.files.len(),
                self.files[self.current_index].name
            )
            .as_str(),
        )?;*/
        let image_surface = Surface::from_file(&self.files[self.index].filename)?;
        self.image.load(image_surface, &window.texture_creator)?;
        Ok(())
    }
}
