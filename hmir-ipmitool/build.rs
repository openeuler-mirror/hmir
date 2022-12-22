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
        .file("ipmitool/src/ipmitool.c")
        .file("ipmitool/lib/dimm_spd.c")
        .file("ipmitool/lib/helper.c")
        .file("ipmitool/lib/hpm2.c")
        .file("ipmitool/lib/ipmi_cfgp.c")
        .file("ipmitool/lib/ipmi_channel.c")
        .file("ipmitool/lib/ipmi_chassis.c")
        .file("ipmitool/lib/ipmi_dcmi.c")
        .file("ipmitool/lib/ipmi_delloem.c")
        .file("ipmitool/lib/ipmi_ekanalyzer.c")
        .file("ipmitool/lib/ipmi_event.c")
        .file("ipmitool/lib/ipmi_firewall.c")
        .file("ipmitool/lib/ipmi_fru.c")
        .file("ipmitool/lib/ipmi_fwum.c")
        .file("ipmitool/lib/ipmi_gendev.c")
        .file("ipmitool/lib/ipmi_hpmfwupg.c")
        .file("ipmitool/lib/ipmi_ime.c")
        .file("ipmitool/lib/ipmi_isol.c")
        .file("ipmitool/lib/ipmi_kontronoem.c")
        .file("ipmitool/lib/ipmi_lanp6.c")
        .file("ipmitool/lib/ipmi_lanp.c")
        .file("ipmitool/lib/ipmi_main.c")
        .file("ipmitool/lib/ipmi_mc.c")
        .file("ipmitool/lib/ipmi_oem.c")
        .file("ipmitool/lib/ipmi_pef.c")
        .file("ipmitool/lib/ipmi_picmg.c")
        .file("ipmitool/lib/ipmi_quantaoem.c")
        .file("ipmitool/lib/ipmi_raw.c")
        .file("ipmitool/lib/ipmi_sdradd.c")
        .file("ipmitool/lib/ipmi_sdr.c")
        .file("ipmitool/lib/ipmi_sel.c")
        .file("ipmitool/lib/ipmi_sensor.c")
        .file("ipmitool/lib/ipmi_session.c")
        // .file("ipmitool/lib/ipmi_sol.c")
        .file("ipmitool/lib/ipmi_strings.c")
        .file("ipmitool/lib/ipmi_sunoem.c")
        .file("ipmitool/lib/ipmi_time.c")
        // .file("ipmitool/lib/ipmi_tsol.c")
        .file("ipmitool/lib/ipmi_user.c")
        .file("ipmitool/lib/ipmi_vita.c")
        .file("ipmitool/lib/log.c")
        .file("ipmitool/src/plugins/ipmi_intf.c")
        .file("ipmitool/src/plugins/imb/imbapi.c")
        .file("ipmitool/src/plugins/imb/imb.c")
        .file("ipmitool/src/plugins/open/open.c")
        .file("ipmitool/src/plugins/serial/serial_basic.c")
        .file("ipmitool/src/plugins/serial/serial_terminal.c")
        // .file("ipmitool/src/plugins/lanplus/lanplus.c")
        // .file("ipmitool/src/plugins/lanplus/lanplus_crypt.c")
        // .file("ipmitool/src/plugins/lanplus/lanplus_crypt_impl.c")
        // .file("ipmitool/src/plugins/lanplus/lanplus_dump.c")
        // .file("ipmitool/src/plugins/lanplus/lanplus_strings.c")
        // .file("ipmitool/src/plugins/lan/lan.c")
        .include("ipmitool/lib/")
        .include("ipmitool/include/")
        .include("ipmitool/")
        .define("HAVE_CONFIG_H","1")
        .compile("ipmi");


    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ipmi");

    println!("cargo:rerun-if-changed=ipmitool/lib/ipmitool.c");

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
