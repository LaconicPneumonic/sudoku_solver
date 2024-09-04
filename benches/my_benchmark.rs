use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sudoku_solver::{SudokuBoard, BACKTRACKING_SOLVER};

fn criterion_benchmark(c: &mut Criterion) {
    let board = SudokuBoard::from_condensed(
        "..3..2.8.14......9.68.593.7..24.5...............2.85..9.457.86.6......75.8.6..4..",
    );

    c.bench_function("sudoku solver", |b| {
        b.iter(|| {
            let _ = black_box(BACKTRACKING_SOLVER(&board));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
