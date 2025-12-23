#pragma once
#include <libtorrent/alert_types.hpp>

namespace ltrs {
lt::add_torrent_params *save_resume_data_alert_get_params(lt::save_resume_data_alert* alert);

} // namespace ltrs
