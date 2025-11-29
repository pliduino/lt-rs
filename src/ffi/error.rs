#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {

    enum ErrorCategory {
        LibtorrentError,
        HttpError,
        GzipError,
        I2pError,
        PcpError,
        BdecodeError,
        SocksError,
        UpnpError,
        Unknown,
    }

    struct Error {
        category: ErrorCategory,
        code: i32,
    }

    unsafe extern "C++" {
        include!("cpp/error.h");
    }
}
