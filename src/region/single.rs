use crate::region::palette::Palette;

pub struct SingleValuePalette {
    dimension: u8,
    value: i32,
}

impl Palette for SingleValuePalette {
    fn get(&self, _x: i32, _y: i32, _z: i32) -> i32 {
        self.value
    }

    fn dimension(&self) -> i32 {
        self.dimension as i32
    }

    fn max_bits_per_entry(&self) -> Option<i32> {
        None
    }

    fn bits_per_entry(&self) -> Option<i32> {
        None
    }
}

