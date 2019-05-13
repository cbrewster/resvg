// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use piet::{RenderContext, LinearGradient, RadialGradient, Gradient, GradientStop,};
use piet_common::Piet;
use kurbo::Vec2;

use super::prelude::*;

pub fn linear_brush<'a>(
    gradient: &usvg::LinearGradient,
    opacity: usvg::Opacity,
    bbox: Rect,
    rc: &'a mut Piet
) -> <Piet<'a> as RenderContext>::Brush {
    let stops = gradient_stops(&gradient.base, opacity);
    let start = Vec2::new(gradient.x1, gradient.y1);
    let end = Vec2::new(gradient.x2, gradient.y2);
    let linear_gradient = LinearGradient {
        start,
        end,
        stops,
    };
    rc.gradient(Gradient::Linear(linear_gradient)).unwrap()
}

pub fn radial_brush<'a>(
    gradient: &usvg::RadialGradient,
    opacity: usvg::Opacity,
    bbox: Rect,
    rc: &'a mut Piet
) -> <Piet<'a> as RenderContext>::Brush {
    let stops = gradient_stops(&gradient.base, opacity);
    let center = Vec2::new(gradient.cx, gradient.cy);
    let origin_offset = Vec2::new(gradient.fx, gradient.fy);
    let radius = gradient.r.value();
    let radial_gradient = RadialGradient {
        center,
        origin_offset,
        stops,
        radius,
    };
    rc.gradient(Gradient::Radial(radial_gradient)).unwrap()
}

fn gradient_stops(
    gradient: &usvg::BaseGradient,
    opacity: usvg::Opacity
) -> Vec<GradientStop> {
    gradient.stops
        .iter()
        .map(|stop| {
            GradientStop {
                pos: stop.offset.value() as f32,
                rgba: color_to_rgba(stop.color, (stop.opacity.value() as f64 * opacity.value() * 255.0) as u8)
            }
        })
        .collect()
}
