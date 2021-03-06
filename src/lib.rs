#[macro_use]
mod obsmodule;

obs_declare_module!("obs-module-rust", "Rust OBS module example");
obs_module_author!("Stéphane Lepin");

#[no_mangle]
pub extern fn obs_module_load() -> bool
{
    println!("Rust module loaded!");
    true
}

#[no_mangle]
pub extern fn obs_module_unload() -> ()
{
    println!("Rust module unloaded!");
}