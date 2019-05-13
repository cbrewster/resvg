use crate::prelude::*;

use piet_common::{BitmapTarget, Device};
use piet::ImageFormat;

/// Piet backend
#[derive(Clone, Copy)]
pub struct Backend;

pub struct Image {
    pixels: Vec<u8>,
    width: u32,
    height: u32
}

impl Render for Backend {
    fn render_to_image(&self, tree: &usvg::Tree, opt: &Options) -> Option<Box<OutputImage>> {
        let img = render_to_image(tree, opt)?;
        Some(Box::new(img))
    }

    fn render_node_to_image(&self, node: &usvg::Node, opt: &Options) -> Option<Box<OutputImage>> {
        None
    }

    fn calc_node_bbox(&self, node: &usvg::Node, opt: &Options) -> Option<Rect> {
        None
    }
}

impl OutputImage for Image {
    fn save(&self, path: &::std::path::Path) -> bool {
        image::save_buffer(
            path,
            &self.pixels,
            self.width as u32,
            self.height as u32,
            image::ColorType::RGBA(8),
        ).is_ok()
    }
}

/// Renders SVG to image
pub fn render_to_image(
    tree: &usvg::Tree,
    opt: &Options
) -> Option<Image> {
    None
}
