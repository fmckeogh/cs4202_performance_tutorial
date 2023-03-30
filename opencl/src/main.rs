use {ocl::ProQue, std::time::Instant};

fn main() {
    let src = r#"
    __kernel void add(__global float* buffer, float initial, float scalar) {
        buffer[get_global_id(0)] = initial;
        buffer[get_global_id(0)] /= powr(scalar, 1.42132);
        buffer[get_global_id(0)] *= powr(484.23423, 0.48474);
        buffer[get_global_id(0)] /= powr(scalar, 1.42132);
        buffer[get_global_id(0)] *= powr(484.23423, 0.48474);
        buffer[get_global_id(0)] /= powr(scalar, 1.42132);
        buffer[get_global_id(0)] *= powr(484.23423, 0.48474);
        buffer[get_global_id(0)] /= powr(scalar, 1.42132);
        buffer[get_global_id(0)] *= powr(484.23423, 0.48474);
    }
"#;

    let pro_que = ProQue::builder().src(src).dims(1 << 29).build().unwrap();

    let buffer = pro_que.create_buffer::<f32>().unwrap();

    let kernel = pro_que
        .kernel_builder("add")
        .arg(&buffer)
        .arg(10.0f32)
        .arg(3.141f32)
        .build()
        .unwrap();

    let start = Instant::now();
    unsafe {
        kernel.enq().unwrap();
    }
    let mut vec = vec![0.0f32; buffer.len()];
    buffer.read(&mut vec).enq().unwrap();
    println!("gpu {} us", start.elapsed().as_micros());

    let start = Instant::now();
    let mut vec = vec![0.0; buffer.len()];
    for element in &mut vec {
        *element = 10.0f32;
        *element /= 3.141f32.powf(1.42132);
        *element *= 484.23423f32.powf(0.48474);
        *element /= 3.141f32.powf(1.42132);
        *element *= 484.23423f32.powf(0.48474);
        *element /= 3.141f32.powf(1.42132);
        *element *= 484.23423f32.powf(0.48474);
        *element /= 3.141f32.powf(1.42132);
        *element *= 484.23423f32.powf(0.48474);
    }
    println!("cpu {} us", start.elapsed().as_micros());
}
