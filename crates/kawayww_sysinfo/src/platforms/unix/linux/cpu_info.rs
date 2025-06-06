use crate::{models::CPUInfo, platforms::unix::utils};
use std::{
    collections::HashSet, io::{BufRead, BufReader}, path::Path
};

pub fn new() -> Option<CPUInfo> {
    let cpu_info = utils::read_file("/proc/cpuinfo")?;
    let scaling_cur_freq = utils::read_file("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")?;
    
    let brand = get_brand(&cpu_info)?;
    let physical_core_num = get_physical_core_num(&cpu_info);
    let logical_core_num = get_logical_core_num(&cpu_info)?;
    let freq_khz = get_freq_khz(&scaling_cur_freq)?;

    Some(CPUInfo::new(brand, physical_core_num, logical_core_num, freq_khz))
}

pub(crate) fn parse_cpuinfo(content: &str) -> Option<(String, usize, usize)> {
    Some((get_brand(content)?, get_physical_core_num_via_proc(content)?, get_logical_core_num(content)?))
}

pub(crate) fn get_freq_khz(content: &str) -> Option<u64> {
    content.trim().parse().ok()
}

pub(crate) fn get_brand(content: &str) -> Option<String> {
    let mut brand = None;

    for line in content.lines().filter(|x| !x.is_empty()) {
        let (head, tail) = line.split_once(':').map(|(a, b)| (a.trim(), b.trim()))?;
        if brand.is_none() && head == "model name" {
            brand = tail.split(" with").next().map(|s| s.to_string());
            break
        }
    }

    brand
}

pub(crate) fn get_physical_core_num(content: &str) -> usize {
    if let Some(cores) = get_physical_core_num_via_sys() {
        return cores;
    }

    if let Some(cores) = get_physical_core_num_via_proc(content) {
        return cores;
    }

    // avoid logical error
    1
}


pub(crate) fn get_physical_core_num_via_proc(content: &str) -> Option<usize> {
    let mut physical_core_num = None;

    for line in content.lines().filter(|x| !x.is_empty()) {
        let (head, tail) = line.split_once(':').map(|(a, b)| (a.trim(), b.trim()))?;
        if physical_core_num.is_none() && head == "cpu cores" {
            physical_core_num = tail.parse().ok();
            break
        }
    }

    physical_core_num
}

pub(crate) fn get_physical_core_num_via_sys() -> Option<usize> {
    let mut core_ids = HashSet::new();
    let sys_cpu_dir = Path::new("/sys/devices/system/cpu");

    if !sys_cpu_dir.exists() {
        return None;
    }

    for entry in std::fs::read_dir(sys_cpu_dir).ok()? {
        let cpu_dir = entry.ok()?.path();
        let cpu_name = cpu_dir.file_name()?.to_str()?;

        if !cpu_name.starts_with("cpu") || !cpu_name[3..].chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        let core_id_path = cpu_dir.join("topology/core_id");
        if !core_id_path.exists() {
            continue; // continue when dir without topology
        }
        let core_id = std::fs::read_to_string(core_id_path).ok()?;
        core_ids.insert(core_id.trim().to_string());
    }

    (!core_ids.is_empty()).then_some(core_ids.len())
}

pub(crate) fn get_logical_core_num(content: &str) -> Option<usize> {
    let core_num = std::thread::available_parallelism().ok()?.get();
    Some(core_num)
}

#[test]
fn cpu_info() {
    let scaling_cur_freq = utils::FILES::SCALING_CUR_FREQ;
    let cpu_info = utils::FILES::CPUINFO;

    let brand = get_brand(&cpu_info).unwrap();
    let physical_core_num_via_proc = get_physical_core_num_via_proc(&cpu_info).unwrap();
    let _physical_core_num_via_sys = get_physical_core_num_via_sys().unwrap();
    let _logical_core_num = get_logical_core_num(&cpu_info).unwrap();
    let freq_khz = get_freq_khz(scaling_cur_freq).unwrap();

    assert_eq!(brand, "AMD Ryzen 7 5700U");
    assert_eq!(physical_core_num_via_proc, 1);
    assert_eq!(freq_khz, 1776714);
}
