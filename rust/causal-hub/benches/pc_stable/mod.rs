pub mod discrete {
    use causal_hub::prelude::*;
    use criterion::{criterion_group, Criterion};
    use polars::prelude::*;

    // Set ChiSquared significance level
    const ALPHA: f64 = 0.05;

    // PC-Stable `asia` benchmark (just for debug purposes)
    fn asia(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/asia.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("asia", |b| b.iter(|| pcs.call().meek_procedure_until_3()));
    }

    // PC-Stable `andes` benchmark
    fn andes(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/andes.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("andes", |b| b.iter(|| pcs.call().meek_procedure_until_3()));
    }

    // PC-Stable parallel `andes` benchmark
    fn par_andes(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/andes.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("par_andes", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    // PC-Stable `pigs` benchmark
    fn pigs(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/pigs.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("pigs", |b| b.iter(|| pcs.call().meek_procedure_until_3()));
    }

    // PC-Stable parallel `pigs` benchmark
    fn par_pigs(c: &mut Criterion) {
        // Load data set.
        let d = CsvReader::from_path("./tests/assets/PC-Stable/pigs.csv")
            .unwrap()
            .finish()
            .unwrap();
        let d = DiscreteDataMatrix::from(d);

        // Create ChiSquared conditional independence test
        let test = ChiSquared::new(&d).with_significance_level(ALPHA);

        // Create PC-Stable functor
        let pcs = PCStable::new(&test);

        // Benchmark
        c.bench_function("par_pigs", |b| {
            b.iter(|| pcs.parallel_call().meek_procedure_until_3())
        });
    }

    criterion_group!(discrete, asia, andes, par_andes, pigs, par_pigs);
}
