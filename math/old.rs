#![allow(dead_code)]
use std::ops::{Add, Sub, Mul};
use std::f64::consts::PI;
use std::fmt;
use plotters::prelude::*;

#[derive(Debug, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, scalar: T) -> Point<T> {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Point<T> {
    fn dot(&self, other: &Point<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy + Into<f64>> Point<T> {
    fn norm(&self) -> f64 {
        let sum: f64 = (self.x * self.x + self.y * self.y).into();
        sum.sqrt()
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy + Into<f64>> Point<T> {
    fn normalize(&self) -> Point<f64> {
        let length = self.norm();
        Point {
            x: self.x.into() / length,
            y: self.y.into() / length,
        }
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Point<T> {
    fn cross(&self, other: &Point<T>) -> T {
        self.x * other.y - self.y * other.x
    }
}

impl Point<f64> {
    fn rotate(&self, angle: f64) -> Point<f64> {
        let rad = angle * PI / 180.0;
        Point {
            x: self.x * rad.cos() - self.y * rad.sin(),
            y: self.x * rad.sin() + self.y * rad.cos(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let point = Point { x: 0.0, y: 0.0 };

    let root = BitMapBackend::new("vector_trajectory.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Trajectoire du vecteur", ("sans-serif", 30))
        .build_cartesian_2d(-15..15, -15..15)?;

    chart.configure_mesh().draw()?;

    let red_style = ShapeStyle {
        color: RED.to_rgba(),
        filled: true,
        stroke_width: 0,
    };

    // Tracer le point (0, 0)
    chart.draw_series(std::iter::once(Circle::new(
        (0, 0),
        5,
        red_style,
    )))?;

    let vectors = vec![
        Point { x: -5.0, y: 6.0 },
        Point { x: 7.0, y: -3.0 },
        Point { x: 4.0, y: 9.0 },
    ];

    for vector in vectors {
        chart.draw_series(LineSeries::new(
                vec![
                (point.x as i32, point.y as i32),
                (vector.x as i32, vector.y as i32),
                ],
                &BLUE,
        ))?;
    }

    // Sauvegarder le graphique
    root.present()?;

    Ok(())
}

