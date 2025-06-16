use primal::Primes;

// The inequality $p_{n+1}-p_n < (\log p_n)^2-\log p_n$ holds for $n > 4$.
// A consequence of Firuzbakht's conjecture.
pub fn check_consequence(max_n: usize) {

    let primes: Vec<usize> = Primes::all()
        .take_while(|&p| p <= max_n)
        .collect();

    if primes.len() < 2 {
        panic!("At least 2 prime numbers are required for analysis");
    }

    let mut violations = Vec::new();

    for i in 1..primes.len() {
        if i % 10000 == 0 {
            println!("Processed {} out of {}", i, primes.len() - 1);
        }

        let gap = (primes[i] - primes[i-1]) as f64;
        let log_p = (primes[i-1] as f64).ln();
        let rhs = log_p.powi(2) - log_p;

        if gap >= rhs {
            violations.push((i, primes[i-1], gap, rhs));
        }
    }

    println!("Violations: {}", violations.len());
    for v in &violations {
        println!(
            "n: {}, p_n: {}, gap: {}, rhs: {:.6}",
            v.0, v.1, v.2, v.3
        );
    }
}