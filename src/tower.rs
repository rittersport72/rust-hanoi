use graphics::types::Color;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Tower {
    disk_storage: Vec<Disk>, // Contains disks
}

impl Tower {
    pub fn insert(&mut self, disk: Disk) {
        self.disk_storage.push(disk);
    }

    pub fn remove(&mut self) -> Disk {
        self.disk_storage.pop().unwrap()
    }

    pub fn get(&self) -> Vec<Disk> {
        self.disk_storage.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Disk {
    number: u32,
    color: Color,
}

impl Disk {
    pub fn new(number: u32) -> Self {
        Self {
            number,
            color: Disk::create_color(number),
        }
    }

    pub fn get_number(&self) -> u32 {
        self.number
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    fn create_color(number: u32) -> Color {
        [
            0.2,
            0.2 + 0.1 * number as f32,
            0.6,
            1.0, // translucent
        ]
    }
}
