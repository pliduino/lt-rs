#pragma once
#include <cstdint>
#include <libtorrent/alert_types.hpp>

namespace ltrs {
uint8_t peer_connect_alert_get_direction(lt::peer_connect_alert* alert);
uint8_t peer_connect_alert_get_socket_type(lt::peer_connect_alert* alert);

} // namespace ltrs
