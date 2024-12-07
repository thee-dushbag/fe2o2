use num::complex::Complex;

struct Pair<T> {
  x: T,
  y: T,
}

fn canculate_mandelbrot(
  max_iters: usize,
  min: Pair<f64>,
  max: Pair<f64>,
  dim: Pair<usize>,
) -> Vec<Vec<usize>> {
  let mut rows: Vec<_> = Vec::with_capacity(dim.x);
  for y in 0..dim.y {
    let mut row: Vec<_> = Vec::with_capacity(dim.y);
    for x in 0..dim.x {
      let xp = x as f64 / dim.x as f64;
      let yp = y as f64 / dim.y as f64;
      let cx = min.x + (max.x - min.x) * xp;
      let cy = min.y + (max.y - min.y) * yp;
      row.push(mandelbrot_at_point(Complex { re: cx, im: cy }, max_iters));
    }
    rows.push(row);
  }
  rows
}

fn mandelbrot_at_point(c: Complex<f64>, max_iters: usize) -> usize {
  let mut z = Complex { re: 0.0, im: 0.0 };
  for i in 0..max_iters {
    if z.norm() >= 3.0 {
      return i;
    }
    z = z * z + c;
  }
  max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
  for row in escape_vals {
    let mut line = String::with_capacity(row.len());
    for column in row {
      let val = match column {
        0..3 => ' ',
        3..6 => '.',
        6..11 => '•',
        11..31 => '*',
        31..101 => '+',
        101..201 => 'x',
        201..401 => '$',
        401..701 => '#',
        _ => '%',
      };
      line.push(val);
    }
    println!("{}", line);
  }
}

fn main() {
  let mandelbrot = canculate_mandelbrot(
    1000,
    Pair { x: -2.0, y: -1.5 },
    Pair { x: 2.0, y: 1.5 },
    Pair { x: 125, y: 30 },
  );
  render_mandelbrot(mandelbrot);
}
