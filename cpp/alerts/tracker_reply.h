#pragma once
#include <cstdint>
#include <libtorrent/alert_types.hpp>

namespace ltrs {
    int tracker_reply_alert_get_num_peers(lt::tracker_reply_alert* alert);
    uint8_t tracker_reply_alert_get_version(lt::tracker_reply_alert* alert);
} // namespace ltrs
