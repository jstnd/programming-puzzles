use rustc_hash::FxHashMap;

pub fn part1(input: &str) -> usize {
    let devices = parse(input);
    let mut cache = FxHashMap::default();
    paths1("you", &devices, &mut cache)
}

pub fn part2(input: &str) -> usize {
    let devices = parse(input);
    let mut cache = FxHashMap::default();

    // Each device state will be a tuple in the form of (&str, bool, bool), where:
    // &str => name of device
    // bool(1) => flag if we've seen `dac` in this path
    // bool(2) => flag if we've seen `fft` in this path
    paths2(("svr", false, false), &devices, &mut cache)
}

fn parse(input: &str) -> FxHashMap<&str, Vec<&str>> {
    let mut devices = FxHashMap::default();

    input.trim().lines().for_each(|line| {
        let (device, outputs) = line.split_once(": ").unwrap();
        let outputs = outputs.split_whitespace().collect();
        devices.insert(device, outputs);
    });

    devices
}

fn paths1<'a>(
    device: &'a str,
    devices: &'a FxHashMap<&str, Vec<&str>>,
    cache: &mut FxHashMap<&'a str, usize>,
) -> usize {
    // Check if the cache contains this device, and return the cached paths if so.
    if cache.contains_key(device) {
        return cache[device];
    } else if devices[device].contains(&"out") {
        // The `out` device has been found, so record the current device in the cache and return.
        cache.insert(device, 1);
        return 1;
    }

    // Recursively find the number of paths for all of this device's outputs.
    let paths = devices[device]
        .iter()
        .map(|output| paths1(output, devices, cache))
        .sum();

    // Insert the device's paths into the cache and return.
    cache.insert(device, paths);
    paths
}

fn paths2<'a>(
    device: (&'a str, bool, bool),
    devices: &'a FxHashMap<&str, Vec<&str>>,
    cache: &mut FxHashMap<(&'a str, bool, bool), usize>,
) -> usize {
    // Check if the cache contains this device's state, and return the cached paths if so.
    if cache.contains_key(&device) {
        return cache[&device];
    }

    // Determine if we've found `dac` and/or `fft` at this point.
    let found_dac = device.1 || device.0 == "dac";
    let found_fft = device.2 || device.0 == "fft";

    if devices[device.0].contains(&"out") {
        // Determine if this path was valid by checking if we've found both `dac` and `fft`.
        let is_valid_path = (found_dac && found_fft) as usize;
        cache.insert(device, is_valid_path);
        return is_valid_path;
    }

    // Recursively find the number of paths for all of this device's outputs.
    let paths = devices[device.0]
        .iter()
        .map(|output| paths2((output, found_dac, found_fft), devices, cache))
        .sum();

    // Insert the device's paths into the cache and return.
    cache.insert(device, paths);
    paths
}
