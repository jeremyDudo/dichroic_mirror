extern crate nalgebra as na;
use na::{Matrix2, Complex};

extern crate palette;
use palette::{LinSrgba, Srgba, Pixel, Blend};
use palette::blend::{Equations, Parameter};

extern crate image;
use image::{ImageBuffer, Rgba};

// Matrices for incidence and propagation through media
fn d_e(n:&f32, theta:&f32) -> Matrix2<Complex<f32>> {
    Matrix2::new(Complex::new(1.0, 0.0), Complex::new(1.0, 0.0), n*theta.cos()*Complex::new(1.0, 0.0), -1.0*n*theta.cos()*Complex::new(1.0,0.0) )
}
fn d_m(n:&f32, theta:&f32) -> Matrix2<Complex<f32>> {
    Matrix2::new(theta.cos()*Complex::new(1.0, 0.0), -1.0*theta.cos()*Complex::new(1.0, 0.0), *n*Complex::new(1.0, 0.0), *n*Complex::new(1.0, 0.0))
}
fn k_func(n:&f32, lambda:&f32, theta:&f32) -> f32 {
    n*2.0*std::f32::consts::PI / lambda * theta.cos()
}
fn p_mat(n:&f32, d:&f32, lambda:&f32, theta:&f32) -> Matrix2<Complex<f32>> {
    let k = k_func(n,lambda, theta);
    Matrix2::new((k*d*Complex::new(0.0,1.0)).exp(), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), (-1_f32*k*d*Complex::new(0.0,1.0)).exp())
}

// make some math easier for later
fn matrix_power(matr: Matrix2<Complex<f32>>, powr: u32) -> Matrix2<Complex<f32>>{
    let mut fin = matr;
    let mut count = powr;
    // Should include a catch for negative powers, but the power should always be positive in this case so I'm being lazy
    while count > 0 {
        fin = fin*matr;
        count -= 1;
    }
    fin
}

// make the mirror stack of materials
fn mirror(n1:&f32, n2:&f32, d1:&f32, d2:&f32, d_sub:&f32, total_layers:&u32, polarization:&bool, lambda:&f32) -> Matrix2<Complex<f32>> {
    let theta1 = (45.0_f32.sin()/n1).asin();
    let theta2 = (n1*theta1.sin()/n2).asin();

    if *polarization {

        if total_layers % 2 == 0 {
            let n_sub = n1;
            let theta_sub = theta1;
            d_e(&n_sub, &d_sub).try_inverse().unwrap()*d_e(&1.0, &45.0) * p_mat(&n_sub, &d_sub, &lambda, &theta_sub) * matrix_power( d_e(&n1, &d1).try_inverse().unwrap()*d_e(&n2, &d2) * p_mat(&n2, &d2, &lambda, &theta2) * d_e(&n2, &d2).try_inverse().unwrap()*d_e(&n1, &d1)*p_mat(&n1, &d1, &lambda, &theta1), total_layers/2) * d_e(&1.0, &45.0).try_inverse().unwrap()*d_e(&n1, &theta1)

        } else {
            let n_sub = n2;
            let theta_sub = theta2;
            d_e(&n_sub, &d_sub).try_inverse().unwrap()*d_e(&1.0, &45.0) * p_mat(&n_sub, &d_sub, &lambda, &theta_sub) * d_e(&n2, &d2).try_inverse().unwrap() * d_e(&n1, &d1) * p_mat(&n1, &d1, &lambda, &theta1) * matrix_power( d_e(&n1, &d1).try_inverse().unwrap()*d_e(&n2, &d2) * p_mat(&n2, &d2, &lambda, &theta2) * d_e(&n2, &d2).try_inverse().unwrap()*d_e(&n1, &d1)*p_mat(&n1, &d1, &lambda, &theta1) , (total_layers-1)/2) * d_e(&1.0, &45.0).try_inverse().unwrap()*d_e(&n1, &theta1)
        }

    } else {

        if total_layers % 2 == 0 {
            let n_sub = n1;
            let theta_sub = theta1;
            d_m(&n_sub, &d_sub).try_inverse().unwrap()*d_m(&1.0, &45.0) * p_mat(&n_sub, &d_sub, &lambda, &theta_sub) * matrix_power( d_m(&n1, &d1).try_inverse().unwrap()*d_m(&n2, &d2) * p_mat(&n2, &d2, &lambda, &theta2) * d_m(&n2, &d2).try_inverse().unwrap()*d_m(&n1, &d1)*p_mat(&n1, &d1, &lambda, &theta1), total_layers/2) * d_m(&1.0, &45.0).try_inverse().unwrap()*d_m(&n1, &theta1)

        } else {
            let n_sub = n2;
            let theta_sub = theta2;
            d_m(&n_sub, &d_sub).try_inverse().unwrap()*d_m(&1.0, &45.0) * p_mat(&n_sub, &d_sub, &lambda, &theta_sub) * d_m(&n2, &d2).try_inverse().unwrap() * d_m(&n1, &d1) * p_mat(&n1, &d1, &lambda, &theta1) * matrix_power( d_m(&n1, &d1).try_inverse().unwrap()*d_m(&n2, &d2) * p_mat(&n2, &d2, &lambda, &theta2) * d_m(&n2, &d2).try_inverse().unwrap()*d_m(&n1, &d1)*p_mat(&n1, &d1, &lambda, &theta1), (total_layers-1)/2) * d_m(&1.0, &45.0).try_inverse().unwrap()*d_m(&n1, &theta1)
        }

    }
}

