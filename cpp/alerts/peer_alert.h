#pragma once
#include <array>
#include <libtorrent/alert_types.hpp>

namespace ltrs {
    std::array<unsigned char, 20> peer_alert_get_pid(lt::peer_alert* alert);

} // namespace ltrs
