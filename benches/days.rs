use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_days(c: &mut Criterion) {
    for day in advent_of_code_2024::days::days() {
        let day_name = format!("day{:02}", day);
        c.bench_function(&day_name, |b| {
            let input_path = format!("inputs/day{:02}.txt", day);
            let input = std::fs::read_to_string(input_path).unwrap();
            let day_solve = advent_of_code_2024::days::solver(day).unwrap();

            b.iter(|| black_box(day_solve(&input)));
        });
    }
}

criterion_group!(benches, bench_days);
criterion_main!(benches);
