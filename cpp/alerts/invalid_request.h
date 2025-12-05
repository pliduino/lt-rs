#pragma once
#include <libtorrent/alert_types.hpp>

#include "lt-rs/src/ffi/mod.rs.h"

namespace ltrs {
    PeerRequest invalid_request_alert_get_request(lt::invalid_request_alert* alert);
    bool invalid_request_alert_get_we_have(lt::invalid_request_alert* alert);
    bool invalid_request_alert_get_peer_interested(lt::invalid_request_alert* alert);
    bool invalid_request_alert_get_withheld(lt::invalid_request_alert* alert);

} // namespace ltrs
