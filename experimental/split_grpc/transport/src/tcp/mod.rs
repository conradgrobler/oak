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
use std::mem::MaybeUninit;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{self, AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tonic::transport::server::Connected;

pub struct TcpConnector {
  addr: SocketAddr,
}

impl TcpConnector {
  pub fn new() -> Self {
    TcpConnector {
      addr: "".parse().unwrap(),
    }
  }
}

#[async_trait]
impl crate::Connector for TcpConnector {
  async fn connect(
    &self,
    incoming_addr: SocketAddr,
  ) -> io::Result<Box<dyn crate::ConnectedStream>> {
    Ok(Box::from(TcpConnectedStream {
      remote_address: incoming_addr,
      inner_stream: TcpStream::connect(self.addr).await?,
    }))
  }
}

pub struct TcpConnectedStream {
  remote_address: SocketAddr,
  inner_stream: TcpStream,
}

impl crate::ConnectedStream for TcpConnectedStream {}

impl Connected for TcpConnectedStream {
  fn remote_addr(&self) -> Option<SocketAddr> {
    Some(self.remote_address.clone())
  }
}

impl AsyncRead for TcpConnectedStream {
  unsafe fn prepare_uninitialized_buffer(&self, _: &mut [MaybeUninit<u8>]) -> bool {
    false
  }

  fn poll_read(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    self.inner_stream.poll_read(context, buffer)
  }
}

impl AsyncWrite for TcpConnectedStream {
  fn poll_write(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    self.inner_stream.poll_write(context, buffer)
  }

  fn poll_write_buf<B: Buf>(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    self.inner_stream.poll_write_buf(context, buffer)
  }

  #[inline]
  fn poll_flush(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<()>> {
    self.inner_stream.poll_flush(context)
  }

  fn poll_shutdown(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<()>> {
    self.inner_stream.poll_shutdown(context)
  }
}
