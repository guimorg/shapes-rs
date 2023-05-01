use std::f64::consts::PI;

fn render(a: f64, b: f64) {
    let theta_spacing: f64 = 0.07;
    let phi_spacing: f64 = 0.02;

    let r1: f64 = 4.;
    let r2: f64 = 4.;
    let k2: f64 = 40.;
    let (w, h) = (45, 100);
    let k1 = w as f64 * k2 * 3. / (8. * (r1 + r2));

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

        let mut phi: f64 = 0.0;
        while phi < two_pi {
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();

            let circle_x: f64 = r2 + r1 * cos_theta;
            let circle_y: f64 = r1 * sin_theta;

            let x: f64 =
                circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi) - circle_y * cos_a * cos_b;
            let y: f64 =
                circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi) + circle_y * cos_a * cos_b;
            let z: f64 = k2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
            let ooz: f64 = 1. / z;

            let xp = (w as f64 / 2. + k1 * ooz * x) as usize;
            let yp = (h as f64 / 2. - k1 * ooz * y) as usize;

            if xp < w && yp < h {
                let l: f64 =
                    cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta
                        + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);

                if l > 0. {
                    let z_current = z_buffer[xp][yp];
                    if ooz > z_current {
                        z_buffer[xp][yp] = ooz;
                        let luminance_index = (l * 8.) as usize;
                        output_buffer[xp][yp] = light_chars[luminance_index];
                    }
                }
            }

            phi += phi_spacing;
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
