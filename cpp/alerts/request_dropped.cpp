#include "./request_dropped.h"

namespace ltrs {

int request_dropped_alert_get_block_index(lt::request_dropped_alert* alert) {
    return alert->block_index;
}

PieceIndex request_dropped_alert_get_piece_index(lt::request_dropped_alert* alert) {
    return PieceIndex { .inner = static_cast<int32_t>(alert->piece_index) };
}

} // namespace ltrs
