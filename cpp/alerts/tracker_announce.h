#pragma once
#include <libtorrent/alert_types.hpp>

namespace ltrs {
uint8_t tracker_announce_alert_get_event(lt::tracker_announce_alert* alert);
uint8_t tracker_announce_alert_get_version(lt::tracker_announce_alert* alert);

} // namespace ltrs
