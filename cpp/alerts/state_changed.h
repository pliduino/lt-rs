#pragma once

#include <libtorrent/alert_types.hpp>

namespace ltrs {
    unsigned char state_changed_alert_get_state(lt::state_changed_alert *alert);
    unsigned char state_changed_alert_get_prev_state(lt::state_changed_alert *alert);
}
