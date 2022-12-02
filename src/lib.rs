use image::draw::Draw;
use image::image::draw::{Circle, Line};
use image::{Image, Rgba};
use once_cell::sync::Lazy;
use rand::prelude::*;
use std::collections::HashMap;
use std::time::UNIX_EPOCH;

struct Constellation {
    /// x,y locations of the stars, eg (500, 250) for the center of a 1000x500
    /// image
    nodes: Vec<(usize, usize)>,

    /// indices of nodes that are connected, eg (0,1) means the first two nodes
    /// should have a line between them
    edges: Vec<(usize, usize)>,

    /// the size of the canvas for which the constellation was designed. used
    /// for resizing
    _scale: (usize, usize),
}

impl Constellation {
    // could also have a position to translate to
    fn draw(
        &self,
        img: &mut Image,
        node_shape: impl Draw<Endpoint = usize>,
        line: Line,
        color: Rgba,
    ) {
        for (x, y) in &self.nodes {
            node_shape.draw(img, *x, *y, color);
        }

        for (a, b) in &self.edges {
            line.draw(img, self.nodes[*a], self.nodes[*b], color);
        }
    }
}

static CONSTELLATIONS: Lazy<HashMap<&'static str, Constellation>> =
    Lazy::new(|| {
        HashMap::from([(
            "big-dipper",
            Constellation {
                nodes: vec![
                    (300, 260),
                    (400, 200),
                    (475, 220),
                    (575, 240),
                    (840, 180),
                    (850, 340),
                    (650, 380),
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
                _scale: (1000, 500),
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

        // TODO make these flags too
        let c = Circle::new(20);
        let l = Line::new(20);
        if let Some(con) = CONSTELLATIONS.get(&self.constellation[..]) {
            con.draw(&mut img, c, l, Rgba::green());
        }
        img
    }
}
