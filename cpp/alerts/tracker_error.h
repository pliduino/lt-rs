#pragma once

#include "rust/cxx.h"

#include <libtorrent/alert_types.hpp>
#include <cpp/error.h>
#include <cstdint>

namespace ltrs{
    rust::Str tracker_error_alert_get_failure_reason(lt::tracker_error_alert *a);
    int tracker_error_alert_get_times_in_row(lt::tracker_error_alert *a);
    Error tracker_error_alert_get_error(lt::tracker_error_alert *a);
    uint8_t tracker_error_alert_get_op(lt::tracker_error_alert *a);
    uint8_t tracker_error_alert_get_version(lt::tracker_error_alert *a);
}
