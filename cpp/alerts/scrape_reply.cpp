#include "./scrape_reply.h"

namespace ltrs {
    uint8_t scrape_reply_alert_get_version(lt::scrape_reply_alert *a) { return static_cast<uint8_t>(a->version); }
    int scrape_reply_alert_get_incomplete(lt::scrape_reply_alert *a) { return a->incomplete; }
    int scrape_reply_alert_get_complete(lt::scrape_reply_alert *a) { return a->complete; }
}