// determine reflectivity
fn reflect(n1:&f32, n2:&f32, d1:&f32, d2:&f32, d_sub:&f32, total_layers:&u32, polarization:&bool, lambda:&f32) -> f32 {
    // Finiding the intensity of reflected light from the reflection matrix "r"
    let r = mirror(n1,n2,d1,d2,d_sub,total_layers,polarization,lambda);
    let top = r.row(1).column(0).norm();
    let bottom = r.row(1).column(1).norm();
    let intensity = (top/bottom).powf(2.0);

    intensity

    // Pass the intensity through the wavelength_to_rgb filter to produce the rgb... 
    // Assumption: going to use the intensity as the alpha value (I don't have a good idea as to whether this is the best method)
    // let rgba = wavelength_to_rgba(*lambda, intensity);

    // rgba 
}
// determine transmittivity (1 - reflectivity)
fn transmit(n1:&f32, n2:&f32, d1:&f32, d2:&f32, d_sub:&f32, total_layers:&u32, polarization:&bool, lambda:&f32) -> f32 {
    let r = reflect(n1,n2,d1,d2,d_sub,total_layers,polarization,lambda);
    let t = 1.0 - r;

    t
}

// to improve from the Julia implementation, I thought to show colors instead of just an intensity spectrum
// so I take the colors in rgb to be able to blend the spectrum into a color
fn wavelength_to_rgba(wavelength:f32, alpha:f32) -> LinSrgba<f32> {
    let gamma = 0.8;    // IDK what this gamma is but it is kinda important?
    let r:f32;
    let g:f32;
    let b:f32;
    if wavelength>= 380.0 && wavelength <= 440.0{
        let attenuation: f32 = 0.3 + 0.7 * (wavelength - 380.0) / (440.0 - 380.0);
        r = ((-1.0*(wavelength - 440.0) / (440.0 - 380.0)) * attenuation).powf(gamma);
        g = 0.0;
        b = (1.0 * attenuation).powf(gamma);
    } else if wavelength >= 440.0 && wavelength <= 490.0{
        r = 0.0;
        g = ((wavelength - 440.0) / (490.0 - 440.0)).powf(gamma);
        b = 1.0;
    } else if wavelength >= 490.0 && wavelength <= 510.0{
        r = 0.0;
        g = 1.0;
        b = (-(wavelength - 510.0) / (510.0 - 490.0)).powf(gamma);
    } else if wavelength >= 510.0 && wavelength <= 580.0{
        r = ((wavelength - 510.0) / (580.0 - 510.0)).powf(gamma);
        g = 1.0;
        b = 0.0;
    } else if wavelength >= 580.0 && wavelength <= 645.0{
        r = 1.0;
        g = (-(wavelength - 645.0) / (645.0 - 580.0)).powf(gamma);
        b = 0.0;
    }else if wavelength >= 645.0 && wavelength <= 750.0{
        let attenuation: f32 = 0.3 + 0.7 * (750.0 - wavelength) / (750.0 - 645.0);
        r = (1.0 * attenuation).powf(gamma);
        g = 0.0;
        b = 0.0;
    } else {
        r = 0.0;
        g = 0.0;
        b = 0.0;
    }

    LinSrgba::new(r, g, b, alpha)
}

