use crate::canvas::{Canvas, Draw};
use crate::color::RGBA;

#[derive(Debug)]
pub enum AnkerType {
    CENTER,
    CORNER, // top-left
}

#[derive(Debug)]
pub struct Line {
    pub end1: (f32, f32),
    pub end2: (f32, f32),

    pub width: f32,
    pub anti_aliased: bool,
    pub capped: bool,

    pub color: RGBA,
}

impl Draw for Line {
    fn draw(&self, canvas: &mut Canvas) {
        if self.width == 0.0 {
            return;
        };

        if self.width == 1.0 {
            match self.anti_aliased {
                true => canvas.draw_line_aa(
                    self.end1.0,
                    self.end1.1,
                    self.end2.0,
                    self.end2.1,
                    self.color,
                ),
                false => canvas.draw_line(
                    self.end1.0.round() as isize,
                    self.end1.1.round() as isize,
                    self.end2.0.round() as isize,
                    self.end2.1.round() as isize,
                    self.color,
                ),
            }

            return;
        }

        match (self.anti_aliased, self.capped) {
            (true, true) => canvas.draw_polyline_capped_aa(
                self.end1.0,
                self.end1.1,
                self.end2.0,
                self.end2.1,
                self.width,
                self.color,
            ),
            (true, false) => canvas.draw_polyline_aa(
                self.end1.0,
                self.end1.1,
                self.end2.0,
                self.end2.1,
                self.width,
                self.color,
            ),
            (false, true) => canvas.draw_polyline_capped(
                self.end1.0.round() as isize,
                self.end1.1.round() as isize,
                self.end2.0.round() as isize,
                self.end2.1.round() as isize,
                self.width.round() as u32,
                self.color,
            ),
            (false, false) => canvas.draw_polyline(
                self.end1.0.round() as isize,
                self.end1.1.round() as isize,
                self.end2.0.round() as isize,
                self.end2.1.round() as isize,
                self.width.round() as u32,
                self.color,
            ),
        }
    }
}

#[derive(Debug)]
pub struct Circle {
    pub center: (f32, f32),
    pub radius: f32,

    pub anti_aliased: bool,
    pub solid: bool,

    pub color: RGBA,
}

impl Draw for Circle {
    fn draw(&self, canvas: &mut Canvas) {
        match (self.anti_aliased, self.solid) {
            (true, true) => {
                canvas.draw_circle_solid_aa(self.center.0, self.center.1, self.radius, self.color)
            }
            (true, false) => {
                canvas.draw_circle_aa(self.center.0, self.center.1, self.radius, self.color)
            }
            (false, true) => canvas.draw_circle_solid(
                self.center.0.round() as isize,
                self.center.1.round() as isize,
                self.radius.round() as u32,
                self.color,
            ),
            (false, false) => canvas.draw_circle(
                self.center.0.round() as isize,
                self.center.1.round() as isize,
                self.radius.round() as u32,
                self.color,
            ),
        }
    }
}

#[derive(Debug)]
pub struct Square {
    pub anker: (isize, isize),
    pub length: u32,

    pub anker_type: AnkerType,
    pub solid: bool,

    pub color: RGBA,
}

impl Draw for Square {
    fn draw(&self, canvas: &mut Canvas) {
        let vertices = match self.anker_type {
            AnkerType::CENTER => vec![
                (
                    self.anker.0 - self.length as isize / 2,
                    self.anker.1 - self.length as isize / 2,
                ),
                (
                    self.anker.0 + self.length as isize / 2,
                    self.anker.1 - self.length as isize / 2,
                ),
                (
                    self.anker.0 + self.length as isize / 2,
                    self.anker.1 + self.length as isize / 2,
                ),
                (
                    self.anker.0 - self.length as isize / 2,
                    self.anker.1 + self.length as isize / 2,
                ),
            ],
            AnkerType::CORNER => vec![
                self.anker,
                (self.anker.0 + self.length as isize, self.anker.1),
                (
                    self.anker.0 + self.length as isize,
                    self.anker.1 + self.length as isize,
                ),
                (self.anker.0, self.anker.1 + self.length as isize),
            ],
        };

        match self.solid {
            true => canvas.draw_polygon_solid(&vertices, true, self.color),
            false => canvas.draw_polygon(&vertices, self.color),
        }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub anker: (isize, isize),
    pub width: u32,
    pub height: u32,

    pub anker_type: AnkerType,
    pub solid: bool,

    pub color: RGBA,
}

impl Draw for Rectangle {
    fn draw(&self, canvas: &mut Canvas) {
        let vertices = match self.anker_type {
            AnkerType::CENTER => vec![
                (
                    self.anker.0 - self.width as isize / 2,
                    self.anker.1 - self.height as isize / 2,
                ),
                (
                    self.anker.0 + self.width as isize / 2,
                    self.anker.1 - self.height as isize / 2,
                ),
                (
                    self.anker.0 + self.width as isize / 2,
                    self.anker.1 + self.height as isize / 2,
                ),
                (
                    self.anker.0 - self.width as isize / 2,
                    self.anker.1 + self.height as isize / 2,
                ),
            ],
            AnkerType::CORNER => vec![
                self.anker,
                (self.anker.0 + self.width as isize, self.anker.1),
                (
                    self.anker.0 + self.width as isize,
                    self.anker.1 + self.height as isize,
                ),
                (self.anker.0, self.anker.1 + self.height as isize),
            ],
        };

        match self.solid {
            true => canvas.draw_polygon_solid(&vertices, true, self.color),
            false => canvas.draw_polygon(&vertices, self.color),
        }
    }
}

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<(isize, isize)>,

    pub clockwise: bool,
    pub solid: bool,

    pub color: RGBA,
}

impl Draw for Polygon {
    fn draw(&self, canvas: &mut Canvas) {
        match self.solid {
            true => canvas.draw_polygon_solid(&self.vertices, self.clockwise, self.color),
            false => canvas.draw_polygon(&self.vertices, self.color),
        }
    }
}
