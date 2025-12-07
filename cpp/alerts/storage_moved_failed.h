#pragma once
#include "rust/cxx.h"
#include <cstdint>
#include <libtorrent/alert_types.hpp>
#include "cpp/error.h"

namespace ltrs {
rust::Str storage_moved_failed_alert_file_path(lt::storage_moved_failed_alert* alert);
Error storage_moved_failed_alert_get_error(lt::storage_moved_failed_alert* alert);
uint8_t storage_moved_failed_alert_get_op(lt::storage_moved_failed_alert* alert);

} // namespace ltrs
