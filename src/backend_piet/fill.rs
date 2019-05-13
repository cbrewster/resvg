// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use piet::{RenderContext, FillRule};
use piet_common::Piet;

use super::prelude::*;
use super::{
    gradient,
};

pub fn brush<'a>(
    tree: &usvg::Tree,
    fill: &usvg::Fill,
    opt: &Options,
    bbox: Rect,
    rc: &'a mut Piet
) -> <Piet<'a> as RenderContext>::Brush {
    match fill.paint {
        usvg::Paint::Color(color) => {
            rc.solid_brush(color_to_rgba(color, 0xff)).unwrap()
        }
        usvg::Paint::Link(ref id) => {
            if let Some(node) = tree.defs_by_id(id) {
                match *node.borrow() {
                    usvg::NodeKind::LinearGradient(ref lg) => {
                        gradient::linear_brush(lg, fill.opacity, bbox, rc)
                    }
                    usvg::NodeKind::RadialGradient(ref rg) => {
                        gradient::radial_brush(rg, fill.opacity, bbox, rc)
                    }
                    // TODO: Pattern
                    _ => {
                        println!("Hit Unsupported Pattern");
                        rc.solid_brush(0x00_00_00_00).unwrap()
                    }
                }
            } else {
                rc.solid_brush(0x00_00_00_00).unwrap()
            }
        }
    }
}

pub fn rule(
    fill: &usvg::Fill
) -> FillRule {
    match fill.rule {
        usvg::FillRule::NonZero => FillRule::NonZero,
        usvg::FillRule::EvenOdd => FillRule::EvenOdd,
    }
}