// function for each combination of TE, TM with Reflection, Transmission
pub fn elec_pol_reflect(n1:&f32, n2:&f32, d1:&f32, d2:&f32, d_sub:&f32, total_layers:&u32) -> [u8; 4] {
    let mut _elec_colr: LinSrgba<f32> = LinSrgba::new(0.0, 0.0, 0.0, 0.0);
    let blend_mode = Equations::from_parameters(
        Parameter::SourceAlpha,
        Parameter::OneMinusSourceAlpha
    );
    let visible_light = 380..750;

    for lambda in visible_light{
        let _lambda = lambda as f32;

        let e_colr = wavelength_to_rgba(_lambda, reflect(n1, n2, d1, d2, d_sub, total_layers, &true, &_lambda));

        _elec_colr = _elec_colr.blend(e_colr, blend_mode);
    }
    let elec: [u8; 4] = Srgba::from_linear(_elec_colr).into_format().into_raw();

    elec
}
pub fn elec_pol_transmit(n1:&f32, n2:&f32, d1:&f32, d2:&f32, d_sub:&f32, total_layers:&u32) -> [u8; 4] {
    let mut _elec_colr: LinSrgba<f32> = LinSrgba::new(0.0, 0.0, 0.0, 0.0);
    let blend_mode = Equations::from_parameters(
        Parameter::SourceAlpha,
        Parameter::OneMinusSourceAlpha
    );
    let visible_light = 380..750;

    for lambda in visible_light{
        let _lambda = lambda as f32;

        let e_colr = wavelength_to_rgba(_lambda, transmit(n1, n2, d1, d2, d_sub, total_layers, &true, &_lambda));

        _elec_colr = _elec_colr.blend(e_colr, blend_mode);
    }
    let elec: [u8; 4] = Srgba::from_linear(_elec_colr).into_format().into_raw();

    elec
}
pub fn magn_pol_reflect(n1:&f32, n2:&f32, d1:&f32, d2:&f32, d_sub:&f32, total_layers:&u32) -> [u8; 4] {
    let mut _magn_colr: LinSrgba<f32> = LinSrgba::new(0.0, 0.0, 0.0, 0.0);
    let blend_mode = Equations::from_parameters(
        Parameter::SourceAlpha,
        Parameter::OneMinusSourceAlpha
    );
    let visible_light = 380..750;

    for lambda in visible_light{
        let _lambda = lambda as f32;

        let m_colr = wavelength_to_rgba(_lambda, reflect(n1, n2, d1, d2, d_sub, total_layers, &false, &_lambda));

        _magn_colr = _magn_colr.blend(m_colr, blend_mode);
    }
    let magn: [u8; 4] = Srgba::from_linear(_magn_colr).into_format().into_raw();

    magn
}
pub fn magn_pol_transmit(n1:&f32, n2:&f32, d1:&f32, d2:&f32, d_sub:&f32, total_layers:&u32) -> [u8; 4] {
    let mut _magn_colr: LinSrgba<f32> = LinSrgba::new(0.0, 0.0, 0.0, 0.0);
    let blend_mode = Equations::from_parameters(
        Parameter::SourceAlpha,
        Parameter::OneMinusSourceAlpha
    );
    let visible_light = 380..750;

    for lambda in visible_light{
        let _lambda = lambda as f32;

        let m_colr = wavelength_to_rgba(_lambda, transmit(n1, n2, d1, d2, d_sub, total_layers, &false, &_lambda));

        _magn_colr = _magn_colr.blend(m_colr, blend_mode);
    }
    let magn: [u8; 4] = Srgba::from_linear(_magn_colr).into_format().into_raw();

    magn
}

// make an image with quadrants showing the color expected from each combination of TE, TM with Reflection, Transmission
// TODO: Need labels on the image, as of right now, it requires having read the code to know what is what :(
pub fn quad_show(n1:&f32, n2:&f32, d1:&f32, d2:&f32, d_sub:&f32, total_layers:&u32) {
    let width: u32 = 400;
    let height: u32 = 400;

    let mut image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    let ele_r = elec_pol_reflect(n1, n2, d1, d2, d_sub, total_layers);
    let ele_t = elec_pol_transmit(n1, n2, d1, d2, d_sub, total_layers);
    let mag_r = magn_pol_reflect(n1, n2, d1, d2, d_sub, total_layers);
    let mag_t = magn_pol_transmit(n1, n2, d1, d2, d_sub, total_layers);

    for i in 0..200{
        for j in 0..200{
            image.put_pixel(i, j, Rgba([ele_r[0], ele_r[1], ele_r[2], 255]));
        }
    }

    for i in 201..400{
        for j in 0..200{
            image.put_pixel(i, j, Rgba([ele_t[0], ele_t[1], ele_t[2], 255]));
        }
    }

    for i in 0..200{
        for j in 201..400{
            image.put_pixel(i, j, Rgba([mag_r[0], mag_r[1], mag_r[2], 255]));
        }
    }

    for i in 201..400{
        for j in 201..400{
            image.put_pixel(i, j, Rgba([mag_t[0], mag_t[1], mag_t[2], 255]));
        }
    }
    image.save("output.png").unwrap();
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn print_matrices(){
        const nn: f32 = 1.46;
        const thetan: f32 = 45.0;
        const lambdan: f32 = 400.0;
        const dn: f32 = 0.002;

        println!("{:?}", super::d_e(&nn, &thetan));
        println!("{:?}", super::d_m(&nn, &thetan));
        println!("{:?}", super::k_func(&nn, &lambdan, &thetan));
        println!("{:?}", super::p_mat(&nn, &dn, &lambdan, &thetan));
        assert_eq!(4,4);
    }
}
