#pragma once
#include <cstdint>
#include <libtorrent/alert_types.hpp>
#include "rust/cxx.h"
#include "cpp/error.h"

namespace ltrs {
rust::Str scrape_failed_alert_get_error_message(lt::scrape_failed_alert* alert);
Error scrape_failed_alert_get_error(lt::scrape_failed_alert* alert);
uint8_t scrape_failed_alert_get_version(lt::scrape_failed_alert* alert);

} // namespace ltrs
