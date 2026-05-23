#pragma once

#include "rust/cxx.h"

#include <libtorrent/alert_types.hpp>

namespace ltrs {
    rust::Str file_renamed_alert_get_old_name(lt::file_renamed_alert* alert);
    rust::Str file_renamed_alert_get_new_name(lt::file_renamed_alert* alert);
    int32_t   file_renamed_alert_get_index(lt::file_renamed_alert* alert);
}
