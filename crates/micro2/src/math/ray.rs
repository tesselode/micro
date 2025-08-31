use glam::Vec2;

use super::Circle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
	/// The point of origin of the ray.
	pub origin: Vec2,
	/// A unit vector representing the direction of the ray.
	pub direction: Vec2,
}

impl Ray {
	// https://www.bluebill.net/circle_ray_intersection.html
	pub fn circle_intersection_points(self, circle: Circle) -> Vec<Vec2> {
		let u = circle.center - self.origin;
		let u1 = u.dot(self.direction) * self.direction;
		let u2 = u - u1;
		let d = u2.length();
		let m = (circle.radius.powi(2) - d.powi(2)).sqrt();
		if circle.contains_point(self.origin) {
			vec![self.origin + u1 + m * self.direction]
		} else if d < circle.radius {
			vec![
				self.origin + u1 + m * self.direction,
				self.origin + u1 - m * self.direction,
			]
		} else if d == circle.radius {
			vec![self.origin + u1 + m * self.direction]
		} else {
			vec![]
		}
	}
}
