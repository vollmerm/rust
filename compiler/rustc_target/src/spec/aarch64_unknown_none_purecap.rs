use super::{LinkerFlavor, LldFlavor, PanicStrategy, RelocModel, Target, TargetOptions, MergeFunctions};

pub fn target() -> Target {
    let opts = TargetOptions {
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker: Some("rust-lld".to_owned()),
        features: "+strict-align,+neon,+morello,+c64".to_string(),
        //features: "+strict-align,+neon,+fp-armv8".to_string(),
        executables: true,
        relocation_model: RelocModel::Static,
        disable_redzone: true,
        max_atomic_width: Some(128),
        panic_strategy: PanicStrategy::Abort,
        llvm_abiname: "purecap".to_string(),
        abi: "purecap".to_string(),
        atomic_pointers_via_integers: false,
        merge_functions: MergeFunctions::Disabled,
        ..Default::default()
    };
    Target {
        llvm_target: "aarch64-unknown-none".to_string(),
        pointer_range: 64,
        pointer_width: 128,
        data_layout: "e-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-A200-P200-G200".to_string(),
        //data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        options: opts,
    }
}
