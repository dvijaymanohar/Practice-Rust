use std::convert::TryInto;
use std::ptr;

use syscall::io::Dma;
use syscall::error::Result;

use super::hba::{HbaPort, HbaCmdTable, HbaCmdHeader};
use super::Disk;

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(crate) unsafe fn pause() { std::arch::aarch64::__yield(); }

#[cfg(target_arch = "x86")]
#[inline(always)]
pub(crate) unsafe fn pause() { std::arch::x86::_mm_pause(); }

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub(crate) unsafe fn pause() { std::arch::x86_64::_mm_pause(); }

enum BufferKind<'a> {
    Read(&'a mut [u8]),
    Write(&'a [u8]),
}

struct Request {
    address: usize,
    total_sectors: usize,
    sector: usize,
    running_opt: Option<(u32, usize)>,
}

pub struct DiskATA {
    id: usize,
    port: &'static mut HbaPort,
    size: u64,
    request_opt: Option<Request>,
    clb: Dma<[HbaCmdHeader; 32]>,
    ctbas: [Dma<HbaCmdTable>; 32],
    _fb: Dma<[u8; 256]>,
    buf: Dma<[u8; 256 * 512]>
}

impl DiskATA {
    pub fn new(id: usize, port: &'static mut HbaPort) -> Result<Self> {
        let mut clb = unsafe { Dma::zeroed()?.assume_init() };

        let mut ctbas: [_; 32] = (0..32)
            .map(|_| Dma::zeroed().map(|dma| unsafe { dma.assume_init() }))
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .unwrap_or_else(|_| unreachable!());

        let mut fb = unsafe { Dma::zeroed()?.assume_init() };
        let buf = unsafe { Dma::zeroed()?.assume_init() };

        port.init(&mut clb, &mut ctbas, &mut fb);

        let size = unsafe { port.identify(&mut clb, &mut ctbas).unwrap_or(0) };

        Ok(DiskATA {
            id: id,
            port: port,
            size: size,
            request_opt: None,
            clb: clb,
            ctbas,
            _fb: fb,
            buf: buf
        })
    }

    fn request(&mut self, block: u64, mut buffer_kind: BufferKind) -> Result<Option<usize>> {
        let (write, address, total_sectors) = match buffer_kind {
            BufferKind::Read(ref buffer) => (false, buffer.as_ptr() as usize, buffer.len()/512),
            BufferKind::Write(ref buffer) => (true, buffer.as_ptr() as usize, buffer.len()/512),
        };

        //TODO: Go back to interrupt magic
        let use_interrupts = false;
        loop {
            let mut request = match self.request_opt.take() {
                Some(request) => if address == request.address && total_sectors == request.total_sectors {
                    // Keep servicing current request
                    request
                } else {
                    // Have to wait for another request to finish
                    self.request_opt = Some(request);
                    return Ok(None);
                },
                None => {
                    // Create new request
                    Request {
                        address,
                        total_sectors,
                        sector: 0,
                        running_opt: None,
                    }
                }
            };

            // Finish a previously running request
            if let Some(running) = request.running_opt.take() {
                if self.port.ata_running(running.0) {
                    // Continue waiting for request
                    request.running_opt = Some(running);
                    self.request_opt = Some(request);
                    if use_interrupts {
                        return Ok(None);
                    } else {
                        unsafe { pause(); }
                        continue;
                    }
                }

                self.port.ata_stop(running.0)?;

                if let BufferKind::Read(ref mut buffer) = buffer_kind {
                    unsafe { ptr::copy(self.buf.as_ptr(), buffer.as_mut_ptr().add(request.sector * 512), running.1 * 512); }
                }

                request.sector += running.1;
            }

            if request.sector < request.total_sectors {
                // Start a new request
                let sectors = if request.total_sectors - request.sector >= 255 {
                    255
                } else {
                    request.total_sectors - request.sector
                };

                if let BufferKind::Write(ref buffer) = buffer_kind {
                    unsafe { ptr::copy(buffer.as_ptr().add(request.sector * 512), self.buf.as_mut_ptr(), sectors * 512); }
                }

                if let Some(slot) = self.port.ata_dma(block + request.sector as u64, sectors, write, &mut self.clb, &mut self.ctbas, &mut self.buf) {
                    request.running_opt = Some((slot, sectors));
                }

                self.request_opt = Some(request);

                if use_interrupts {
                    return Ok(None);
                } else {
                    unsafe { pause(); }
                    continue;
                }
            } else {
                // Done
                return Ok(Some(request.sector * 512));
            }
        }
    }
}

impl Disk for DiskATA {
    fn id(&self) -> usize {
        self.id
    }

    fn size(&mut self) -> u64 {
        self.size
    }

    fn read(&mut self, block: u64, buffer: &mut [u8]) -> Result<Option<usize>> {
        self.request(block, BufferKind::Read(buffer))
    }

    fn write(&mut self, block: u64, buffer: &[u8]) -> Result<Option<usize>> {
        self.request(block, BufferKind::Write(buffer))
    }

    fn block_length(&mut self) -> Result<u32> {
        Ok(512)
    }
}
