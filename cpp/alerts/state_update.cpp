#include "./state_update.h"

namespace ltrs {
    std::vector<lt::torrent_status> *state_update_alert_get_status(lt::state_update_alert *alert) {
        return &alert->status;
    }
}
