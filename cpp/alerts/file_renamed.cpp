#include "./file_renamed.h"


namespace ltrs {
    rust::Str file_renamed_alert_get_old_name(lt::file_renamed_alert* alert) {
        return alert->old_name();
    }

    rust::Str file_renamed_alert_get_new_name(lt::file_renamed_alert* alert) {
        return alert->new_name();
    }

    int32_t file_renamed_alert_get_index(lt::file_renamed_alert* alert) {
        return static_cast<int32_t>(alert->index);
    }
}
