#pragma once
#include "rust/cxx.h"
#include <libtorrent/alert_types.hpp>

namespace ltrs {
rust::Str storage_moved_alert_get_storage_path(lt::storage_moved_alert* alert);
rust::Str storage_moved_alert_get_old_path(lt::storage_moved_alert* alert);

} // namespace ltrs
