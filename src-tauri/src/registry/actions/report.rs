use anyhow::anyhow;

use super::super::Registry;

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

pub trait ReportActions {
    async fn update_network_report(download_id: i64, bytes_len: u64) -> anyhow::Result<()>;
    async fn update_disk_report(
        download_id: i64,
        chunk_index: i64,
        bytes_len: u64,
    ) -> anyhow::Result<()>;
    async fn update_chunk_buffer_report(
        download_id: i64,
        chunk_index: i64,
        bytes: Vec<u8>,
    ) -> anyhow::Result<()>;
}

impl ReportActions for Registry {
    async fn update_disk_report(
        download_id: i64,
        chunk_index: i64,
        bytes_len: u64,
    ) -> anyhow::Result<()> {
        let reports = Arc::clone(&Self::get_state().reports);
        let maybe_report = reports.get(&download_id);
        if let Some(report) = maybe_report {
            report
                .chunks_wrote_bytes
                .entry(chunk_index as i64)
                .and_modify(|atomic| {
                    atomic.fetch_add(bytes_len, Ordering::Relaxed);
                })
                .or_insert(AtomicU64::new(bytes_len));

            report
                .total_wrote_bytes
                .fetch_add(bytes_len, Ordering::Relaxed);

            report.wrote_bytes.fetch_add(bytes_len, Ordering::Relaxed);
        }

        Ok(())
    }

    async fn update_network_report(download_id: i64, bytes_len: u64) -> anyhow::Result<()> {
        let reports = Arc::clone(&Self::get_state().reports);
        let maybe_report = reports.get(&download_id);
        if let Some(report) = maybe_report {
            report
                .total_downloaded_bytes
                .fetch_add(bytes_len, Ordering::Relaxed);

            report
                .downloaded_bytes
                .fetch_add(bytes_len, Ordering::Relaxed);
        }

        Ok(())
    }

    async fn update_chunk_buffer_report(
        download_id: i64,
        chunk_index: i64,
        _bytes: Vec<u8>,
    ) -> anyhow::Result<()> {
        let reports = Arc::clone(&Self::get_state().reports);
        let report = reports
            .get(&download_id)
            .ok_or_else(|| anyhow!("cannot find report with download id {}", download_id))?;

        let buffer = report.buffer.get(&chunk_index).ok_or_else(|| {
            anyhow!(
                "cannot find buffer report with download id {} and indec {}",
                download_id,
                chunk_index
            )
        })?;

        // let mut buffer = buffer.lock().await;

        Ok(())
    }
}
