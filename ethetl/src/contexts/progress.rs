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
use ticker::Ticker;

#[derive(Clone, Debug)]
pub struct ProgressValue {
    pub blocks: usize,
    pub txs: usize,
    pub receipts: usize,
    pub logs: usize,
    pub token_transfers: usize,
    pub ens: usize,
    pub traces: usize,
}

#[derive(Debug)]
pub struct Progress {
    all: AtomicUsize,
    blocks: AtomicUsize,
    txs: AtomicUsize,
    receipts: AtomicUsize,
    logs: AtomicUsize,
    token_transfers: AtomicUsize,
    ens: AtomicUsize,
    traces: AtomicUsize,
}

impl Progress {
    pub fn create() -> Arc<Progress> {
        Arc::new(Progress {
            all: AtomicUsize::new(0),
            blocks: AtomicUsize::new(0),
            txs: AtomicUsize::new(0),
            receipts: AtomicUsize::new(0),
            logs: AtomicUsize::new(0),
            token_transfers: AtomicUsize::new(0),
            ens: AtomicUsize::new(0),
            traces: AtomicUsize::new(0),
        })
    }

    pub fn inc_all(&self, v: usize) {
        self.all.fetch_add(v, Ordering::Relaxed);
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

    pub fn incr_ens(&self, v: usize) {
        self.ens.fetch_add(v, Ordering::Relaxed);
    }

    pub fn incr_traces(&self, v: usize) {
        self.traces.fetch_add(v, Ordering::Relaxed);
    }

    pub fn value(&self) -> Arc<ProgressValue> {
        Arc::new(ProgressValue {
            blocks: self.blocks.load(Ordering::Relaxed),
            txs: self.txs.load(Ordering::Relaxed),
            receipts: self.receipts.load(Ordering::Relaxed),
            logs: self.logs.load(Ordering::Relaxed),
            token_transfers: self.token_transfers.load(Ordering::Relaxed),
            ens: self.ens.load(Ordering::Relaxed),
            traces: self.traces.load(Ordering::Relaxed),
        })
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let ticker = Ticker::new(0.., Duration::from_secs(2));
            for _i in ticker {
                self.print_progress();
            }
        });
    }

    fn print_progress(&self) {
        let all = self.all.load(Ordering::Relaxed);
        let value = self.value();

        if value.blocks > 0 {
            let percent = ((value.blocks as f32 / all as f32) * 100_f32) as usize;
            log::info!(
                "block {:?} processed/{}, {:?} transactions processed, {:?} receipts processed, {:?} logs processed, {:?} token_transfers processed, {:?} ens processed, {:?} traces processed. Progress is {:.2}",
                value.blocks,
                all,
                value.txs,
                value.receipts,
                value.logs,
                value.token_transfers,
                value.ens,
                value.traces,
                percent.percent(),
            );
        }
    }
}

// Print the final result when the progress dropped.
impl Drop for Progress {
    fn drop(&mut self) {
        self.print_progress();
    }
}
