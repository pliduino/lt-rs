use cxx::UniquePtr;

use crate::{alerts::AlertCategory, ffi::ffi};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProxyKind {
    Socks5,
    Http,
}

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

    pub fn set_listen_interfaces(&mut self, value: impl AsRef<str>) {
        ffi::lt_set_listen_interfaces(self.inner.pin_mut(), value.as_ref());
    }

    pub fn set_outgoing_interfaces(&mut self, value: impl AsRef<str>) {
        ffi::lt_set_outgoing_interfaces(self.inner.pin_mut(), value.as_ref());
    }

    pub fn set_proxy(
        &mut self,
        kind: ProxyKind,
        hostname: impl AsRef<str>,
        port: u16,
        username: Option<&str>,
        password: Option<&str>,
    ) {
        let username = username.unwrap_or("");
        let password = password.unwrap_or("");
        let authenticated = !username.is_empty() || !password.is_empty();
        let proxy_kind = match kind {
            ProxyKind::Socks5 => 0,
            ProxyKind::Http => 1,
        };
        ffi::lt_set_proxy_settings(
            self.inner.pin_mut(),
            proxy_kind,
            hostname.as_ref(),
            port,
            username,
            password,
            authenticated,
        );
    }

    // ─── Bandwidth ────────────────────────────────────────────────────────────

    /// Global upload cap in bytes/sec. 0 = unlimited.
    pub fn set_upload_rate_limit(&mut self, bytes_per_sec: i32) {
        ffi::lt_settings_set_upload_rate_limit(self.inner.pin_mut(), bytes_per_sec);
    }
    /// Global download cap in bytes/sec. 0 = unlimited.
    pub fn set_download_rate_limit(&mut self, bytes_per_sec: i32) {
        ffi::lt_settings_set_download_rate_limit(self.inner.pin_mut(), bytes_per_sec);
    }

    // ─── Connections ──────────────────────────────────────────────────────────

    /// Maximum number of peer connections across all torrents.
    pub fn set_connections_limit(&mut self, n: i32) {
        ffi::lt_settings_set_connections_limit(self.inner.pin_mut(), n);
    }

    // ─── Queue limits ─────────────────────────────────────────────────────────

    /// Max simultaneously active (downloading) torrents. -1 = unlimited.
    pub fn set_active_downloads(&mut self, n: i32) {
        ffi::lt_settings_set_active_downloads(self.inner.pin_mut(), n);
    }
    /// Max simultaneously active (seeding) torrents. -1 = unlimited.
    pub fn set_active_seeds(&mut self, n: i32) {
        ffi::lt_settings_set_active_seeds(self.inner.pin_mut(), n);
    }
    /// Total active torrent limit (downloads + seeds). -1 = unlimited.
    pub fn set_active_limit(&mut self, n: i32) {
        ffi::lt_settings_set_active_limit(self.inner.pin_mut(), n);
    }

    // ─── Seeding policy ───────────────────────────────────────────────────────

    /// Stop seeding when ratio (upload/download) exceeds `n / 100`.
    /// E.g. 200 = stop at 2.0 ratio.
    pub fn set_seed_time_ratio_limit(&mut self, n: i32) {
        ffi::lt_settings_set_seed_time_ratio_limit(self.inner.pin_mut(), n);
    }
    /// Stop seeding after this many seconds. 0 = never.
    pub fn set_seed_time_limit(&mut self, secs: i32) {
        ffi::lt_settings_set_seed_time_limit(self.inner.pin_mut(), secs);
    }
    /// Stop seeding when upload/download ratio reaches this value.
    pub fn set_share_ratio_limit(&mut self, ratio: f32) {
        ffi::lt_settings_set_share_ratio_limit(self.inner.pin_mut(), ratio);
    }

    // ─── Tracker health ───────────────────────────────────────────────────────

    /// Trackers with this many consecutive failures are considered dead.
    pub fn set_max_failcount(&mut self, n: i32) {
        ffi::lt_settings_set_max_failcount(self.inner.pin_mut(), n);
    }

    // ─── Protocol features ────────────────────────────────────────────────────

    pub fn set_dht_enabled(&mut self, enabled: bool) {
        ffi::lt_settings_set_dht_enabled(self.inner.pin_mut(), enabled);
    }
    pub fn set_lsd_enabled(&mut self, enabled: bool) {
        ffi::lt_settings_set_lsd_enabled(self.inner.pin_mut(), enabled);
    }
    pub fn set_upnp_enabled(&mut self, enabled: bool) {
        ffi::lt_settings_set_upnp_enabled(self.inner.pin_mut(), enabled);
    }
    pub fn set_natpmp_enabled(&mut self, enabled: bool) {
        ffi::lt_settings_set_natpmp_enabled(self.inner.pin_mut(), enabled);
    }
}

impl From<UniquePtr<ffi::settings_pack>> for SettingsPack {
    fn from(inner: UniquePtr<ffi::settings_pack>) -> SettingsPack {
        SettingsPack { inner }
    }
}

// TODO: Check if this is safe
unsafe impl Send for SettingsPack {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_pack_accepts_network_binding_settings() {
        let mut settings = SettingsPack::new();
        settings.set_listen_interfaces("10.8.0.2:6881");
        settings.set_outgoing_interfaces("wg0");
        settings.set_proxy(
            ProxyKind::Socks5,
            "proxy.example.test",
            1080,
            Some("user"),
            Some("pass"),
        );
    }
}
