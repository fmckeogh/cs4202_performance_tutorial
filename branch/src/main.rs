use {
    rand::{thread_rng, Rng},
    std::{fs::File, hint::black_box, io::Read, os::fd::FromRawFd, time::Instant},
};

fn init_perf() -> File {
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
    let raw_fd = unsafe { perf_event_open_sys::perf_event_open(&mut attrs, 0, -1, -1, 0) };
    if raw_fd < 0 {
        panic!("perf_event_open failed");
    }

    unsafe { File::from_raw_fd(raw_fd) }
}

fn main() {
    let f = init_perf();

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

        print!("{} {}", hi, lo);

        println!(
            "\rrandom: {} us, {} branch mispredicts",
            start.elapsed().as_micros(),
            final_count - count,
        );
    }

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

        print!("{} {}", hi, lo);

        println!(
            "\rsorted: {} us, {} branch mispredicts",
            start.elapsed().as_micros(),
            final_count - count,
        );
    }
}

fn read_count(mut f: &File) -> i64 {
    let mut buf = [0u8; 8];
    f.read_exact(&mut buf).unwrap();
    unsafe { std::mem::transmute::<_, i64>(buf) }
}
