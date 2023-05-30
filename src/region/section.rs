use std::path::Path;

const ZERO: ChunkPos = ChunkPos { x: 0, z: 0 };

pub struct ChunkPos {
    x: i32,
    z: i32,
}

impl ChunkPos {
    fn get_from_file(file: &Path) -> Option<ChunkPos> {
        let name = file.file_name().unwrap().to_str().unwrap();
        if !name.starts_with("r.") || !name.ends_with(".mca") {
            return None;
        }
        let split = name.split('.').collect::<Vec<&str>>();
        if split.len() != 4 {
            return None;
        }
        let x = split[0].parse::<i32>().unwrap();
        let z = split[1].parse::<i32>().unwrap();
        Some(ChunkPos { x: x << 5, z: z << 5 })
    }

    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn region_local_x(&self) -> i32 {
        self.x & 31
    }

    pub fn region_local_z(&self) -> i32 {
        self.z & 31
    }

    pub fn region_x(&self) -> i32 {
        self.x >> 5
    }

    pub fn region_z(&self) -> i32 {
        self.z >> 5
    }

    pub fn offset_index(&self) -> i32 {
        self.region_local_x() + self.region_local_z() * 32
    }
}