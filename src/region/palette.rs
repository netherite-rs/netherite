/// Represents a palette used to store blocks and biomes.
///
/// 0 is the default value.
pub trait Palette {
    fn get(&self, x: i32, y: i32, z: i32) -> i32;

    fn dimension(&self) -> i32;

    fn max_bits_per_entry(&self) -> Option<i32>;

    fn bits_per_entry(&self) -> Option<i32>;

    fn max_size(&self) -> i32 {
        let dimension = self.dimension();
        return dimension.pow(3);
    }
}

pub trait PaletteMut {

    fn set(&mut self, x: i32, y: i32, z: i32, value: i32);

    fn fill(&mut self, value: i32);

    fn replace(&mut self, x: i32, y: i32, z: i32, remap: impl FnOnce(i32) -> i32);

}