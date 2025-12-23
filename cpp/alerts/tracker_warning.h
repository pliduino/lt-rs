#pragma once

#include <libtorrent/alert_types.hpp>
#include "rust/cxx.h"

namespace ltrs {
    rust::Str tracker_warning_alert_get_warning_message(lt::tracker_warning_alert *a);

    uint8_t tracker_warning_alert_get_version(lt::tracker_warning_alert *a);
}
