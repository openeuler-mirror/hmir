extern crate cc;

use std::env;
use std::path::PathBuf;

fn build_lib() {


    cc::Build::new()
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-variable")
        .flag("-Wno-pointer-sign")
        .flag("-Wno-implicit-fallthrough")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-tautological-compare")
        .flag("-Wno-type-limits")
        .flag("-Wno-unused-function")
        .flag("-Wno-format-extra-args")
        .flag("-Wno-format")
        .file("src/ipmi/ipmitool.c")
        .file("src/ipmi/dimm_spd.c")
        .file("src/ipmi/helper.c")
        .file("src/ipmi/hpm2.c")
        .file("src/ipmi/ipmi_cfgp.c")
        .file("src/ipmi/ipmi_channel.c")
        .file("src/ipmi/ipmi_chassis.c")
        .file("src/ipmi/ipmi_dcmi.c")
        .file("src/ipmi/ipmi_delloem.c")
        .file("src/ipmi/ipmi_ekanalyzer.c")
        .file("src/ipmi/ipmi_event.c")
        .file("src/ipmi/ipmi_firewall.c")
        .file("src/ipmi/ipmi_fru.c")
        .file("src/ipmi/ipmi_fwum.c")
        .file("src/ipmi/ipmi_gendev.c")
        .file("src/ipmi/ipmi_hpmfwupg.c")
        .file("src/ipmi/ipmi_ime.c")
        .file("src/ipmi/ipmi_isol.c")
        .file("src/ipmi/ipmi_kontronoem.c")
        .file("src/ipmi/ipmi_lanp6.c")
        .file("src/ipmi/ipmi_lanp.c")
        .file("src/ipmi/ipmi_main.c")
        .file("src/ipmi/ipmi_mc.c")
        .file("src/ipmi/ipmi_oem.c")
        .file("src/ipmi/ipmi_pef.c")
        .file("src/ipmi/ipmi_picmg.c")
        .file("src/ipmi/ipmi_quantaoem.c")
        .file("src/ipmi/ipmi_raw.c")
        .file("src/ipmi/ipmi_sdradd.c")
        .file("src/ipmi/ipmi_sdr.c")
        .file("src/ipmi/ipmi_sel.c")
        .file("src/ipmi/ipmi_sensor.c")
        .file("src/ipmi/ipmi_session.c")
        // .file("src/ipmi/ipmi_sol.c")
        .file("src/ipmi/ipmi_strings.c")
        .file("src/ipmi/ipmi_sunoem.c")
        .file("src/ipmi/ipmi_time.c")
        // .file("src/ipmi/ipmi_tsol.c")
        .file("src/ipmi/ipmi_user.c")
        .file("src/ipmi/ipmi_vita.c")
        .file("src/ipmi/log.c")
        .file("src/ipmi/ipmi_intf.c")
        .file("src/ipmi/imbapi.c")
        .file("src/ipmi/imb.c")
        .file("src/ipmi/open.c")
        // .file("src/ipmi/lan.c")
        // .file("src/ipmi/lanplus.c")
        // .file("src/ipmi/lanplus_crypt.c")
        // .file("src/ipmi/lanplus_crypt_impl.c")
        // .file("src/ipmi/lanplus_strings.c")
        // .file("src/ipmi/free/free.c")
        .include("src/ipmi/")
        .define("HAVE_CONFIG_H","1")
        .compile("ipmi");


    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ipmi");

    println!("cargo:rerun-if-changed=src/ipmi/ipmitool.c");

}




fn gen_bind() {
    system_deps::Config::new().probe().unwrap();

    // Invalidate the built crate whenever the wrapper and the build script changes.
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("ipmi.*")
        .allowlist_function("picmg.*")
        .allowlist_function("vita_.*")
        .allowlist_function("sdr_*")
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}


fn main()
{
    build_lib();
    //gen_bind();
}
