use crate::prelude::*;

/// Piet backend
#[derive(Clone, Copy)]
pub struct Backend;

impl Render for Backend {
    fn render_to_image(&self, tree: &usvg::Tree, opt: &Options) -> Option<Box<OutputImage>> {
        None
    }

    fn render_node_to_image(&self, node: &usvg::Node, opt: &Options) -> Option<Box<OutputImage>> {
        None
    }

    fn calc_node_bbox(&self, node: &usvg::Node, opt: &Options) -> Option<Rect> {
        None
    }
}

