use std::f64::consts::PI;

fn render(a: f64, b: f64) {
    let theta_spacing: f64 = 0.07;
    let u_spacing: f64 = 0.02;

    let r: f64 = 7.;
    let c_h: f64 = 20.;
    let k2: f64 = 40.;
    let (w, h) = (45, 100);
    let k1 = w as f64 * k2 * 3. / (8. * (r + c_h / 2.));

    let mut output_buffer: Vec<Vec<char>> = vec![vec![' '; h]; w];
    let mut z_buffer: Vec<Vec<f64>> = vec![vec![0.; h]; w];

    let light_chars: Vec<char> = vec!['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

    let cos_a = a.cos();
    let cos_b = b.cos();
    let sin_a = a.sin();
    let sin_b = b.sin();

    let two_pi = 2.0 * PI;
    let mut theta: f64 = 0.0;

    while theta < two_pi {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        let mut u: f64 = 0.0;
        while u < 1. {
            let circle_x: f64 = r * cos_theta;
            let circle_y: f64 = r * sin_theta;
            let circle_h: f64 = c_h * u;

            let x: f64 = circle_x * cos_b - circle_y * cos_a * sin_b + circle_h * sin_a * sin_b;
            let y: f64 = circle_x * sin_b + circle_y * cos_a * cos_b - circle_h * sin_a * cos_b;
            let z: f64 = k2 + circle_y * sin_a + circle_h * cos_a;
            let ooz: f64 = 1. / z;

            let xp = (w as f64 / 2. + k1 * ooz * x) as usize;
            let yp = (h as f64 / 2. - k1 * ooz * y) as usize;

            if xp < w && yp < h {
                let l: f64 = sin_b * cos_theta + sin_theta * cos_a * cos_b - sin_theta * sin_a;
                let z_current = z_buffer[xp][yp];
                if ooz > z_current {
                    z_buffer[xp][yp] = ooz;
                    let luminance_index = (l * 8.) as usize;
                    output_buffer[xp][yp] = light_chars[luminance_index];
                }
            }

            u += u_spacing;
        }

        theta += theta_spacing;
    }

    println!("\x1b[H");

    for i in 0..output_buffer.len() {
        for j in 0..output_buffer[i].len() {
            print!("{}", output_buffer[i][j]);
        }
        println!("");
    }
}

fn main() {
    let mut a = 1.;
    let mut b = 2.;

    loop {
        render(a, b);
        a -= 0.07;
        b += 0.02;
        std::thread::sleep(std::time::Duration::from_millis(15));
    }
}
