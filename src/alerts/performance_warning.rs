#[cfg_attr(feature = "safe_enums", derive(num_enum::FromPrimitive))]
#[repr(u8)]
pub enum PerformanceWarning {
    /// This warning means that the number of bytes queued to be written to disk
    /// exceeds the max disk byte queue setting (``settings_pack::max_queued_disk_bytes``).
    /// This might restrict the download rate, by not queuing up enough write jobs
    /// to the disk I/O thread. When this alert is posted, peer connections are
    /// temporarily stopped from downloading, until the queued disk bytes have fallen
    /// below the limit again. Unless your ``max_queued_disk_bytes`` setting is already
    /// high, you might want to increase it to get better performance.
    OutstandingDiskBufferLimitReached,

    /// This is posted when libtorrent would like to send more requests to a peer,
    /// but it's limited by ``settings_pack::max_out_request_queue``. The queue length
    /// libtorrent is trying to achieve is determined by the download rate and the
    /// assumed round-trip-time (``settings_pack::request_queue_time``). The assumed
    /// round-trip-time is not limited to just the network RTT, but also the remote disk
    /// access time and message handling time. It defaults to 3 seconds. The target number
    /// of outstanding requests is set to fill the bandwidth-delay product (assumed RTT
    /// times download rate divided by number of bytes per request). When this alert
    /// is posted, there is a risk that the number of outstanding requests is too low
    /// and limits the download rate. You might want to increase the ``max_out_request_queue``
    /// setting.
    OutstandingRequestLimitReached,

    /// This warning is posted when the amount of TCP/IP overhead is greater than the
    /// upload rate limit. When this happens, the TCP/IP overhead is caused by a much
    /// faster download rate, triggering TCP ACK packets. These packets eat into the
    /// rate limit specified to libtorrent. When the overhead traffic is greater than
    /// the rate limit, libtorrent will not be able to send any actual payload, such
    /// as piece requests. This means the download rate will suffer, and new requests
    /// can be sent again. There will be an equilibrium where the download rate, on
    /// average, is about 20 times the upload rate limit. If you want to maximize the
    /// download rate, increase the upload rate limit above 5% of your download capacity.
    UploadLimitTooLow,

    /// This is the same warning as ``upload_limit_too_low`` but referring to the download
    /// limit instead of upload. This suggests that your download rate limit is much lower
    /// than your upload capacity. Your upload rate will suffer. To maximize upload rate,
    /// make sure your download rate limit is above 5% of your upload capacity.
    DownloadLimitTooLow,

    /// We're stalled on the disk. We want to write to the socket, and we can write
    /// but our send buffer is empty, waiting to be refilled from the disk.
    /// This either means the disk is slower than the network connection
    /// or that our send buffer watermark is too small, because we can
    /// send it all before the disk gets back to us.
    /// The number of bytes that we keep outstanding, requested from the disk, is calculated
    /// as follows:
    ///
    /// .. code:: C++
    ///
    ///    min(512, max(upload_rate * send_buffer_watermark_factor / 100, send_buffer_watermark))
    ///
    /// If you receive this alert, you might want to either increase your ``send_buffer_watermark``
    /// or ``send_buffer_watermark_factor``.
    SendBufferWatermarkTooLow,

    /// If the half (or more) of all upload slots are set as optimistic unchoke slots, this
    /// warning is issued. You probably want more regular (rate based) unchoke slots.
    TooManyOptimisticUnchokeSlots,

    /// If the disk write queue ever grows larger than half of the cache size, this warning
    /// is posted. The disk write queue eats into the total disk cache and leaves very little
    /// left for the actual cache. This causes the disk cache to oscillate in evicting large
    /// portions of the cache before allowing peers to download any more, onto the disk write
    /// queue. Either lower ``max_queued_disk_bytes`` or increase ``cache_size``.
    TooHighDiskQueueLimit,

    AioLimitReached,

    #[deprecated]
    BittyrantWithNoUplimit,

    /// This is generated if outgoing peer connections are failing because of *address in use*
    /// errors, indicating that ``settings_pack::outgoing_ports`` is set and is too small of
    /// a range. Consider not using the ``outgoing_ports`` setting at all, or widen the range to
    /// include more ports.
    TooFewOutgoingPorts,

    TooFewFileDescriptors,

    #[cfg(feature = "safe_enums")]
    #[num_enum(default)]
    UnknownError,
}
