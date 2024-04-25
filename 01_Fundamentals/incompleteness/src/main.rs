use peroxide::fuga::*;
use std::f64::consts::PI;

#[allow(non_snake_case)]
fn main() {
    let N_vec = [1, 5, 10, 100, 1000];
    N_vec.into_iter()
        .map(|N| (N, poisson_summation(N)))
        .for_each(|(N, (mu, poisson))| {
            let poisson = poisson.fmap(|x| x / (2f64 * N as f64 + 1f64));
            let legend_str = format!("N = {}", N);
            let mut plt = Plot2D::new();
            plt
                .set_domain(mu)
                .insert_image(poisson)
                .set_xlabel(r"$\mu$")
                .set_ylabel(r"$\sin \pi \mu (2N+1) / \sin \pi \mu$")
                .set_title(&legend_str)
                .set_style(PlotStyle::Nature)
                .tight_layout()
                .set_fig_size((5, 3))
                .set_dpi(600)
                .set_path(&format!("figs/poisson_{:04}.png", N))
                .savefig().unwrap();
        })
}

#[allow(non_snake_case)]
fn poisson_summation(N: usize) -> (Vec<f64>, Vec<f64>) {
    let N = N as f64;
    let mu = linspace(-3, 3, 100000);
    mu.into_iter()
        .map(|mu| {
            (mu, f(mu, N))
        })
        .unzip()
}

#[allow(non_snake_case)]
fn f(mu: f64, N: f64) -> f64 {
    if mu == mu.round() {
        2f64 * N + 1f64
    } else {
        let pi_mu = PI * mu;
        (pi_mu * (2f64 * N + 1f64)).sin() / pi_mu.sin()
    }
}
