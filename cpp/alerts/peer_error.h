#pragma once
#include <libtorrent/alert_types.hpp>
#include "cpp/error.h"
#include <cstdint>

namespace ltrs {
uint8_t peer_error_alert_get_op(lt::peer_error_alert* alert);
Error peer_error_alert_get_error(lt::peer_error_alert* alert);

} // namespace ltrs
