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

pub const ENET_HOST_ANY: libc::c_uint = 0;

pub const ENET_EVENT_TYPE_CONNECT: libc::c_uint    = 1;
pub const ENET_EVENT_TYPE_DISCONNECT: libc::c_uint = 2;
pub const ENET_EVENT_TYPE_RECEIVE: libc::c_uint    = 3;

pub const ENET_PACKET_FLAG_RELIABLE: libc::c_uint = 1;
pub const ENET_PACKET_FLAG_UNSEQUENCED: libc::c_uint = 2;
pub const ENET_PACKET_FLAG_NO_ALLOCATE: libc::c_uint = 4;
pub const ENET_PACKET_UNRELIABLE_FRAGMENT: libc::c_uint = 8;
pub const ENET_PACKET_FLAG_SENT: libc::c_uint = 256;

#[repr(C)]
pub struct ENetAddress {
    pub host: libc::c_uint,
    pub port: libc::c_ushort,
}

#[repr(C)]
pub struct ENetEvent {
    pub etype: libc::c_uint,
    pub peer: *mut ENetPeer,
    pub channelID: libc::c_uchar,
    pub data: libc::c_uint,
    pub packet: *mut ENetPacket,
}

#[repr(C)]
pub struct ENetPacket {
    pub referenceCount: libc::size_t,
    pub flags: libc::c_uint,
    pub data: *mut libc::c_void,
    pub dataLength: libc::size_t,
    pub freeCallback: *mut libc::c_void,
    pub userData: *mut libc::c_void
}

#[repr(C)]
pub struct ENetListNode {
    pub next: *mut ENetListNode,
    pub previous: *mut ENetListNode
}

#[repr(C)]
pub struct ENetPeer {
    pub dispatchList: ENetListNode,
    pub host: *mut libc::c_void,
    pub outgoingPeerID: libc::c_ushort,
    pub incomingPeerID: libc::c_ushort,
    pub connectID: libc::c_uint,
    pub outgoingSessionID: libc::c_uchar,
    pub incomingSessionID: libc::c_uchar,
    pub address: ENetAddress,
    pub data: *mut libc::c_void,

    // The rest of the fields has been omitted for now
}

#[link(name = "enet")]
#[cfg_attr(target_os = "windows", link(name = "winmm"))]
extern {
    pub fn enet_initialize() -> libc::c_int;
    pub fn enet_deinitialize();
    pub fn enet_linked_version() -> libc::c_uint;
    pub fn enet_host_create(address           : *const ENetAddress,
                            peerCount         : libc::size_t,
                            channelCount      : libc::size_t,
                            incomingBandwidth : libc::c_uint,
                            outgoingBandwidth : libc::c_uint) -> *mut libc::c_void;
    pub fn enet_host_service(host : *mut libc::c_void, event : *mut libc::c_void, timeout : libc::c_uint) -> libc::c_int;
    pub fn enet_host_destroy(host : *mut libc::c_void);
    pub fn enet_host_connect(host: *mut libc::c_void, address: *const ENetAddress, channelCount: libc::size_t, data: libc::c_uint) -> *mut ENetPeer;

    pub fn enet_packet_create(data: *mut libc::c_void, dataLength: libc::size_t, flags: libc::c_uint) -> *mut ENetPacket;
    pub fn enet_packet_destroy(packet: *mut ENetPacket);
    pub fn enet_peer_send(peer: *mut ENetPeer, channelID: libc::c_uchar, packet: *mut ENetPacket);
    pub fn enet_host_flush(host: *mut libc::c_void);
    pub fn enet_address_set_host(address: *mut ENetAddress, hostName: *const libc::c_char);
}
