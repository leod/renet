/*
 *           _           _            _             _          _
 *          /\ \        /\ \         /\ \     _    /\ \       /\ \
 *         /  \ \      /  \ \       /  \ \   /\_\ /  \ \      \_\ \
 *        / /\ \ \    / /\ \ \     / /\ \ \_/ / // /\ \ \     /\__ \
 *       / / /\ \_\  / / /\ \_\   / / /\ \___/ // / /\ \_\   / /_ \ \
 *      / / /_/ / / / /_/_ \/_/  / / /  \/____// /_/_ \/_/  / / /\ \ \
 *     / / /__\/ / / /____/\    / / /    / / // /____/\    / / /  \/_/
 *    / / /_____/ / /\____\/   / / /    / / // /\____\/   / / /
 *   / / /\ \ \  / / /______  / / /    / / // / /______  / / /
 *  / / /  \ \ \/ / /_______\/ / /    / / // / /_______\/_/ /
 *  \/_/    \_\/\/__________/\/_/     \/_/ \/__________/\_\/
 *
 *
 * This file is released under terms described in the LICENSE file at
 * the top directory of this repository.
 *
 * Please contact Mark McDermott <mark.elias.mcdermott@gmail.com> or
 * the current maintainer of this software if you do not have a copy of
 * the license file.
 *
 * -------------------------------------------------------------------
 *  REnet is a rust language wrapper for the ENet networking library
 *
 *  TODO List
 * -------------------------------------------------------------------
 */

extern crate libc;

use libc::{c_uint,c_void,size_t,c_int};

#[allow(non_snake_case)]
pub mod ffi;

pub fn initialize() -> Result<(), i32>  {
    unsafe {
        match ffi::enet_initialize() {
            0    => Ok(()),
            code => Err(code)
        }
    }
}

pub fn deinitialize() {
    unsafe {
        ffi::enet_deinitialize();
    }
}


pub fn linked_version() -> u32 {
    unsafe {
        ffi::enet_linked_version() as u32
    }
}


pub struct Host {
    ffi_handle : *mut c_void,
}

pub struct Peer {
    ffi_handle : *mut ffi::ENetPeer,
}

pub struct Packet {
    ffi_handle: *mut ffi::ENetPacket
}

/*
 * ------------------------------------------------------------------
 * Event: enum of event types*
 * ------------------------------------------------------------------
 */
pub enum Event {
    Connect(Peer),
    Receive(Peer,u8,Packet),
    Disconnect(Peer),
    None,
}


/*
 * ------------------------------------------------------------------
 * Host object: a wrapper around ENetHost*
 * ------------------------------------------------------------------
 */


impl Host {
    pub fn new_server(port: u16,
                      peer_count: u32,
                      channel_count: u32,
                      incoming_bandwidth: u32,
                      outgoing_bandwidth: u32) -> Result<Host, String> {
        let address = ffi::ENetAddress {
            host: ffi::ENET_HOST_ANY,
            port: port
        };

        let host_c = unsafe {
            ffi::enet_host_create(&address as *const ffi::ENetAddress,
                                  peer_count as size_t,
                                  channel_count as size_t,
                                  incoming_bandwidth as c_uint,
                                  outgoing_bandwidth as c_uint)
        };

        if host_c.is_null() {
            Err("Could not initialize host".to_string())
        } else {
            Ok(Host { ffi_handle: host_c })
        }
    }

    pub fn connect(timeout_ms: u32,
                   host_name: String,
                   port: u16,
                   channel_count: u32,
                   incoming_bandwidth: u32,
                   outgoing_bandwidth: u32) -> Result<(Host, Peer), String> {
        let host_c = unsafe {
            ffi::enet_host_create(std::ptr::null(),
                                  1, // 1 outgoing connection
                                  channel_count as size_t,
                                  incoming_bandwidth as c_uint,
                                  outgoing_bandwidth as c_uint)
        };

        if host_c.is_null() {
            return Err("Could not initialize host".to_string());
        }

        let host = Host { ffi_handle: host_c };
        let mut address = ffi::ENetAddress {
            host: 0,
            port: port
        };
        let host_name_c = std::ffi::CString::new(host_name).unwrap();

        let peer_c = unsafe {
            let address_ptr = &mut address as *mut ffi::ENetAddress;
            ffi::enet_address_set_host(address_ptr, host_name_c.as_ptr());
            ffi::enet_host_connect(host_c, address_ptr, channel_count as libc::size_t, 0)
        };

        if peer_c.is_null() {
            return Err("Could not initialize peer".to_string());
        }

        match host.service(timeout_ms) {
            Ok(Event::Connect(_)) =>
                Ok((host, Peer { ffi_handle: peer_c })),
            _ =>
                Err("Could not connect".to_string())
        }
    }

    pub fn service(&self, timeout_ms: u32) -> Result<Event, String> {
        let mut ffi_event = ffi::ENetEvent {
            etype: 0,
            peer: std::ptr::null_mut(),
            channelID: 0,
            data: 0,
            packet: std::ptr::null_mut()
        };

        let result: c_int = unsafe {
            let p_evt = &mut ffi_event as *mut _ as *mut c_void;
            ffi::enet_host_service(self.ffi_handle, p_evt, timeout_ms as c_uint)
        };

        if 0 > result {
            Err("Could not service the host".to_string())
        } else if 0 == result {
            Ok(Event::None)
        } else {
            let peer = Peer { ffi_handle: ffi_event.peer };

            match ffi_event.etype {
                ffi::ENET_EVENT_TYPE_CONNECT =>
                    Ok(Event::Connect(peer)),
                ffi::ENET_EVENT_TYPE_DISCONNECT =>
                    Ok(Event::Disconnect(peer)),
                ffi::ENET_EVENT_TYPE_RECEIVE => {
                    Ok(Event::Receive(peer, ffi_event.channelID,
                                      Packet { ffi_handle: ffi_event.packet }))
                }
                _ =>
                    Err("Invalid event".to_string())
            }
        }
    }

    pub fn flush(&self) {
        unsafe { 
            ffi::enet_host_flush(self.ffi_handle)
        }
    }
}

impl Peer {
    pub fn send(&self, data: &[u8], flags: libc::c_uint, channel_id: u8) {
        unsafe {
            let data_ptr = data.as_ptr() as *mut libc::c_void;
            let data_len = data.len() as libc::size_t;
            let packet = ffi::enet_packet_create(data_ptr, data_len, flags);
            ffi::enet_peer_send(self.ffi_handle, channel_id, packet);
        }
    }

    pub fn set_user_data(&self, data: *mut libc::c_void) {
        unsafe {
            (*self.ffi_handle).data = data;
        }
    }

    pub fn get_user_data(&self) -> *mut libc::c_void {
        unsafe {
            (*self.ffi_handle).data
        }
    }
}

impl Packet {
    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts((*self.ffi_handle).data as *const u8,
                (*self.ffi_handle).dataLength as usize)
        }
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        unsafe { ffi::enet_host_destroy(self.ffi_handle); }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe { ffi::enet_packet_destroy(self.ffi_handle); }
    }
}
