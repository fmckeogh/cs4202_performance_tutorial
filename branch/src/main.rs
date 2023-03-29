use std::io::Read;

use {
    rand::{thread_rng, Rng},
    std::{fs::File, os::fd::FromRawFd, time::Instant},
};

fn init_perf() -> i32 {
    // Construct a zero-filled `perf_event_attr`.
    let mut attrs = perf_event_open_sys::bindings::perf_event_attr::default();

    // Populate the fields we need.
    attrs.size = std::mem::size_of::<perf_event_open_sys::bindings::perf_event_attr>() as u32;
    attrs.type_ = perf_event_open_sys::bindings::PERF_TYPE_HARDWARE;
    attrs.config = perf_event_open_sys::bindings::PERF_COUNT_HW_BRANCH_MISSES as u64;
    attrs.set_disabled(0);
    attrs.set_exclude_kernel(1);
    attrs.set_exclude_hv(1);

    // Make the system call.
    unsafe { perf_event_open_sys::perf_event_open(&mut attrs, 0, -1, -1, 0) }
}

fn main() {
    let f = unsafe { File::from_raw_fd(init_perf()) };

    let mut data = vec![0u32; 10_000_000];
    thread_rng().fill(data.as_mut_slice());

    let mut hi = 0;
    let mut lo = 0;

    {
        let start = Instant::now();
        let count = read_count(&f);

        for element in &data {
            if *element > (u32::MAX / 2) {
                hi += 1;
            } else {
                lo += 1;
            }
        }

        let final_count = read_count(&f);

        println!(
            "random: {} us, {} branch mispredicts, {}, {}",
            start.elapsed().as_micros(),
            final_count - count,
            hi,
            lo
        );
    }

    println!("sorting...");
    thread_rng().fill(data.as_mut_slice());
    // sort the data
    data.sort();
    hi = 0;
    lo = 0;

    {
        let start = Instant::now();
        let count = read_count(&f);
        for element in &data {
            if *element > (u32::MAX / 2) {
                hi += 1;
            } else {
                lo += 1;
            }
        }
        let final_count = read_count(&f);

        println!(
            "sorted: {} us, {} branch mispredicts, {}, {}",
            start.elapsed().as_micros(),
            final_count - count,
            hi,
            lo
        );
    }
}

fn read_count(mut f: &File) -> i64 {
    let mut buf = [0u8; 8];
    f.read_exact(&mut buf).unwrap();
    unsafe { std::mem::transmute::<_, i64>(buf) }
}
