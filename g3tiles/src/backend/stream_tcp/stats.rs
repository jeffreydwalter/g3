/*
 * Copyright 2024 ByteDance and/or its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(unused)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use arc_swap::ArcSwapOption;

use g3_types::metrics::{MetricsName, StaticMetricsTags};
use g3_types::stats::StatId;

pub(crate) struct StreamTcpBackendStats {
    name: MetricsName,
    id: StatId,
    extra_metrics_tags: Arc<ArcSwapOption<StaticMetricsTags>>,

    conn_attempt: AtomicU64,
    conn_established: AtomicU64,
}

impl StreamTcpBackendStats {
    pub(crate) fn new(name: &MetricsName) -> Self {
        StreamTcpBackendStats {
            name: name.clone(),
            id: StatId::new(),
            extra_metrics_tags: Arc::new(ArcSwapOption::new(None)),
            conn_attempt: AtomicU64::new(0),
            conn_established: AtomicU64::new(0),
        }
    }

    pub(crate) fn set_extra_tags(&self, tags: Option<Arc<StaticMetricsTags>>) {
        self.extra_metrics_tags.store(tags);
    }

    pub(crate) fn load_extra_tags(&self) -> Option<Arc<StaticMetricsTags>> {
        self.extra_metrics_tags.load_full()
    }

    #[inline]
    pub(crate) fn name(&self) -> &MetricsName {
        &self.name
    }

    #[inline]
    pub(crate) fn stat_id(&self) -> StatId {
        self.id
    }

    pub(crate) fn add_conn_attempt(&self) {
        self.conn_attempt.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn conn_attempt(&self) -> u64 {
        self.conn_attempt.load(Ordering::Relaxed)
    }

    pub(crate) fn add_conn_established(&self) {
        self.conn_established.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn conn_established(&self) -> u64 {
        self.conn_established.load(Ordering::Relaxed)
    }
}
