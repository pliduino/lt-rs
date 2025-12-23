#include "./invalid_request.h"

namespace ltrs {

PeerRequest invalid_request_alert_get_request(lt::invalid_request_alert* alert) {
    lt::peer_request req = alert->request;
    return PeerRequest {
        .piece = PieceIndex { .inner = static_cast<int>(req.piece) },
        .start = req.start,
        .length = req.length,
    };
}

bool invalid_request_alert_get_we_have(lt::invalid_request_alert* alert) {
    return alert->we_have;
}

bool invalid_request_alert_get_peer_interested(lt::invalid_request_alert* alert) {
    return alert->peer_interested;
}

bool invalid_request_alert_get_withheld(lt::invalid_request_alert* alert) {
    return alert->withheld;
}

} // namespace ltrs
