#include "./read_piece.h"

#include "cpp/error.h"

#include <libtorrent/gzip.hpp>
#include <libtorrent/error_code.hpp>
#include <libtorrent/i2p_stream.hpp>
#include <libtorrent/socks5_stream.hpp>
#include <libtorrent/upnp.hpp>
#include <libtorrent/natpmp.hpp>

namespace ltrs {
    int read_piece_alert_get_size(lt::read_piece_alert* a) {
        return a->size;
    }

    Error read_piece_alert_get_error(lt::read_piece_alert* a) {
        return error_code_to_error(a->error);
    }
}
