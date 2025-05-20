// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

mod ctr500;
mod ctr600;
mod ctr700;
mod ctr750;
mod ctr800;
mod fallback;
mod pi;

use crate::shm;
use crate::Io;

pub fn load_device_definition(device_name: &str) -> Io {
    match device_name {
        "ctr500" => ctr500::definition(),
        "ctr600" => ctr600::definition(),
        "ctr700" => ctr700::definition(),
        "ctr750" => ctr750::definition(),
        "ctr800" => ctr800::definition(),
        "pi" => pi::definition(),
        _ => fallback::definition(),
    }
}

pub fn load_device_definition_shm(device_name: &str) -> Option<(Io, shm::Mappings)> {
    match device_name {
        "ctr700" => Some(ctr700::definition_shm()),
        "ctr750" => Some(ctr750::definition_shm()),
        "ctr800" => Some(ctr800::definition_shm()),
        _ => fallback::definition_shm(),
    }
}
