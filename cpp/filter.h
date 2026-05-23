#pragma once
#include "rust/cxx.h"
#include <libtorrent/ip_filter.hpp>
#include <libtorrent/session.hpp>
#include <memory>

namespace ltrs {
    struct IpFilterWrapper { lt::ip_filter inner; };

    std::unique_ptr<IpFilterWrapper> lt_ip_filter_new();
    void     lt_ip_filter_add_rule(IpFilterWrapper &f, rust::Str start, rust::Str end, uint32_t flags);
    uint32_t lt_ip_filter_access(const IpFilterWrapper &f, rust::Str addr);
    void     lt_session_set_ip_filter(lt::session &s, const IpFilterWrapper &f);
}
