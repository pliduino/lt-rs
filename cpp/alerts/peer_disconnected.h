#pragma once
#include <cstdint>
#include <libtorrent/alert_types.hpp>

#include "cpp/error.h"


namespace ltrs {
    uint8_t peer_disconnected_alert_get_socket_type(lt::peer_disconnected_alert* alert);
    uint8_t peer_disconnected_alert_get_op(lt::peer_disconnected_alert* alert);
    Error peer_disconnected_alert_get_error(lt::peer_disconnected_alert* alert);
    uint16_t peer_disconnected_alert_get_reason(lt::peer_disconnected_alert* alert);

} // namespace ltrs
