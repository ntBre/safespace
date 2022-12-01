use safespace::Generator;

fn main() {
    let width = 1000;
    let height = 500;
    let prob = 0.01;
    let radius = 2;
    let white_radius = 1;
    let red_max = 10;
    let blue_max = 10;

    let img = Generator::new()
        .width(width)
        .height(height)
        .prob(prob)
        .radius(radius)
        .white_radius(white_radius)
        .red_max(red_max)
        .blue_max(blue_max)
        .generate();

    img.write("test.png").unwrap();
}
