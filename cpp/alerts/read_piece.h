#pragma once

#include <libtorrent/alert_types.hpp>
#include "cpp/error.h"

namespace ltrs {
    int read_piece_alert_get_size(lt::read_piece_alert* a);
    Error read_piece_alert_get_error(lt::read_piece_alert* a);
}
