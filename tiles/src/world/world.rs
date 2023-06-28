use sdl2::{
    pixels::Color,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
};

use super::{model::map_model::MapModel, view::map_view::MapView};

pub struct World<'a> {
    current_map: MapModel,
    current_map_view: MapView<'a>,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        Self {
            current_map: MapModel::test_map(),
            current_map_view: MapView::new(),
        }
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        self.current_map_view
            .draw(canvas, &self.current_map, texture_creator);
    }
}
