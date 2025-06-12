use tokio::time::Instant;

pub fn calc_download_speed(last: &mut Instant, received_bytes: &mut u64, speed: &mut f64) {
    let now = Instant::now();

    let elapsed = now.duration_since(*last).as_secs_f64();
    *speed = if elapsed >= 0.001 {
        (*received_bytes as f64 / elapsed) / 1024.0
    } else {
        *speed
    };

    *last = Instant::now();
    *received_bytes = 0;
}

pub fn get_chunk_ranges(content_length: u64, chunk: u8) -> Result<Vec<(u64, u64)>, String> {
    let chunk = chunk as u64;
    let mut ranges = Vec::with_capacity(chunk as usize);

    let base_chunk_size = content_length / chunk;
    let remainder = content_length % chunk;

    let mut start = 0;

    for i in 0..chunk {
        let extra = if i < remainder { 1 } else { 0 };
        let end = start + base_chunk_size + extra - 1;

        ranges.push((start, end));
        start = end + 1;
    }

    Ok(ranges)
}
