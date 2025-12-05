#include "./peer_alert.h"

#include <memory>

namespace ltrs {

std::array<unsigned char, 20> peer_alert_get_pid(lt::peer_alert* alert) {
    std::array<unsigned char, 20> copied;
    std::memcpy(copied.data(), alert->pid.data(), copied.size());
    return copied;
}

} // namespace ltrs
