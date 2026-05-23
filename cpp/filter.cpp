#include "./filter.h"
#include <boost/asio/ip/address.hpp>

namespace ltrs {
    std::unique_ptr<IpFilterWrapper> lt_ip_filter_new() {
        return std::make_unique<IpFilterWrapper>();
    }

    void lt_ip_filter_add_rule(IpFilterWrapper &f, rust::Str start, rust::Str end, uint32_t flags) {
        auto s = boost::asio::ip::make_address(std::string(start));
        auto e = boost::asio::ip::make_address(std::string(end));
        f.inner.add_rule(s, e, static_cast<int>(flags));
    }

    uint32_t lt_ip_filter_access(const IpFilterWrapper &f, rust::Str addr) {
        auto a = boost::asio::ip::make_address(std::string(addr));
        return static_cast<uint32_t>(f.inner.access(a));
    }

    void lt_session_set_ip_filter(lt::session &s, const IpFilterWrapper &f) {
        s.set_ip_filter(f.inner);
    }

    std::unique_ptr<IpFilterWrapper> lt_session_get_ip_filter(lt::session &s) {
        auto w = std::make_unique<IpFilterWrapper>();
        w->inner = s.get_ip_filter();
        return w;
    }
}
