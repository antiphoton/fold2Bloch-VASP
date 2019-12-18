mod point;

use num_rational::Rational;
use point::Point;
fn generate_single_line(begin: Point, end: Point, n: usize) -> Vec<Point> {
    let n = n as isize;
    (0..n).into_iter().map(|i| {
        begin.lerp(end, Rational::new(i, n))
    }).collect()
}

fn generate_multiple_lines(a: Vec<(Point, usize)>) -> Vec<Point> {
    let mut result = vec![];
    let n = a.len();
    for i in 0..n {
        let mut single_line = generate_single_line(a[i].0, a[(i + 1) % n].0, a[i].1);
        result.append(&mut single_line);
    }
    result
}

fn print_points(a: &[Point]) -> () {
    println!("{}", a.len());
    for p in a {
        if false {
            println!("{}", p);
        } else {
            fn f(v: &Rational) {
                let p = v.numer();
                let q = v.denom();
                print!("{:.4}", *p as f64 / *q as f64);
            }
            print!("(");
            f(&p.x);
            print!(", ");
            f(&p.y);
            print!(", ");
            f(&p.z);
            print!(")");
            println!();
        }
    }
}

fn main() {
    let kpr = generate_multiple_lines(vec![
        (Point::new(Rational::new(0, 1), Rational::new(0, 1), Rational::new(0, 1)), 16),
        (Point::new(Rational::new(1, 2), Rational::new(1, 2), Rational::new(1, 2)), 9),
        (Point::new(Rational::new(1, 2), Rational::new(1, 2), Rational::new(0, 1)),  1),
    ]);
    let folds = (2, 2, 2);
    let ksc: Vec<_> = kpr.iter().map(|p| {
        let p = Point::new(p.x * folds.0, p.y * folds.1, p.z * folds.2);
        point::fold(p)
    }).collect();
    print_points(&ksc);
}

