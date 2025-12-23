#include "./piece_finished.h"
#include <cstdint>

namespace ltrs {

    PieceIndex piece_finished_alert_get_piece_index(lt::piece_finished_alert* alert) {
        return PieceIndex { .inner = static_cast<int32_t>(alert->piece_index) };
    }

} // namespace ltrs
