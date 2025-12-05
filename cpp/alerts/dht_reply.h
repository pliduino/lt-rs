#pragma once
#include <libtorrent/alert_types.hpp>

namespace ltrs {
int dht_reply_alert_get_num_peers(lt::dht_reply_alert* alert);

} // namespace ltrs
