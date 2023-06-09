extern crate event;
extern crate netutils;
extern crate syscall;

use std::cell::RefCell;
use std::fs::File;
use std::io::{ErrorKind, Read, Result, Write};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::sync::Arc;
use std::{env, thread};

use event::EventQueue;
use std::time::Duration;
use syscall::{EventFlags, Packet, SchemeBlockMut, PHYSMAP_NO_CACHE, PHYSMAP_WRITE};

pub mod device;
#[rustfmt::skip]
mod ixgbe;

const IXGBE_MMIO_SIZE: usize = 512 * 1024;

fn handle_update(
    socket: &mut File,
    device: &mut device::Intel8259x,
    todo: &mut Vec<Packet>,
) -> Result<bool> {
    // Handle any blocked packets
    let mut i = 0;
    while i < todo.len() {
        if let Some(a) = device.handle(&todo[i]) {
            let mut packet = todo.remove(i);
            packet.a = a;
            socket.write(&packet)?;
        } else {
            i += 1;
        }
    }

    // Check that the socket is empty
    loop {
        let mut packet = Packet::default();
        match socket.read(&mut packet) {
            Ok(0) => return Ok(true),
            Ok(_) => (),
            Err(err) => {
                if err.kind() == ErrorKind::WouldBlock {
                    break;
                } else {
                    return Err(err);
                }
            }
        }

        if let Some(a) = device.handle(&packet) {
            packet.a = a;
            socket.write(&packet)?;
        } else {
            todo.push(packet);
        }
    }

    Ok(false)
}

fn main() {
    let mut args = env::args().skip(1);

    let mut name = args.next().expect("ixgbed: no name provided");
    name.push_str("_ixgbe");

    let bar_str = args.next().expect("ixgbed: no address provided");
    let bar = usize::from_str_radix(&bar_str, 16).expect("ixgbed: failed to parse address");

    let irq_str = args.next().expect("ixgbed: no irq provided");
    let irq = irq_str.parse::<u8>().expect("ixgbed: failed to parse irq");

    println!(" + IXGBE {} on: {:X} IRQ: {}", name, bar, irq);

    redox_daemon::Daemon::new(move |daemon| {
        let socket_fd = syscall::open(
            ":network",
            syscall::O_RDWR | syscall::O_CREAT | syscall::O_NONBLOCK,
        )
        .expect("ixgbed: failed to create network scheme");
        let socket = Arc::new(RefCell::new(unsafe {
            File::from_raw_fd(socket_fd as RawFd)
        }));

        daemon.ready().expect("ixgbed: failed to signal readiness");

        let mut irq_file =
            File::open(format!("irq:{}", irq)).expect("ixgbed: failed to open IRQ file");

        let address = unsafe {
            syscall::physmap(bar, IXGBE_MMIO_SIZE, PHYSMAP_WRITE | PHYSMAP_NO_CACHE)
                .expect("ixgbed: failed to map address")
        };
        {
            let device = Arc::new(RefCell::new(
                device::Intel8259x::new(address, IXGBE_MMIO_SIZE)
                    .expect("ixgbed: failed to allocate device")
            ));

            let mut event_queue =
                EventQueue::<usize>::new().expect("ixgbed: failed to create event queue");

            syscall::setrens(0, 0).expect("ixgbed: failed to enter null namespace");

            let todo = Arc::new(RefCell::new(Vec::<Packet>::new()));

            let device_irq = device.clone();
            let socket_irq = socket.clone();
            let todo_irq = todo.clone();
            event_queue
                .add(
                    irq_file.as_raw_fd(),
                    move |_event| -> Result<Option<usize>> {
                        let mut irq = [0; 8];
                        irq_file.read(&mut irq)?;
                        if device_irq.borrow().irq() {
                            irq_file.write(&irq)?;

                            if handle_update(
                                &mut socket_irq.borrow_mut(),
                                &mut device_irq.borrow_mut(),
                                &mut todo_irq.borrow_mut(),
                            )? {
                                return Ok(Some(0));
                            }

                            let next_read = device_irq.borrow().next_read();
                            if next_read > 0 {
                                return Ok(Some(next_read));
                            }
                        }
                        Ok(None)
                    },
                )
                .expect("ixgbed: failed to catch events on IRQ file");

            let device_packet = device.clone();
            let socket_packet = socket.clone();

            event_queue
                .add(socket_fd as RawFd, move |_event| -> Result<Option<usize>> {
                    if handle_update(
                        &mut socket_packet.borrow_mut(),
                        &mut device_packet.borrow_mut(),
                        &mut todo.borrow_mut(),
                    )? {
                        return Ok(Some(0));
                    }

                    let next_read = device_packet.borrow().next_read();
                    if next_read > 0 {
                        return Ok(Some(next_read));
                    }

                    Ok(None)
                })
                .expect("ixgbed: failed to catch events on scheme file");

            let send_events = |event_count| {
                for (handle_id, _handle) in device.borrow().handles.iter() {
                    socket
                        .borrow_mut()
                        .write(&Packet {
                            id: 0,
                            pid: 0,
                            uid: 0,
                            gid: 0,
                            a: syscall::number::SYS_FEVENT,
                            b: *handle_id,
                            c: syscall::flag::EVENT_READ.bits(),
                            d: event_count,
                        })
                        .expect("ixgbed: failed to write event");
                }
            };

            for event_count in event_queue
                .trigger_all(event::Event { fd: 0, flags: EventFlags::empty() })
                .expect("ixgbed: failed to trigger events")
            {
                send_events(event_count);
            }

            loop {
                let event_count = event_queue.run().expect("ixgbed: failed to handle events");
                if event_count == 0 {
                    //TODO: Handle todo
                    break;
                }

                send_events(event_count);
            }
        }
        unsafe {
            let _ = syscall::physunmap(address);
        }
        std::process::exit(0);
    }).expect("ixgbed: failed to daemonize");

    thread::sleep(Duration::from_secs(20));
}
