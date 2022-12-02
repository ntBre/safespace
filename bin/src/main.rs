use clap::Parser;
use safespace::Generator;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// width of the output image
    #[arg(short = 'x', long, default_value_t = 1000)]
    width: usize,

    /// height of the output image
    #[arg(short = 'y', long, default_value_t = 500)]
    height: usize,

    /// radius of the small white stars
    #[arg(short, long, default_value_t = 1)]
    white_radius: usize,

    /// probability of spawning a star in each pixel
    #[arg(short, long, default_value_t = 0.01)]
    prob: f64,

    /// radius of the large red and blue stars
    #[arg(short, long, default_value_t = 2)]
    radius: usize,

    /// maximum number of red stars
    #[arg(short = 'n', long, default_value_t = 10)]
    red_max: i32,

    /// maximum number of blue stars
    #[arg(short, long, default_value_t = 10)]
    blue_max: i32,

    /// maximum number of blue stars
    #[arg(short, long)]
    seed: Option<u64>,

    /// output file
    output: String,

    /// constellations. currently supports big-dipper only
    #[arg(short, long, default_value_t = String::from(""))]
    constellation: String,
}
fn main() {
    let args = Args::parse();
    let mut gen = Generator::new()
        .width(args.width)
        .height(args.height)
        .prob(args.prob)
        .radius(args.radius)
        .white_radius(args.white_radius)
        .red_max(args.red_max)
        .blue_max(args.blue_max)
        .constellation(args.constellation);

    if let Some(seed) = args.seed {
        gen = gen.seed(seed);
    }

    let img = gen.generate();

    img.write(args.output).unwrap();
}
