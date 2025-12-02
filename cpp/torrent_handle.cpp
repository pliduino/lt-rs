#include "./torrent_handle.h"

#include "lt-rs/src/ffi/mod.rs.h"

namespace ltrs {
    bool torrent_handle_in_session(const lt::torrent_handle &handle) {
      return handle.in_session();
    }

    InfoHashCpp torrent_handle_info_hashes(const lt::torrent_handle &handle) {
      const auto hash = handle.info_hashes();
      return info_hash_t_to_info_hash_cpp(hash);
    }

    std::unique_ptr<lt::torrent_status>
    torrent_handle_status(const lt::torrent_handle &handle) {
      return std::make_unique<lt::torrent_status>(handle.status());
    }

    void torrent_handle_save_resume_data(const lt::torrent_handle &handle,
                                            uint8_t flags) {
      handle.save_resume_data((lt::resume_data_flags_t)flags);
    }

    void torrent_handle_read_piece(const lt::torrent_handle &handle, int piece) {
      // TODO
       // handle.read_piece((lt::piece_index_t)piece);

       // Just so the compiler shuts up for now
       (void)piece;
       (void)handle;
    }
}
