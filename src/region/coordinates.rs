use num_traits::PrimInt;

pub fn size_in_sectors(location: i32) -> usize {
    (location & 0xFF) as usize
}

pub fn sector_offset(location: i32) -> usize {
    location.unsigned_shr(8) as usize
}

pub fn get_index(chunk_x: i32, chunk_z: i32) -> usize {
    ((chunk_inside_region(chunk_x) & 31) + (chunk_inside_region(chunk_z) & 31) * 32) as usize
}

pub fn to_region(coordinate: i32) -> i32 {
    (coordinate as f32 / 32.0).floor() as i32
}

pub fn to_chunk(region: i32) -> i32 {
    region * 32
}

pub fn chunk_inside_region(chunk: i32) -> i32 {
    chunk & 31
}