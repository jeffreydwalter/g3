/*
 * Copyright 2023 ByteDance and/or its affiliates.
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

use g3_daemon::stat::task::{TcpStreamConnectionStats, TcpStreamHalfConnectionStats};

use crate::module::ftp_over_http::{FtpTaskRemoteControlStats, FtpTaskRemoteTransferStats};

#[derive(Default)]
pub(crate) struct FtpOverHttpServerStats {
    pub(crate) control_read: TcpStreamHalfConnectionStats,
    pub(crate) control_write: TcpStreamHalfConnectionStats,
    pub(crate) transfer_read: TcpStreamHalfConnectionStats,
    pub(crate) transfer_write: TcpStreamHalfConnectionStats,
}

#[derive(Default)]
pub(crate) struct FtpOverHttpTaskStats {
    pub(crate) http_client: TcpStreamConnectionStats,
    pub(crate) ftp_server: FtpOverHttpServerStats,
}

impl FtpTaskRemoteControlStats for FtpOverHttpTaskStats {
    fn add_read_bytes(&self, size: u64) {
        self.ftp_server.control_read.add_bytes(size);
    }

    fn add_write_bytes(&self, size: u64) {
        self.ftp_server.control_write.add_bytes(size);
    }
}

impl FtpTaskRemoteTransferStats for FtpOverHttpTaskStats {
    fn add_read_bytes(&self, size: u64) {
        self.ftp_server.transfer_read.add_bytes(size);
    }

    fn add_write_bytes(&self, size: u64) {
        self.ftp_server.transfer_write.add_bytes(size);
    }
}
