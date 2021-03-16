extern crate bindgen;

use std::env;
use std::path::PathBuf;

struct TizenContext {
    rootstrap_path: String,
}

impl TizenContext {
    fn path(&self, path: &str) -> String {
        format!("{}{}", self.rootstrap_path, path)
    }

    fn sys_clang_args(&self) -> Vec<String> {
        return vec![format!("--sysroot={}", self.path(""))];
    }

    fn lib_clang_args(&self, includes: &[String]) -> Vec<String> {
        return includes
            .iter()
            .map(|path| format!("-I{}", self.path(path)))
            .collect();
    }
}

struct TizenLibContext {
    header_path: String,
    link_lib: Vec<String>,
    includes: Vec<String>,
    whitelist_functions: Vec<String>,
    whitelist_vars: Vec<String>,
    whitelist_types: Vec<String>,
}

fn main() {
    println!("cargo:rerun-if-env-changed=TIZEN_ROOTSTRAP_PATH");

    let tizen_ctx = TizenContext {
        rootstrap_path: env::var("TIZEN_ROOTSTRAP_PATH")
            .expect("Configure the TIZEN_ROOTSTRAP_PATH env variable"),
    };

    let tizen_libs = vec![
        make_appfw_lib_context(),
        make_system_settings_lib_context(),
        make_dlog_lib_context(),
        make_elementary_lib_context(),
        make_evas_lib_context(),
    ];

    let mut builder = bindgen::Builder::default().clang_args(tizen_ctx.sys_clang_args());

    for tizen_lib in tizen_libs {
        for link_lib in tizen_lib.link_lib {
            println!("cargo:rustc-link-lib={}", link_lib);
        }

        builder = builder
            .clang_args(tizen_ctx.lib_clang_args(&tizen_lib.includes))
            .header(tizen_ctx.path(&tizen_lib.header_path));

        for whitelist_function in tizen_lib.whitelist_functions {
            builder = builder.whitelist_function(&whitelist_function);
        }

        for whitelist_type in tizen_lib.whitelist_types {
            builder = builder.whitelist_type(&whitelist_type);
        }

        for whitelist_var in tizen_lib.whitelist_vars {
            builder = builder.whitelist_var(&whitelist_var);
        }
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    builder
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("tizen_sys_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn make_appfw_lib_context() -> TizenLibContext {
    TizenLibContext {
        header_path: "/usr/include/appfw/app.h".to_string(),
        link_lib: vec!["capi-appfw-application".to_string()],
        includes: vec!["/usr/include/appfw".to_string()],
        whitelist_functions: vec!["(ui_)?app_.*".to_string()],
        whitelist_types: vec!["(ui_)?app_.*".to_string()],
        whitelist_vars: vec!["(UI_)?APP_.*".to_string()],
    }
}

fn make_system_settings_lib_context() -> TizenLibContext {
    TizenLibContext {
        header_path: "/usr/include/system/system_settings.h".to_string(),
        link_lib: vec!["capi-system-system-settings".to_string()],
        includes: vec![],
        whitelist_functions: vec!["system_.*".to_string()],
        whitelist_types: vec!["system_.*".to_string()],
        whitelist_vars: vec!["SYSTEM_.*".to_string()],
    }
}

fn make_dlog_lib_context() -> TizenLibContext {
    TizenLibContext {
        header_path: "/usr/include/dlog/dlog.h".to_string(),
        link_lib: vec!["dlog".to_string()],
        includes: vec![],
        whitelist_functions: vec!["dlog_.*".to_string()],
        whitelist_types: vec!["dlog_.*".to_string()],
        whitelist_vars: vec!["dlog_.*".to_string()],
    }
}

fn make_elementary_lib_context() -> TizenLibContext {
    TizenLibContext {
        header_path: "/usr/include/elementary-1/Elementary.h".to_string(),
        link_lib: vec!["elementary".to_string()],
        includes: vec![
            "/usr/include/efl-1".to_string(),
            "/usr/include/eina-1".to_string(),
            "/usr/include/eina-1/eina".to_string(),
            "/usr/include/eet-1".to_string(),
            "/usr/include/emile-1".to_string(),
            "/usr/include/evas-1".to_string(),
            "/usr/include/eo-1".to_string(),
            "/usr/include/ecore-1".to_string(),
            "/usr/include/ecore-evas-1".to_string(),
            "/usr/include/ecore-file-1".to_string(),
            "/usr/include/ecore-input-1".to_string(),
            "/usr/include/ecore-imf-1".to_string(),
            "/usr/include/ecore-con-1".to_string(),
            "/usr/include/edje-1".to_string(),
            "/usr/include/efreet-1".to_string(),
            "/usr/include/ethumb-client-1".to_string(),
            "/usr/include/ethumb-1".to_string(),
            "/usr/include/elementary-1".to_string(),
        ],
        whitelist_functions: vec!["elm_.*".to_string()],
        whitelist_types: vec!["Elm_.*".to_string()],
        whitelist_vars: vec!["ELM_.*".to_string()],
    }
}

fn make_evas_lib_context() -> TizenLibContext {
    TizenLibContext {
        header_path: "/usr/include/evas-1/Evas.h".to_string(),
        link_lib: vec!["evas".to_string()],
        includes: vec![
            "/usr/include/evas-1".to_string(),
            "/usr/include/efl-1".to_string(),
            "/usr/include/eina-1".to_string(),
            "/usr/include/eina-1/eina".to_string(),
            "/usr/include/eo-1".to_string(),
            "/usr/include/emile-1".to_string(),
        ],
        whitelist_functions: vec!["evas_.*".to_string()],
        whitelist_types: vec!["Evas.*".to_string()],
        whitelist_vars: vec!["EVAS_.*".to_string()],
    }
}
