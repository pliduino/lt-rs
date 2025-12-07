#pragma once
#include <libtorrent/alert_types.hpp>
#include <cstdint>
#include "cpp/error.h"
#include "rust/cxx.h"

namespace ltrs {
rust::Str file_error_alert_get_filename(lt::file_error_alert* alert);
Error file_error_alert_get_error(lt::file_error_alert* alert);
uint8_t file_error_alert_get_op(lt::file_error_alert* alert);

} // namespace ltrs
