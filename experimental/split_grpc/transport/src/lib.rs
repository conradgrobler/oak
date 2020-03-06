//
// Copyright 2020 The Project Oak Authors
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
//

use async_trait::async_trait;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use structopt::StructOpt;
use tokio::io::{self, AsyncRead, AsyncWrite};
use tonic::transport::server::Connected;

pub mod tcp;
pub mod uds;

#[derive(Debug)]
pub enum Transport {
    Tcp,
    Uds,
}

#[derive(Debug)]
pub struct TransportParseError {}

impl Display for TransportParseError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("Invalid transport")
    }
}

impl FromStr for Transport {
    type Err = TransportParseError;
    fn from_str(transport: &str) -> Result<Self, Self::Err> {
        match transport.to_lowercase().as_str() {
            "tcp" => Ok(Transport::Tcp),
            "uds" => Ok(Transport::Uds),
            _ => Err(TransportParseError {}),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long, default_value = "tcp")]
    pub transport: Transport,
}

pub trait ConnectedStream:
    AsyncRead + AsyncWrite + Connected + Unpin + Send + 'static
{
}

#[async_trait]
pub trait Connector {
    async fn connect(&self, incoming_addr: &str) -> io::Result<Box<dyn ConnectedStream>>;
}