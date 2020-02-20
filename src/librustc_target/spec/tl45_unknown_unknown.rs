use crate::spec::{LldFlavor, LinkerFlavor, PanicStrategy, Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: "tl45-unknown-unknown".to_string(),
        target_endian: "big".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "E-m:e-p:32:32-i8:8-i16:16-i32:32-i64:32-a:0:32-n8:16:32-S32".to_string(),
        arch: "tl45".to_string(),
        target_os: "none".to_string(),
        target_env: String::new(),
        target_vendor: String::new(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),

        options: TargetOptions {
            executables: true,
            linker: Some("ld.lld".to_string()),
            cpu: String::new(),
            max_atomic_width: Some(0),
            atomic_cas: false, // incomplete +a extension
            panic_strategy: PanicStrategy::Abort,
            relocation_model: "static".to_string(),
            trap_unreachable: false,
            emit_debug_gdb_scripts: false,
            .. Default::default()
        },
    })
}
