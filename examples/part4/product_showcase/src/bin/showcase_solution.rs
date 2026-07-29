use bevy::prelude::*;

fn pan_focus(focus: Vec3, camera: Transform, mouse_delta: Vec2, sensitivity: f32) -> Vec3 {
    focus + (-camera.right() * mouse_delta.x + camera.up() * mouse_delta.y) * sensitivity
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ProductInstance {
    body_mesh: u64,
    lens_mesh: u64,
}

fn instantiate_products(count: usize, body_mesh: u64, lens_mesh: u64) -> Vec<ProductInstance> {
    vec![
        ProductInstance {
            body_mesh,
            lens_mesh,
        };
        count
    ]
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct MaterialControls {
    metallic: f32,
    roughness: f32,
}

impl MaterialControls {
    fn edit(&mut self, metallic_delta: f32, roughness_delta: f32) {
        self.metallic = (self.metallic + metallic_delta).clamp(0.0, 1.0);
        self.roughness = (self.roughness + roughness_delta).clamp(0.045, 1.0);
    }
}

fn orbiting_light(elapsed_seconds: f32, radius: f32, height: f32, speed: f32) -> Vec3 {
    let angle = elapsed_seconds * speed;
    Vec3::new(angle.cos() * radius, height, angle.sin() * radius)
}

fn main() {
    let camera = Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y);
    let focus = pan_focus(Vec3::ZERO, camera, Vec2::new(20.0, -10.0), 0.01);
    let products = instantiate_products(3, 101, 202);
    let mut material = MaterialControls {
        metallic: 0.2,
        roughness: 0.5,
    };
    material.edit(0.1, -0.1);
    let light = orbiting_light(1.0, 4.0, 3.0, 0.75);
    println!(
        "focus={focus:.2}, shared_body={}, material={material:?}, light={light:.2}",
        products[0].body_mesh
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pan_moves_in_camera_plane() {
        let camera = Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
        let focus = pan_focus(Vec3::ZERO, camera, Vec2::new(10.0, 5.0), 0.1);
        assert!(focus.abs_diff_eq(Vec3::new(-1.0, 0.5, 0.0), 0.0001));
    }

    #[test]
    fn product_instances_share_mesh_ids() {
        let products = instantiate_products(4, 10, 20);
        assert!(products.iter().all(|product| product.body_mesh == 10));
        assert!(products.iter().all(|product| product.lens_mesh == 20));
    }

    #[test]
    fn material_editor_keeps_pbr_values_valid() {
        let mut material = MaterialControls {
            metallic: 0.9,
            roughness: 0.1,
        };
        material.edit(0.5, -0.5);
        assert_eq!(material.metallic, 1.0);
        assert_eq!(material.roughness, 0.045);
    }

    #[test]
    fn light_keeps_constant_orbit_radius() {
        for time in [0.0, 1.0, 8.0] {
            let position = orbiting_light(time, 5.0, 3.0, 1.2);
            assert!((position.xz().length() - 5.0).abs() < 0.0001);
            assert_eq!(position.y, 3.0);
        }
    }
}
