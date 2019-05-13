// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use piet_common::Piet;
use piet::{RenderContext, StrokeStyle, FillRule};
use kurbo::{BezPath, Shape, Line};

use super::prelude::*;
use super::{
    fill,
    stroke,
};
use crate::backend_utils;


pub fn draw(
    tree: &usvg::Tree,
    path: &usvg::Path,
    opt: &Options,
    rc: &mut Piet
) -> Option<Rect> {
    let mut is_square_cap = false;
    if let Some(ref stroke) = path.stroke {
        is_square_cap = stroke.linecap == usvg::LineCap::Square;
    }

    let paths = draw_path(&path.segments, is_square_cap, rc);

    let bbox = utils::path_bbox(&path.segments, None, &usvg::Transform::default());

    // `usvg` guaranties that path without a bbox will not use
    // a paint server with ObjectBoundingBox,
    // so we can pass whatever rect we want, because it will not be used anyway.
    let style_bbox = bbox.unwrap_or_else(|| Rect::new(0.0, 0.0, 1.0, 1.0).unwrap());

    if path.visibility != usvg::Visibility::Visible {
        return bbox;
    }

    // TODO: Support antialiasing
    if let Some(fill) = path.fill.as_ref() {
        let brush = fill::brush(tree, fill, opt, style_bbox, rc);
        let rule = fill::rule(fill);
        for path in &paths {
            rc.fill(path, &brush, rule);
        }
    }

    // if let Some(stroke) = path.stroke.as_ref() {
    //     let brush = stroke::brush(tree, stroke, opt, style_bbox, rc);
    //     let width = stroke::width(stroke);
    //     let style = stroke::style(stroke);
    //     for path in &paths {
    //         rc.stroke(path, &brush, width, style);
    //     }
    // }

    println!("Drawing Path!");

    bbox
}

fn draw_path(
    segments: &[usvg::PathSegment],
    is_square_cap: bool,
    rc: &mut Piet
) -> Vec<impl Shape> {
    let mut i = 0;
    let mut paths = vec![];
    loop {
        let subpath = get_subpath(i, segments);
        if subpath.is_empty() {
            break;
        }

        paths.push(create_subpath(subpath, is_square_cap, rc));
        i += subpath.len();
    }
    paths
}

fn get_subpath(start: usize, segments: &[usvg::PathSegment]) -> &[usvg::PathSegment] {
    let mut i = start;
    while i < segments.len() {
        match segments[i] {
            usvg::PathSegment::MoveTo { .. } => {
                if i != start {
                    break;
                }
            }
            usvg::PathSegment::ClosePath => {
                i += 1;
                break;
            }
            _ => {}
        }

        i += 1;
    }

    &segments[start..i]
}

fn create_subpath(
    segments: &[usvg::PathSegment],
    is_square_cap: bool,
    rc: &mut Piet
) -> impl Shape {
    assert_ne!(segments.len(), 0);
    let mut path = BezPath::new();

    // TODO: Check zero path case?
    for seg in segments {
        match *seg {
            usvg::PathSegment::MoveTo { x, y } => {
                path.moveto((x, y));
            }
            usvg::PathSegment::LineTo { x, y } => {
                path.lineto((x, y));
            }
            usvg::PathSegment::CurveTo { x1, y1, x2, y2, x, y } => {
                path.curveto((x1, y1), (x2, y2), (x, y));
            }
            usvg::PathSegment::ClosePath => {
                path.closepath();
            }
        }
    }

    path
}