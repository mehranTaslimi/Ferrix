impl super::DownloadsManager {
    pub(super) fn get_chunk_ranges(content_length: u64, chunk_count: u64) -> Vec<(u64, u64)> {
        let chunk = chunk_count;
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

        ranges
    }
}
