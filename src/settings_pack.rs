use cxx::UniquePtr;

use crate::{alerts::AlertCategory, ffi::ffi};

pub struct SettingsPack {
    inner: UniquePtr<ffi::settings_pack>,
}

impl SettingsPack {
    pub fn new() -> SettingsPack {
        SettingsPack {
            inner: ffi::lt_create_settings_pack(),
        }
    }

    pub(crate) fn inner(&self) -> &ffi::settings_pack {
        &self.inner
    }

    pub fn set_alert_mask(&mut self, mask: AlertCategory) {
        ffi::lt_set_alert_mask(self.inner.pin_mut(), mask.bits());
    }
}

impl From<UniquePtr<ffi::settings_pack>> for SettingsPack {
    fn from(inner: UniquePtr<ffi::settings_pack>) -> SettingsPack {
        SettingsPack { inner }
    }
}

// TODO: Check if this is safe
unsafe impl Send for SettingsPack {}
