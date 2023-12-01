use crate::spec::base::apple::{opts, Arch};
use crate::spec::{StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let base = opts("watchos", Arch::Arm64_32);
    Target {
        llvm_target: "arm64_32-apple-watchos".into(),
        pointer_width: 32,
        data_layout: "e-m:o-p:32:32-i64:64-i128:128-n32:64-S128".into(),
        arch: "aarch64".into(),
        options: TargetOptions {
            features: "+v8a,+neon,+fp-armv8,+apple-a7".into(),
            max_atomic_width: Some(128),
            stack_probes: StackProbeType::Inline,
            dynamic_linking: false,
            position_independent_executables: true,
            ..base
        },
    }
}
