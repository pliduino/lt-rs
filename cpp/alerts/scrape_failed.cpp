#include "./scrape_failed.h"
#include "cpp/error.h"
#include "rust/cxx.h"

namespace ltrs {

rust::Str scrape_failed_alert_get_error_message(lt::scrape_failed_alert* alert) {
    return alert->error_message();
}

Error scrape_failed_alert_get_error(lt::scrape_failed_alert* alert) {
    return error_code_to_error(alert->error);
}

uint8_t scrape_failed_alert_get_version(lt::scrape_failed_alert* alert) {
    return static_cast<uint8_t>(alert->version);
}

} // namespace ltrs
