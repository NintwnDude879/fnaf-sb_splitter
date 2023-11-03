use asr::{Address, Process};
use bytemuck::Pod;

use crate::Version;

pub struct Path<T, const N: usize> {
    path: [u64; N],
    ty: core::marker::PhantomData<T>,
}

pub struct Paths {
    pub closest_interactible_address: Path<u8, 6>,
}

impl Paths {
    pub fn new(version: Version) -> Self {
        let game_engine_offset; // TODO: Sig scan for this for resilency to game updates.

        match version {
            Version::V1_0_2 => {
                game_engine_offset = 0x0575_8730;
            }
            Version::V1_0_3 => {
                game_engine_offset = 0x0576_1E70;
            }
        };

        Self {
            closest_interactible_address: Path::new([
                game_engine_offset,
                _OFFSET,
                SUBSYSTEM_MAP_OFFSET,
                game_flow_manager_offset,
                0x60,
                0x0,
            ]),
        }
    }
}

impl<T: Pod, const N: usize> Path<T, N> {
    fn new(path: [u64; N]) -> Self {
        Self {
            path,
            ty: core::marker::PhantomData,
        }
    }

    pub fn read(&self, proc: &Process, module: Address) -> Option<T> {
        proc.read_pointer_path64(module, &self.path).ok()
    }
}