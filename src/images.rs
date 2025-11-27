use sdl3::{
    render::{Texture, TextureCreator},
    surface::Surface,
    video::WindowContext,
};

pub struct Image {
    pub image: Option<Texture>,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
}

impl Image {
    pub fn from_surface(surface: Surface, creator: &TextureCreator<WindowContext>) -> anyhow::Result<Image> {
        let width = surface.width();
        let height = surface.height();
        Ok(Image {
            image: Some(creator.create_texture_from_surface(surface)?),
            width,
            height,
            aspect_ratio: width as f64 / height as f64,
        })
    }

    pub fn clear(&mut self) {
        if let Some(texture) = self.image.take() {
            unsafe {
                texture.destroy();
            }
        }
    }

    pub fn load(&mut self, surface: Surface, creator: &TextureCreator<WindowContext>) -> anyhow::Result<()> {
        self.clear();
        self.width = surface.width();
        self.height = surface.height();
        self.aspect_ratio = self.width as f64 / self.height as f64;
        self.image = Some(creator.create_texture_from_surface(surface)?);
        Ok(())
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            image: None,
            width: 0,
            height: 0,
            aspect_ratio: 1.0,
        }
    }
}
