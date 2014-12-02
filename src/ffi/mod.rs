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


#[repr(C)]
pub struct ENetAddress {
    pub host: libc::c_uint,
    pub port: libc::c_ushort,
}

#[link(name = "enet")]
extern {
    pub fn enet_initialize() -> libc::c_int;
    pub fn enet_deinitialize();
    pub fn enet_linked_version() -> libc::c_uint;
    pub fn enet_host_create(address           : *const libc::c_void,
                            peerCount         : libc::size_t,
                            channelCount      : libc::size_t,
                            incomingBandwidth : libc::c_uint,
                            outgoingBandwidth : libc::c_uint) -> *mut libc::c_void;
    pub fn enet_host_destroy(host : *mut libc::c_void);
}
