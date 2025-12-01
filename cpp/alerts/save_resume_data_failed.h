#pragma once

#include <libtorrent/alert_types.hpp>
#include "cpp/error.h"

namespace ltrs {
    Error save_resume_data_failed_alert_get_error(lt::save_resume_data_failed_alert *alert);
}
