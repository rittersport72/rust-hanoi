use std::fmt::Debug;
use crate::tower::{Disk, Tower};
use graphics::{clear, rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};

// Window dimensions
pub const WIDTH: f64 = TOWER_X_POSITION * 5.0 + TOWER_X_POSITION / 2.0;
pub const HEIGHT: f64 = NUMBER_OF_DISKS as f64 * DISK_HEIGHT + 110.0;

const BACKGROUND_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0]; // black

// Tower
pub const NUMBER_OF_DISKS: u32 = 5;
const DISK_MINIMUM_WIDTH: f64 = 50.0;
const DISK_HEIGHT: f64 = 20.0;
const TOWER_X_POSITION: f64 = 180.0;
const TOWER_Y_POSITION: f64 = 150.0;

pub struct Application {
    gl: GlGraphics,
    timer: f64,
    towec: Vec<Tower>,         // Contains the three towers
    solution: Vec<(u32, u32)>, // Solution moves disk from pole to pole
    solution_index: usize,
}

impl Application {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        Application {
            gl: GlGraphics::new(opengl),
            timer: 0.0,
            towec: vec![Tower::default(), Tower::default(), Tower::default()],
            solution: Vec::new(),
            solution_index: 0,
        }
    }

    pub fn create_disks(&mut self) {
        // Put disks in first tower
        let tower = &mut self.towec[0];

        // Loop from largest disk to smallest disk
        for n in (0..NUMBER_OF_DISKS).rev() {
            let disk = Disk::new(n + 1);
            tower.insert(disk);
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND_COLOR, gl);

            // Loop over all towers
            let mut tower_count = 0u32;
            for tower in &mut self.towec {
                tower_count += 1;
                let disks = tower.get();

                // Loop over all disks in tower from the top
                for (index, disk) in disks.iter().enumerate() {
                    //println!("Element at position {}: {:?}", index, disk);
                    let n = disk.get_number();

                    // Draw disk
                    let rect = [
                        (TOWER_X_POSITION * 1.4 * tower_count as f64)  - (DISK_MINIMUM_WIDTH * n as f64 / 2.0),
                        TOWER_Y_POSITION - (DISK_HEIGHT * index as f64),
                        DISK_MINIMUM_WIDTH * n as f64,
                        DISK_HEIGHT - 1.0,
                    ];
                    rectangle(disk.get_color(), rect, c.transform, gl);
                }
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.timer = self.timer + args.dt;
        if self.timer >= 2.0 {
            //println!("timer hit {}", self.timer);
            self.timer = 0.0;

            let moveto = self.solution.get(self.solution_index);
            if moveto.is_some() {
                self.solution_index += 1;
                let fromto= moveto.unwrap();
                println!("disk from {} to {}", fromto.0, fromto.1);

                let from_tower= &mut self.towec[fromto.0 as usize];
                let disk = from_tower.remove();
                let to_tower= &mut self.towec[fromto.1 as usize];
                to_tower.insert(disk);
            }
        }
    }

    pub fn solve(&mut self, n: u32, from: u32, to: u32, via: u32) {
        if n > 0 {
            self.solve(n - 1, from, via, to);
            println!("Move disk from pole {} to pole {}", from, to);
            self.solution.push((from, to));
            self.solve(n - 1, via, to, from);
        }
    }
}
