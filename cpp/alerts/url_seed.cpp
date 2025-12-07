#include "./url_seed.h"
#include "cpp/error.h"

namespace ltrs {

rust::Str url_seed_alert_get_server_url(lt::url_seed_alert* alert) {
    return alert->server_url();
}

rust::Str url_seed_alert_get_error_message(lt::url_seed_alert* alert) {
    return alert->error_message();
}

Error url_seed_alert_get_error(lt::url_seed_alert* alert) {
    return error_code_to_error(alert->error);
}

} // namespace ltrs
