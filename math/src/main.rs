#![allow(dead_code)]
use std::ops::{Add, Sub, Mul};
use std::f64::consts::PI;
use std::fmt;

#[derive(Debug, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Plane {
    position: Point<f64>,
    velocity: f64,
    direction: f64, // En radians
}

#[derive(Debug)]
struct Station {
    position: Point<f64>,
    radius: f64, // Rayon du cercle de signal
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

impl<T: Sub<Output = T> + Mul<Output = T> + Copy + Into<f64>> Point<T> {
    fn distance(&self, other: &Point<T>) -> f64 {
        let dx = (self.x - other.x).into();
        let dy = (self.y - other.y).into();
        (dx * dx + dy * dy).sqrt()
    }
}

impl Point<f64> {
    fn angle_with(&self, other: &Point<f64>) -> f64 {
        let dot_product = self.dot(other);
        let norms = self.norm() * other.norm();
        (dot_product / norms).acos().to_degrees()
    }

    fn project_on(&self, other: &Point<f64>) -> Point<f64> {
        let scalar = self.dot(other) / other.dot(other);
        other.clone() * scalar
    }
    
    fn transform(&self, matrix: [[f64; 2]; 2]) -> Point<f64> {
        Point {
            x: matrix[0][0] * self.x + matrix[0][1] * self.y,
            y: matrix[1][0] * self.x + matrix[1][1] * self.y,
        }
    }
    
    fn is_intersecting(p1: &Point<f64>, p2: &Point<f64>, q1: &Point<f64>, q2: &Point<f64>) -> bool {
        let cross1 = (q1.x - p1.x) * (p2.y - p1.y) - (q1.y - p1.y) * (p2.x - p1.x);
        let cross2 = (q2.x - p1.x) * (p2.y - p1.y) - (q2.y - p1.y) * (p2.x - p1.x);
        let cross3 = (p1.x - q1.x) * (q2.y - q1.y) - (p1.y - q1.y) * (q2.x - q1.x);
        let cross4 = (p2.x - q1.x) * (q2.y - q1.y) - (p2.y - q1.y) * (q2.x - q1.x);

        (cross1 * cross2 < 0.0) && (cross3 * cross4 < 0.0)
    }

    fn is_inside_polygon(point: &Point<f64>, polygon: &[Point<f64>]) -> bool {
        let mut count = 0;
        let mut j = polygon.len() - 1;
        for i in 0..polygon.len() {
            if (polygon[i].y > point.y) != (polygon[j].y > point.y) &&
               (point.x < (polygon[j].x - polygon[i].x) * (point.y - polygon[i].y) /
                          (polygon[j].y - polygon[i].y) + polygon[i].x) {
                count += 1;
            }
            j = i;
        }
        count % 2 == 1
    }
    
    fn is_collinear(p1: &Point<f64>, p2: &Point<f64>, p3: &Point<f64>) -> bool {
        let determinant = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
        determinant.abs() < 1e-10
    }
}

/*
(Selon un exo brilliant)
rac((x - a)² + (y - b)²) = distance entre deux vecteurs quand dans la même direction
 */
/*fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _planet = Point { x: 0.0, y: 0.0 };

    let spaceship = Point { x: -5.0, y: 6.0 };
    let asteroid = Point { x: -3.0, y: 4.5 };

    let d = spaceship.distance(&asteroid);
    println!("Distance entre le vaisseau et l'astéroïde : {:.2}", d);

    let alert_threshold = 3.0;
    if d < alert_threshold {
        println!("🚨 Alerte ! L'astéroïde est trop proche du vaisseau !");
    } else {
        println!("✅ Pas de danger immédiat.");
    }

    Ok(())
}*/

impl Plane {
    fn move_forward(&mut self) {
        self.position.x += self.velocity * self.direction.cos();
        self.position.y += self.velocity * self.direction.sin();
    }

    fn is_near_station(&self, station: &Station) -> bool {
        let dist = ((self.position.x - station.position.x).powi(2) + 
                    (self.position.y - station.position.y).powi(2)).sqrt();
        dist < station.radius
    }

    fn is_colliding(&self, other: &Plane, collision_radius: f64) -> bool {
        let dist = ((self.position.x - other.position.x).powi(2) +
                    (self.position.y - other.position.y).powi(2)).sqrt();
        dist < collision_radius
    }
}

fn main() {
    let mut plane1 = Plane { position: Point { x: 0.0, y: 0.0 }, velocity: 1.0, direction: PI / 4.0 };
    let mut plane2 = Plane { position: Point { x: 5.0, y: 5.0 }, velocity: 1.2, direction: -PI / 4.0 };
    let station = Station { position: Point { x: 3.0, y: 3.0 }, radius: 2.0 };

    let collision_radius = 0.5;

    for _ in 0..10 {
        plane1.move_forward();
        plane2.move_forward();

        if plane1.is_near_station(&station) {
            println!("🚀 Plane 1 envoie un signal à la station !");
        }
        if plane2.is_near_station(&station) {
            println!("🚀 Plane 2 envoie un signal à la station !");
        }

        if plane1.is_colliding(&plane2, collision_radius) {
            println!("💥 Crash entre Plane 1 et Plane 2 !");
            break;
        }

        println!("Plane 1 position: ({:.2}, {:.2})", plane1.position.x, plane1.position.y);
        println!("Plane 2 position: ({:.2}, {:.2})", plane2.position.x, plane2.position.y);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let v1 = Point { x: 3.0, y: 4.0 };
        let matrix = [[2.0, 0.0], [0.0, 2.0]];
        let transformed = v1.transform(matrix);
        assert_eq!(transformed.x, 6.0);
        assert_eq!(transformed.y, 8.0);
    }

    #[test]
    fn test_is_intersecting() {
        let a1 = Point { x: 1.0, y: 1.0 };
        let a2 = Point { x: 4.0, y: 4.0 };
        let b1 = Point { x: 1.0, y: 4.0 };
        let b2 = Point { x: 4.0, y: 1.0 };
        assert!(Point::is_intersecting(&a1, &a2, &b1, &b2));
    }

    #[test]
    fn test_is_inside_polygon() {
        let polygon = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 5.0, y: 0.0 },
            Point { x: 5.0, y: 5.0 },
            Point { x: 0.0, y: 5.0 },
        ];
        let inside = Point { x: 3.0, y: 3.0 };
        let outside = Point { x: 6.0, y: 3.0 };
        assert!(Point::is_inside_polygon(&inside, &polygon));
        assert!(!Point::is_inside_polygon(&outside, &polygon));
    }

    #[test]
    fn test_is_collinear() {
        let v1 = Point { x: 3.0, y: 4.0 };
        let v2 = Point { x: 5.0, y: 12.0 };
        assert!(Point::is_collinear(&v1, &v2, &Point { x: 7.0, y: 20.0 }));
        assert!(!Point::is_collinear(&v1, &v2, &Point { x: 7.0, y: 19.0 }));
    }
}
