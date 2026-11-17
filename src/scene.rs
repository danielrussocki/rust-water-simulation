//! State and water evolution for the reference scene, independent of GPUI.

#[derive(Clone, Copy, Debug)]
pub struct Drop {
    pub position: [f32; 2],
    pub radius: f32,
    pub strength: f32,
}

pub struct Scene {
    pub sphere_center: [f32; 3],
    pub sphere_radius: f32,
    pub sphere_velocity: [f32; 3],
    pub camera_angles: [f32; 2],
    pub light_direction: [f32; 3],
    pub paused: bool,
    pub sphere_physics: bool,
    pub initial_drops: Vec<Drop>,
    pub water: Water,
}

impl Scene {
    pub fn update(&mut self, seconds: f32) {
        if !self.paused {
            self.water.update(seconds);
        }
    }

    pub fn with_random(mut random: impl FnMut() -> f32) -> Self {
        let mut water = Water::new(256);
        let mut initial_drops = Vec::with_capacity(20);
        for i in 0..20 {
            let drop = Drop {
                position: [random() * 2.0 - 1.0, random() * 2.0 - 1.0],
                radius: 0.03,
                strength: if i % 2 == 0 { -0.01 } else { 0.01 },
            };
            water.add_drop(drop);
            initial_drops.push(drop);
        }
        Self {
            sphere_center: [-0.4, -0.75, 0.2],
            sphere_radius: 0.25,
            sphere_velocity: [0.0; 3],
            camera_angles: [-25.0, -200.5],
            light_direction: [2.0 / 3.0, 2.0 / 3.0, -1.0 / 3.0],
            paused: false,
            sphere_physics: false,
            initial_drops,
            water,
        }
    }
}

pub struct Water {
    size: usize,
    cells: Vec<(f32, f32)>,
    next: Vec<(f32, f32)>,
}

impl Water {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "el tamaño del agua debe ser mayor que cero");
        Self {
            size,
            cells: vec![(0.0, 0.0); size * size],
            next: vec![(0.0, 0.0); size * size],
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> (f32, f32) {
        self.cells[y * self.size + x]
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set_cell(&mut self, x: usize, y: usize, height: f32, velocity: f32) {
        self.cells[y * self.size + x] = (height, velocity);
    }

    fn add_drop(&mut self, drop: Drop) {
        for y in 0..self.size {
            for x in 0..self.size {
                let coord = [
                    (x as f32 + 0.5) / self.size as f32,
                    (y as f32 + 0.5) / self.size as f32,
                ];
                let dx = drop.position[0] * 0.5 + 0.5 - coord[0];
                let dy = drop.position[1] * 0.5 + 0.5 - coord[1];
                let amount = (1.0 - (dx * dx + dy * dy).sqrt() / drop.radius).max(0.0);
                self.cells[y * self.size + x].0 +=
                    (0.5 - (amount * std::f32::consts::PI).cos() * 0.5) * drop.strength;
            }
        }
    }

    pub fn update(&mut self, seconds: f32) {
        if seconds > 1.0 {
            return;
        }
        for _ in 0..2 {
            for y in 0..self.size {
                for x in 0..self.size {
                    let (height, velocity) = self.cell(x, y);
                    let average = (self.cell(x.saturating_sub(1), y).0
                        + self.cell(x, y.saturating_sub(1)).0
                        + self.cell((x + 1).min(self.size - 1), y).0
                        + self.cell(x, (y + 1).min(self.size - 1)).0)
                        * 0.25;
                    let velocity = (velocity + (average - height) * 2.0) * 0.995;
                    self.next[y * self.size + x] = (height + velocity, velocity);
                }
            }
            std::mem::swap(&mut self.cells, &mut self.next);
        }
    }
}
