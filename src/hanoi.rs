use crate::tower::{Disk, Tower};
use graphics::{clear, rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};

// Window dimensions
pub const WIDTH: f64 = 400.0 * 3.0;
pub const HEIGHT: f64 = TOWER_Y_POSITION * NUMBER_OF_DISKS as f64;

const BACKGROUND_COLOR: [f32; 4] = [0.1, 0.1, 0.1, 1.0]; // black

// Tower
pub const NUMBER_OF_DISKS: u32 = 3;
const DISK_MINIMUM_WIDTH: f64 = 50.0;
const DISK_HEIGHT: f64 = 20.0;
const TOWER_X_POSITION: f64 = 200.0;
const TOWER_Y_POSITION: f64 = 100.0;

pub struct Application {
    gl: GlGraphics,
    timer: f64,
    towec: Vec<Tower>,         // Contains the three towers
    solution: Vec<(u32, u32)>, // Solution moves disk from pole to pole
}

impl Application {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        Application {
            gl: GlGraphics::new(opengl),
            timer: 0.0,
            towec: vec![Tower::default(), Tower::default(), Tower::default()],
            solution: Vec::new(),
        }
    }

    pub fn create_disks(&mut self) {
        // Put disks in first tower
        let tower = &mut self.towec[0];

        for n in 0..NUMBER_OF_DISKS {
            let disk = Disk::new(n + 1);
            tower.insert(disk);
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND_COLOR, gl);

            let tower = &mut self.towec[0];
            let disks = tower.get();

            for disk in disks {
                let n = disk.get_number();

                // Draw disk
                let rect = [
                    TOWER_X_POSITION / 2.0,
                    TOWER_Y_POSITION + DISK_HEIGHT * n as f64,
                    DISK_MINIMUM_WIDTH * n as f64,
                    DISK_HEIGHT,
                ];
                rectangle(disk.get_color(), rect, c.transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.timer = self.timer + args.dt;
        if self.timer >= 1.0 {
            //println!("timer hit {}", self.timer);
            self.timer = 0.0;
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
