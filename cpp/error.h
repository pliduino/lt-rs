#pragma once

#include <libtorrent/error_code.hpp>

namespace ltrs {
    class Error;
    Error error_code_to_error(lt::error_code e);
}
