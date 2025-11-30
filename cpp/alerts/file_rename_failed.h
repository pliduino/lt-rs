#pragma once

#include <libtorrent/alert_types.hpp>
#include "cpp/error.h"

namespace ltrs {
    Error file_rename_failed_alert_get_error(lt::file_rename_failed_alert* alert);
}
