// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Piet backend implementation

use crate::prelude::*;
use crate::{
    backend_utils,
    layers,
};

use piet_common::{BitmapTarget, Device, Piet};
use piet::{ImageFormat, RenderContext};
use kurbo::{Affine, Line};
use usvg::Color;

mod path;
mod fill;
mod stroke;
mod gradient;

mod prelude {
    pub use super::super::prelude::*;
    pub use super::color_to_rgba;
}

type PietLayers<'a> = layers::Layers<BitmapTarget<'a>>;

impl ConvTransform<Affine> for usvg::Transform {
    fn to_native(&self) -> Affine {
        Affine::new([self.a, self.b, self.c, self.d, self.e, self.f])
    }

    fn from_native(ts: &Affine) -> Self {
        let a = ts.as_coeffs();
        Self::new(a[0], a[1], a[2], a[3], a[4], a[5])
    }
}

pub fn color_to_rgba(color: Color, alpha: u8) -> u32 {
    (color.red as u32) << 24
    | (color.green as u32) << 16
    | (color.blue as u32) << 8
    | (alpha as u32)
}

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
    opt: &Options,
) -> Option<Image> {
    let device = Device::new().ok()?;
    let (mut bitmap, img_view) = create_surface(
        &device,
        tree.svg_node().size.to_screen_size(),
        opt,
    )?;

    let mut rc = bitmap.render_context();

    if let Some(color) = opt.background {
        let color = (color.red as u32) << 16 | (color.green as u32) << 8 | color.blue as u32;
        rc.clear(0xFFFFFF);
    }

    render_to_canvas(tree, opt, img_view, &mut rc);

    rc.finish().unwrap();

    let pixels = bitmap.into_raw_pixels(ImageFormat::RgbaPremul).unwrap();

    Some(Image {
        pixels,
        width: img_view.width(),
        height: img_view.height(),
    })
}

/// Renders SVG to canvas.
pub fn render_to_canvas(
    tree: &usvg::Tree,
    opt: &Options,
    img_size: ScreenSize,
    rc: &mut Piet,
) {
    render_node_to_canvas(&tree.root(), opt, tree.svg_node().view_box, img_size, rc);
}

/// Renders SVG node to canvas.
pub fn render_node_to_canvas(
    node: &usvg::Node,
    opt: &Options,
    view_box: usvg::ViewBox,
    img_size: ScreenSize,
    rc: &mut Piet
) {
    let mut layers = create_layers(img_size, opt);

    apply_viewbox_transform(view_box, img_size, rc);

    let mut ts = utils::abs_transform(node);
    ts.append(&node.transform());

    rc.save().unwrap();
    rc.transform(ts.to_native());
    render_node(node, opt, &mut layers, rc);
    rc.restore();
}

/// Applies viewbox transformation to the painter.
fn apply_viewbox_transform(
    view_box: usvg::ViewBox,
    img_size: ScreenSize,
    rc: &mut Piet,
) {
    let ts = utils::view_box_to_transform(view_box.rect, view_box.aspect, img_size.to_size());
    rc.transform(ts.to_native());
}

fn render_node(
    node: &usvg::Node,
    opt: &Options,
    layers: &mut PietLayers,
    rc: &mut Piet,
) -> Option<Rect> {
    match *node.borrow() {
        usvg::NodeKind::Svg(_) => {
            Some(render_group(node, opt, layers, rc))
        }
        usvg::NodeKind::Path(ref path) => {
            path::draw(&node.tree(), path, opt, rc)
        }
        _ => None
    }
}

fn render_group(
    parent: &usvg::Node,
    opt: &Options,
    layers: &mut PietLayers,
    rc: &mut Piet
) -> Rect {
    let mut g_bbox = Rect::new_bbox();

    for node in parent.children() {
        rc.save();

        rc.transform(node.transform().to_native());

        let bbox = render_node(&node, opt, layers, rc);

        if let Some(bbox) = bbox {
            let bbox = bbox.transform(&node.transform()).unwrap();
            g_bbox = g_bbox.expand(bbox);
        }

        rc.restore();
    }

    g_bbox
}

fn create_surface<'a>(
    device: &'a Device,
    size: ScreenSize,
    opt: &Options,
) -> Option<(BitmapTarget<'a>, ScreenSize)> {
    let img_size = utils::fit_to(size, opt.fit_to)?;

    // TODO: Support DPI
    let bitmap = device.bitmap_target(size.width() as usize, size.height() as usize, 1.0).ok()?;

    Some((bitmap, img_size))
}

fn create_layers<'a>(img_size: ScreenSize, opt: &Options) -> PietLayers<'a> {
    layers::Layers::new(img_size, opt.usvg.dpi, create_subsurface, clear_subsurface)
}

// TODO: Implement surfaces.
// Cannot create a subsurface without a device.
fn create_subsurface<'a>(
    size: ScreenSize,
    _: f64,
) -> Option<BitmapTarget<'a>> {
    None
}

fn clear_subsurface(surface: &mut BitmapTarget<'_>) {
    let mut render_context = surface.render_context();
    render_context.clear(0xffffff);
}

