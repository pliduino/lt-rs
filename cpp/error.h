#pragma once

#include "lt-rs/src/ffi/error.rs.h"
#include <libtorrent/error_code.hpp>

namespace ltrs {
    Error error_code_to_error(lt::error_code e);
}
