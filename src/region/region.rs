use std::fs::File;
use std::io::{Result, Seek, SeekFrom};
use std::os::windows::fs::FileExt;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use bytes::Buf;
use nbt::Blob;

use coordinates::{sector_offset, size_in_sectors};

use crate::region::coordinates;
use crate::region::coordinates::{chunk_inside_region, get_index, to_region};

const GZIP_COMPRESSION: u8 = 1;
const ZLIB_COMPRESSION: u8 = 2;
const NO_COMPRESSION: u8 = 3;
const MAX_ENTRY_COUNT: u64 = 1024;
const SECTOR_SIZE: usize = 4096;
const SECTOR_1_MB: u64 = 256;
const HEADER_LENGTH: u64 = MAX_ENTRY_COUNT * 2 * 4;

pub struct Region {
    region_file: File,
    region_x: i32,
    region_z: i32,
    locations: [i32; 1024],
    timestamps: [i32; 1024],
    free_sectors: Vec<bool>,
}

impl Region {
    pub fn new(mut region_file: File, region_x: i32, region_z: i32) -> Result<Self> {
        let mut locations: [i32; 1024] = [0; 1024];
        let mut timestamps: [i32; 1024] = [0; 1024];

        region_file.seek(SeekFrom::Start(0))?;
        let length = region_file.metadata().unwrap().len();
        if length < HEADER_LENGTH as u64 {
            for _ in 0..HEADER_LENGTH {
                region_file.write_u8(0)?;
            }
        }

        Self::add_padding(&mut region_file)?;

        let length = region_file.metadata().unwrap().len();

        let available_sectors = length / (SECTOR_SIZE as u64);
        let mut free_sectors: Vec<bool> = vec![true; available_sectors as usize];

        free_sectors[0] = false;
        free_sectors[1] = false;

        region_file.seek(SeekFrom::Start(0))?;

        // Read chunk locations
        for i in 0..(MAX_ENTRY_COUNT as usize) {
            let location = region_file.read_i32::<BigEndian>()?;
            locations[i] = location;
            // mark already allocated sectors as taken.
            // location 0 means the chunk is *not* stored in the file
            if location != 0 && sector_offset(location) + size_in_sectors(location) <= free_sectors.len() {
                for sector_index in 0..size_in_sectors(location) {
                    free_sectors[sector_index + sector_offset(location)] = false
                }
            }
        }
        // read chunk timestamps
        for i in 0..(MAX_ENTRY_COUNT as usize) {
            timestamps[i] = region_file.read_i32::<BigEndian>()?;
        }
        Ok(Self {
            region_file,
            region_x,
            region_z,
            locations,
            timestamps,
            free_sectors,
        })
    }

    pub fn get_chunk_data(&self, chunk_x: i32, chunk_z: i32) -> Result<Option<Blob>> {
        // if self.is_out(chunk_x, chunk_z) {
        //     panic!("Out of region. X: {}. Z: {} (This region -> X: {}, Z: {})", chunk_x, chunk_z, self.region_x, self.region_z);
        // }
        if !self.has_chunk(chunk_x, chunk_z) {
            return Ok(None);
        }
        return self.read_column_data(chunk_inside_region(chunk_x), chunk_inside_region(chunk_z))
            .map(|v| Some(v));
    }

    fn has_chunk(&self, x: i32, z: i32) -> bool {
        self.locations[get_index(chunk_inside_region(x), chunk_inside_region(z))] != 0
    }

    fn read_column_data(&self, x: i32, z: i32) -> Result<Blob> {
        let offset = self.file_offset(x, z);
        let length = {
            let mut v = vec![0_u8; 4];
            self.region_file.seek_read(&mut v, offset as u64)?;
            ((v[0] as u32) << 24) + ((v[1] as u32) << 16) + ((v[2] as u32) << 8) + ((v[3] as u32) << 0)
        } as usize;
        let raw_data = {
            let mut v = vec![0; length - 1];
            self.region_file.seek_read(&mut v, (offset + 5) as u64)?;
            v
        };
        let compressed_type = {
            let mut v = [0; 1];
            self.region_file.seek_read(&mut v, (offset + 4) as u64)?;
            v[0]
        };
        let mut reader = raw_data.reader();
        let data = match compressed_type {
            GZIP_COMPRESSION => Blob::from_gzip_reader(&mut reader),
            ZLIB_COMPRESSION => Blob::from_zlib_reader(&mut reader),
            NO_COMPRESSION => Blob::from_reader(&mut reader),
            compression => panic!("invalid compression: {}", compression)
        }?;
        Ok(data)
    }

    fn file_offset(&self, chunk_x: i32, chunk_z: i32) -> usize {
        sector_offset(self.locations[get_index(chunk_x, chunk_z)]) * SECTOR_SIZE
    }

    fn is_out(&self, chunk_x: i32, chunk_z: i32) -> bool {
        println!("{}", to_region(chunk_x));
        println!("{}", to_region(chunk_z));
        to_region(chunk_x) != self.region_x || to_region(chunk_z) != self.region_z
    }

    fn add_padding(file: &mut File) -> Result<()> {
        let length = file.metadata().unwrap().len();
        let missing_padding = length % (SECTOR_SIZE as u64);
        // file is not a multiple of 4kib, add padding
        if missing_padding > 0 {
            return file.set_len(length + (SECTOR_SIZE as u64 - missing_padding));
        }
        return Ok(());
    }
}