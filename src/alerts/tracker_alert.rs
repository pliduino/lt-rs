use crate::alerts::{
    DhtReplyAlert, HashFailedAlert, ScrapeFailedAlert, ScrapeReplyAlert, TrackerAnnounceAlert,
    TrackerErrorAlert, TrackerReplyAlert, TrackerWarningAlert, TrackeridAlert,
};

pub enum TrackerAlert {
    /// This alert is generated on tracker time outs, premature disconnects,
    /// invalid response or a HTTP response other than "200 OK". From the alert
    /// you can get the handle to the torrent the tracker belongs to.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Tracker`] [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::High`]
    TrackerError(TrackerErrorAlert),
    /// This alert is triggered if the tracker reply contains a warning field.
    /// Usually this means that the tracker announce was successful, but the
    /// tracker has a message to the client.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Tracker`] [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    TrackerWarning(TrackerWarningAlert),
    /// This alert is generated when a scrape request succeeds.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Tracker`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    ScrapeReply(ScrapeReplyAlert),
    /// If a scrape request fails, this alert is generated. This might be due
    /// to the tracker timing out, refusing connection or returning an http response
    /// code indicating an error.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Tracker`] [`AlertCategory::Error`]
    /// ## Alert Priority
    /// [`AlertPriority::Critical`]
    ScrapeFailed(ScrapeFailedAlert),
    /// This alert is only for informational purpose. It is generated when a tracker announce
    /// succeeds. It is generated regardless what kind of tracker was used, be it UDP, HTTP or
    /// the DHT.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Tracker`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    TrackerReply(TrackerReplyAlert),
    /// This alert is generated each time the DHT receives peers from a node. ``num_peers``
    /// is the number of peers we received in this packet. Typically these packets are
    /// received from multiple DHT nodes, and so the alerts are typically generated
    /// a few at a time.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Dht`] [`AlertCategory::Tracker`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    DhtReply(DhtReplyAlert),
    /// This alert is generated each time a tracker announce is sent (or attempted to be sent).
    /// There are no extra data members in this alert. The url can be found in the base class
    /// however.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Tracker`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    TrackerAnnounce(TrackerAnnounceAlert),
    /// This alert is posted whenever a tracker responds with a ``trackerid``.
    /// The tracker ID is like a cookie. libtorrent will store the tracker ID
    /// for this tracker and repeat it in subsequent announces.
    ///
    /// ## Alert Category
    /// [`AlertCategory::Status`]
    /// ## Alert Priority
    /// [`AlertPriority::Normal`]
    Trackerid(TrackeridAlert),
}
