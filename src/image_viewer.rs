use crate::{
    files::ImageContainer,
    window::{Rect, Window},
};

pub struct ImageViewer {
    view_rect: Rect,
}

impl ImageViewer {
    pub fn new() -> ImageViewer {
        return ImageViewer {
            view_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
        };
    }

    fn update_view(&mut self, window: &Window, container: &ImageContainer) {
        let image_ar = container.get_image().aspect_ratio;
        if image_ar < window.aspect_ratio {
            self.view_rect.w = window.height as f32 * image_ar;
            self.view_rect.h = window.height as f32;
            self.view_rect.x = (window.width as f32 - self.view_rect.w) / 2.0;
            self.view_rect.y = 0.0;
        } else {
            self.view_rect.w = window.width as f32;
            self.view_rect.h = window.width as f32 / image_ar;
            self.view_rect.x = 0.0;
            self.view_rect.y = (window.height as f32 - self.view_rect.h) / 2.0;
        }
    }
}
