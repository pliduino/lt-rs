#include "./state_changed.h"

namespace ltrs {
    unsigned char state_changed_alert_get_state(lt::state_changed_alert *alert) {
        return alert->state;
    }
    unsigned char state_changed_alert_get_prev_state(lt::state_changed_alert *alert) {
        return alert->prev_state;
    }
}
