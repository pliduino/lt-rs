#pragma once

#include <libtorrent/alert_types.hpp>

#include "rust/cxx.h"

namespace ltrs {
    rust::Str tracker_alert_get_tracker_url(lt::tracker_alert *a);
}
