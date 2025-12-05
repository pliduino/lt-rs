#pragma once
#include <cstdint>
#include <libtorrent/alert_types.hpp>

namespace ltrs {
int32_t hash_failed_alert_get_piece_index(lt::hash_failed_alert* alert);

} // namespace ltrs
