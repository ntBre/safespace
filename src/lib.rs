use image::draw::Draw;
use image::image::draw::{Circle, Line};
use image::{Image, Rgba};
use once_cell::sync::Lazy;
use rand::prelude::*;
use std::collections::HashMap;
use std::time::UNIX_EPOCH;

struct Constellation {
    /// x,y locations of the stars relative to position. TODO account for scale
    nodes: Vec<(isize, isize)>,

    /// indices of nodes that are connected, eg (0,1) means the first two nodes
    /// should have a line between them
    edges: Vec<(usize, usize)>,
}

impl Constellation {
    // could also have a position to translate to
    fn draw(
        &self,
        img: &mut Image,
        node_shape: impl Draw<Endpoint = usize>,
        line: Line,
        color: Rgba,
        position: (usize, usize),
        scale: f64,
    ) {
        let (dx, dy) = position;
        let (dx, dy) = (dx as isize, dy as isize);
        for (x, y) in &self.nodes {
            node_shape.draw(
                img,
                (scale * (x + dx) as f64) as usize,
                (scale * (y + dy) as f64) as usize,
                color,
            );
        }

        for (a, b) in &self.edges {
            line.draw(img, self.nodes(a, dx, dy), self.nodes(b, dx, dy), color);
        }
    }

    fn nodes(&self, a: &usize, dx: isize, dy: isize) -> (usize, usize) {
        let (x, y) = self.nodes[*a];
        ((x + dx) as usize, (y + dy) as usize)
    }
}

static CONSTELLATIONS: Lazy<HashMap<&'static str, Constellation>> =
    Lazy::new(|| {
        HashMap::from([(
            "big-dipper",
            Constellation {
                nodes: vec![
                    (-200, 10),
                    (-100, -50),
                    (-25, -30),
                    (75, -10),
                    (340, -70),
                    (350, 90),
                    (150, 130),
                ],
                edges: vec![
                    //
                    (0, 1),
                    (1, 2),
                    (2, 3),
                    (3, 4),
                    (4, 5),
                    (6, 5),
                    (3, 6),
                ],
            },
        )])
    });

pub struct Generator {
    width: usize,
    height: usize,
    white_radius: usize,
    prob: f64,
    radius: usize,
    red_max: i32,
    blue_max: i32,
    seed: u64,
    constellation: String,

    /// size of the stars
    constellation_size: usize,

    /// width of the connecting lines
    constellation_width: usize,
}

/// build the setters needed for a builder macro
macro_rules! builder {
    ($($field: ident, $type: ty;)*) => {
	$(
	    pub fn $field(mut self, val: $type) -> Self {
		self.$field = val;
		self
	    }
	)*
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator {
    builder! {
    width, usize;
    height, usize;
    white_radius, usize;
    prob, f64;
    radius, usize;
    red_max, i32;
    blue_max, i32;
    seed, u64;
    constellation, String;
    constellation_size, usize;
    constellation_width, usize;
    }

    pub fn new() -> Self {
        Self {
            width: 1000,
            height: 500,
            prob: 0.01,
            radius: 2,
            white_radius: 1,
            red_max: 10,
            blue_max: 10,
            seed: UNIX_EPOCH.elapsed().unwrap().as_secs(),
            constellation: "".to_owned(),
            constellation_size: 4,
            constellation_width: 2,
        }
    }

    /// generate a new image of space with dimensions `width`x`height`, white stars
    /// with radius `white_radius`, the probability of spawning a white star at
    /// `prob`, the radius of the large stars set to `radius`, and the maximum
    /// numbers of red and blue stars set to `red_max` and `blue_max` respectively.
    /// you can also optionally set the random seed by passing `Some<seed>`.
    pub fn generate(&self) -> Image {
        let red = Rgba::new(255, 112, 3, 255);
        let blue = Rgba::new(0, 112, 255, 255);
        let mut img = Image::new(self.width, self.height);
        img.fill(Rgba::black());
        let star = Circle::new(self.white_radius);
        let mut rng = StdRng::seed_from_u64(self.seed);
        for row in 0..self.height {
            for col in 0..self.width {
                if rng.gen::<f64>() < self.prob {
                    star.draw(&mut img, col, row, Rgba::white());
                }
            }
        }
        let star = Circle::new(self.radius);
        for _ in 0..self.red_max {
            star.draw(
                &mut img,
                rng.gen_range(0..self.width),
                rng.gen_range(0..self.height),
                red,
            );
        }
        for _ in 0..self.blue_max {
            star.draw(
                &mut img,
                rng.gen_range(0..self.width),
                rng.gen_range(0..self.height),
                blue,
            );
        }

        let c = Circle::new(self.constellation_size);
        let l = Line::new(self.constellation_width);
        if let Some(con) = CONSTELLATIONS.get(&self.constellation[..]) {
            con.draw(&mut img, c, l, Rgba::green(), (500, 250), 0.5);
        }
        img
    }
}
