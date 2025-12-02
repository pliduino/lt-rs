#include "./save_resume_data.h"

namespace ltrs {
    lt::add_torrent_params *save_resume_data_alert_get_params(lt::save_resume_data_alert *alert) {
        return &alert->params;
    }
}
