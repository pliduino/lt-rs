#pragma once
#include <libtorrent/alert_types.hpp>
#include "cpp/error.h"

namespace ltrs {
Error metadata_failed_alert_get_error(lt::metadata_failed_alert* alert);

} // namespace ltrs
