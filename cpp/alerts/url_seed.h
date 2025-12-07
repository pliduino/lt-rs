#pragma once
#include "rust/cxx.h"
#include <libtorrent/alert_types.hpp>
#include "cpp/error.h"

namespace ltrs {
rust::Str url_seed_alert_get_server_url(lt::url_seed_alert* alert);
rust::Str url_seed_alert_get_error_message(lt::url_seed_alert* alert);
Error url_seed_alert_get_error(lt::url_seed_alert* alert);

} // namespace ltrs
