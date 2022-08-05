// Copyright 2022 BohuTANG.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Debug;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use percentage_rs::Percent;
use tokio::time;

#[derive(Clone, Debug)]
pub struct ProgressValue {
    pub blocks: usize,
    pub txs: usize,
    pub receipts: usize,
    pub logs: usize,
    pub token_transfers: usize,
}

#[derive(Debug)]
pub struct Progress {
    all: usize,
    blocks: AtomicUsize,
    txs: AtomicUsize,
    receipts: AtomicUsize,
    logs: AtomicUsize,
    token_transfers: AtomicUsize,
}

impl Progress {
    pub fn create(all: usize) -> Arc<Progress> {
        Arc::new(Progress {
            all,
            blocks: AtomicUsize::new(0),
            txs: AtomicUsize::new(0),
            receipts: AtomicUsize::new(0),
            logs: AtomicUsize::new(0),
            token_transfers: AtomicUsize::new(0),
        })
    }

    pub fn incr_blocks(&self, v: usize) {
        self.blocks.fetch_add(v, Ordering::Relaxed);
    }

    pub fn incr_txs(&self, v: usize) {
        self.txs.fetch_add(v, Ordering::Relaxed);
    }

    pub fn incr_receipts(&self, v: usize) {
        self.receipts.fetch_add(v, Ordering::Relaxed);
    }

    pub fn incr_logs(&self, v: usize) {
        self.logs.fetch_add(v, Ordering::Relaxed);
    }

    pub fn incr_token_transfers(&self, v: usize) {
        self.token_transfers.fetch_add(v, Ordering::Relaxed);
    }

    pub fn value(&self) -> Arc<ProgressValue> {
        Arc::new(ProgressValue {
            blocks: self.blocks.load(Ordering::Relaxed),
            txs: self.txs.load(Ordering::Relaxed),
            receipts: self.receipts.load(Ordering::Relaxed),
            logs: self.logs.load(Ordering::Relaxed),
            token_transfers: self.token_transfers.load(Ordering::Relaxed),
        })
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(2));
            loop {
                interval.tick().await;
                print_progress(self.all, self.value().clone());
            }
        });
    }
}

// Print the final result when the progress dropped.
impl Drop for Progress {
    fn drop(&mut self) {
        let value = self.value();
        print_progress(self.all, value);
    }
}

fn print_progress(all: usize, value: Arc<ProgressValue>) {
    if value.blocks > 0 {
        let percent = ((value.blocks as f32 / all as f32) * 100_f32) as usize;
        log::info!(
            "{:?} blocks processed, {:?} transactions processed, {:?} receipts processed, {:?} logs processed, {:?} token_transfers processed. Progress is {:.2}",
            value.blocks,
            value.txs,
            value.receipts,
            value.logs,
            value.token_transfers,
            percent.percent(),
        );
    }
}
