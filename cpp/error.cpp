#include "./error.h"

#include "lt-rs/src/ffi/error.rs.h"

#include <libtorrent/gzip.hpp>
#include <libtorrent/i2p_stream.hpp>
#include <libtorrent/socks5_stream.hpp>
#include <libtorrent/upnp.hpp>
#include <libtorrent/natpmp.hpp>

namespace ltrs {
    Error error_code_to_error(lt::error_code e) {
        if (e.category() == lt::gzip_category()) {
            return Error {
                ErrorCategory::GzipError,
                e.value(),
            };
        }
        else if (e.category() == lt::http_category()) {
            return Error {
                ErrorCategory::HttpError,
                e.value(),
            };
        }
        else if (e.category() == lt::i2p_category()) {
            return Error {
                ErrorCategory::I2pError,
                e.value(),
            };
        }
        else if (e.category() == lt::upnp_category()) {
            return Error {
                ErrorCategory::UpnpError,
                e.value(),
            };
        }
        else if (e.category() == lt::socks_category()) {
            return Error {
                ErrorCategory::SocksError,
                e.value(),
            };
        }
        else if (e.category() == lt::pcp_category()) {
            return Error {
                ErrorCategory::PcpError,
                e.value(),
            };
        }
        else if (e.category() == lt::bdecode_category()) {
            return Error {
                ErrorCategory::BdecodeError,
                e.value(),
            };
        }
        else if (e.category() == lt::libtorrent_category()) {
            return Error {
                ErrorCategory::LibtorrentError,
                e.value(),
            };
        }
        return Error {
            ErrorCategory::Unknown,
            e.value(),
        };
    }
}
