#include "./unwanted_block.h"
#include <cstdint>

namespace ltrs {

int unwanted_block_alert_get_block_index(lt::unwanted_block_alert* alert) {
    return alert->block_index;
}

PieceIndex unwanted_block_alert_get_piece_index(lt::unwanted_block_alert* alert) {
    return PieceIndex { .inner = static_cast<int32_t>(alert->piece_index) };
}

} // namespace ltrs
