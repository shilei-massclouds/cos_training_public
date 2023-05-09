use std::io::Result;

fn main() {
    let feature = std::env::var("STEP_FEATURES").unwrap();
    gen_linker_script(&feature).unwrap();
}

fn gen_linker_script(feature: &str) -> Result<()> {
    let content = if feature == "a2_disable" {
        "CFG_BASE_ADDRESS = 0x80200000;\n"
    } else {
        "CFG_BASE_ADDRESS = 0xffffffc080200000;\n"
    };

    std::fs::write("linker_riscv64_generated.lds", content)?;
    Ok(())
}
