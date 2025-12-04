#pragma once

#include <libtorrent/alert_types.hpp>
#include <cstdint>

namespace ltrs{
    uint8_t scrape_reply_alert_get_version(lt::scrape_reply_alert *a);
    uint32_t scrape_reply_alert_get_incomplete(lt::scrape_reply_alert *a);
    uint32_t scrape_reply_alert_get_complete(lt::scrape_reply_alert *a);
}
