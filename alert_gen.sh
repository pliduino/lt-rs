#!/usr/bin/env bash

set -e

if [ "$#" -lt 2 ]; then
    echo "Usage: $0 <alert_name> <fn1> [fn2 ... fnN]"
    exit 1
fi

alert_name="$1"
shift 1
fn_list=("$@")

outdir_cpp="./cpp/alerts"
outdir_rs="./src/ffi/alerts"
outdir_impl="./src/alerts/implementations"

mkdir -p "$outdir_cpp" "$outdir_rs" "$outdir_impl"

header_file="${outdir_cpp}/${alert_name}.h"
cpp_file="${outdir_cpp}/${alert_name}.cpp"
rs_file="${outdir_rs}/${alert_name}.rs"
impl_file="${outdir_impl}/${alert_name}.rs"

rust_type="${alert_name}_alert"
rust_struct_name="$(echo "$alert_name" | sed -E 's/(^|_)([a-z])/\U\2/g')Alert"


#
# ===== HEADER FILE =====
#
cat > "$header_file" <<EOF
#pragma once
#include <libtorrent/alert_types.hpp>

namespace ltrs {
EOF

for fn in "${fn_list[@]}"; do
    echo "void ${alert_name}_alert_${fn}(lt::${alert_name}_alert* alert);" >> "$header_file"
done

cat >> "$header_file" <<EOF

} // namespace ltrs
EOF


#
# ===== CPP FILE =====
#
cat > "$cpp_file" <<EOF
#include "./${alert_name}.h"

namespace ltrs {
EOF

for fn in "${fn_list[@]}"; do
cat >> "$cpp_file" <<EOF

void ${alert_name}_alert_${fn}(lt::${alert_name}_alert* alert) {
    // TODO: Implement
}
EOF
done

cat >> "$cpp_file" <<EOF

} // namespace ltrs
EOF


#
# ===== RUST FFI FILE =====
#
cat > "$rs_file" <<EOF
#[cxx::bridge(namespace = "ltrs")]
pub(crate) mod ffi {
    #[namespace = "libtorrent"]
    extern "C++" {
        type ${rust_type} = crate::ffi::ffi::${rust_type};
    }

    unsafe extern "C++" {
        include!("cpp/alerts/${alert_name}.h");
EOF

for fn in "${fn_list[@]}"; do
    echo "        unsafe fn ${rust_type}_${fn}(alert: *mut ${rust_type});" >> "$rs_file"
done

cat >> "$rs_file" <<EOF
    }
}
EOF


#
# ===== RUST IMPLEMENTATION FILE =====
#
camel_case_alert_struct="$rust_struct_name"

cat > "$impl_file" <<EOF
use crate::alerts::${camel_case_alert_struct};
use crate::ffi::alerts::${alert_name}::ffi::{
EOF

# Import each function
for fn in "${fn_list[@]}"; do
    echo "    ${rust_type}_${fn}," >> "$impl_file"
done

cat >> "$impl_file" <<EOF
};

impl ${camel_case_alert_struct} {
EOF

# Create impl methods
for fn in "${fn_list[@]}"; do
cat >> "$impl_file" <<EOF

    #[inline(always)]
    pub fn ${fn}(&self) {
        unsafe { ${rust_type}_${fn}(self.0) }
    }
EOF
done

cat >> "$impl_file" <<EOF
}
EOF


echo "Generated:"
echo "  $header_file"
echo "  $cpp_file"
echo "  $rs_file"
echo "  $impl_file"
