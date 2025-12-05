#include "./peer_error.h"
#include "cpp/error.h"

namespace ltrs {

uint8_t peer_error_alert_get_op(lt::peer_error_alert* alert) {
    return static_cast<uint8_t>(alert->op);
}

Error peer_error_alert_get_error(lt::peer_error_alert* alert) {
    return error_code_to_error(alert->error);
}

} // namespace ltrs
