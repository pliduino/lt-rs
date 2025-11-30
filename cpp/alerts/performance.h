#pragma once

#include <libtorrent/alert_types.hpp>

namespace ltrs {
    unsigned char performance_alert_get_warning_code(lt::performance_alert* alert);
}
