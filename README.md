# drawing-stuff

`drawing-stuff` is a collection of utilities to make drawing onto a canvas / pixel buffer easy.

This version of the library is definetely not fully featured and also not fully documented as its mostly thought for my personal use.

## Examples

```rust
use drawing_stuff::canvas::Canvas;
use drawing_stuff::color::{RGB, WHITE};

const WIDTH: usize = 1080;
const HEIGHT: usize = 720;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    // get color of pixel
    canvas.get(200, 100);

    // set color of pixel
    canvas.set(200, 100, RGB { r: 255, g: 255, b: 255 });

    // draw single pixel
    canvas.draw_pixel(200, 100, WHITE);

    // draw line
    canvas.draw_line(200, 100, 500, 700, WHITE);

    // draw circle
    canvas.draw_circle(200, 100, 15, WHITE);
    canvas.draw_circle_solid(200, 100, 15, WHITE);

    // draw polygon
    let vertices = vec![(200, 100), (500, 700), (300, 800)]; // clockwise
    canvas.draw_polygon(&vertices, WHITE);
    canvas.draw_polygon_solid(&vertices, true, WHITE);
}
```

### Creating custom drawables

```rust
use drawing_stuff::canvas::{Canvas, Draw};
use drawing_stuff::color::{RGBA, WHITE};

pub struct Circle {
    pub center: (isize, isize),
    pub radius: u32,
    pub solid: bool,

    pub color: RGBA,
}

impl Draw for Circle {
    fn draw(&self, canvas: &mut Canvas) {
       match self.solid {
          true => canvas.draw_circle_solid(self.center.0, self.center.1, self.radius, self.color),
          false => canvas.draw_circle(self.center.0, self.center.1, self.radius, self.color),
      }
    }
}

const WIDTH: usize = 1080;
const HEIGHT: usize = 720;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let circle = Circle {
        center: (200, 100),
        radius: 15,
        solid: true,
        color: WHITE,
    };

    canvas.draw(&circle);
    // or
    circle.draw(&mut canvas);
}
```

License: MIT
