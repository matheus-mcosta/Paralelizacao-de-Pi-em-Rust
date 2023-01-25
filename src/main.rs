use rayon::prelude::*;
use std::time::Instant;

const NUM_CASAS: u64 = 1_000_000_000;

// Funcao utilizando biblioteca de paralelizacao Rayon
// Utiliza um iterador de 0 ate o valor de input da funcao(casas)
// retorna o valor de cada iteracao e soma ao final
fn pi_rayon(casas: u64) -> f64 {
    (0..casas)
        .into_par_iter()
        .map(|i| {
            let mut pi = 0.0;
            pi += 4.0 / (4.0 * i as f64 + 1.0);
            pi -= 4.0 / (4.0 * i as f64 + 3.0);
            return pi
        })
        .sum::<f64>()
}

// Mesma funcao anterior, porem sequencial. Observar linha 9 e 22.
// into_par_iter X into_iter
fn pi_sequencial(casas: u64) -> f64 {
    (0..casas)
        .into_iter()
        .map(|i| {
            let mut pi = 0.0;
            pi += 4.0 / (4.0 * i as f64 + 1.0);
            pi -= 4.0 / (4.0 * i as f64 + 3.0);
            return pi
        })
        .sum::<f64>()
}

// Funcao utilizando laco for, sintaxe parecida com C
fn pi_for(casas: u64) -> f64 {
    let mut pi: f64 = 0.0;
    for j in 0..casas {
        pi += 4.0 / (4.0 * j as f64 + 1.0);
        pi -= 4.0 / (4.0 * j as f64 + 3.0);
    }
    return pi
}

fn main() {
    // Rayon paralelizado
    let now = Instant::now();
    let pi = pi_rayon(NUM_CASAS);
    let elapsed_time = now.elapsed();
    println!("Running pi took {} micros.", elapsed_time.as_micros());
    println!("Result Multithread: {}", pi);

    // Sequencial
    let now2 = Instant::now();
    let pi2 = pi_sequencial(NUM_CASAS);
    let elapsed_time2 = now2.elapsed();
    println!("Running pi seq took {} micros.", elapsed_time2.as_micros());
    println!("Result sequential: {}", pi2);

    // Utilizando for loop
    let now3 = Instant::now();
    let pi3 = pi_for(NUM_CASAS);
    let elapsed_time3 = now3.elapsed();
    println!("Running pi seq took {} micros.", elapsed_time3.as_micros());
    println!("Result sequential: {}", pi3);
}
