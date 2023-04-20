use glam::{Vec3, Vec4};

pub struct Ray {
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
}

#[derive(Copy, Clone)]
pub struct Intersection {
    position: Vec3,
    normal: Vec3,
    distance: f32,
}

pub trait Object: Sync {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Sphere {
    pub(crate) center: Vec3,
    pub(crate) radius: f32,
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            let position = ray.origin + t * ray.direction;
            let normal = (position - self.center).normalize();
            Some(Intersection {
                position,
                normal,
                distance: t,
            })
        }
    }
}

pub struct Scene {
    pub(crate) objects: Vec<Box<dyn Object>>,
}

impl Scene {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<(Intersection, &Box<dyn Object>)> {
        let mut closest_intersection: Option<(Intersection, &Box<dyn Object>)> = None;

        for object in &self.objects {
            if let Some(intersection) = object.intersect(ray) {
                if let Some((closest_intersection_distance, _)) = closest_intersection {
                    if intersection.distance < closest_intersection_distance.distance {
                        closest_intersection = Some((intersection, object));
                    }
                } else {
                    closest_intersection = Some((intersection, object));
                }
            }
        }

        closest_intersection
    }

    pub(crate) fn trace(&self, ray: &Ray) -> Vec4 {
        if let Some((intersection, object)) = self.intersect(ray) {
            let light_direction = (Vec3::new(-1.0, 2.0, -1.0)).normalize();
            let diffuse_factor = intersection.normal.dot(light_direction).max(0.0);
            let color = Vec4::new(1.0, 0.5, 0.1, 1.0);
            color * diffuse_factor
        } else {
            Vec4::new(0.0, 0.0, 0.0, 1.0)
        }
    }
}
