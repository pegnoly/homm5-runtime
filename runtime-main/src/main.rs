use std::{io::{stdin, Read}, sync::LazyLock};

use frida::{Device, DeviceManager, Frida, Script, ScriptHandler, ScriptOption, SpawnOptions};

static FRIDA: LazyLock<Frida> = LazyLock::new(|| unsafe { Frida::obtain() });

struct Homm5Handler;

impl ScriptHandler for Homm5Handler {
    fn on_message(&mut self, message: &str) {
        println!("Message: {}", message)
    }
}

fn main() {
    let path = std::env::current_exe().unwrap().ancestors()
        .find(|a| a.ends_with("gog_runtime"))
        .unwrap()
        .parent()
        .unwrap()
        .join("bin");
    std::env::set_current_dir(&path).unwrap();
    let device_manager = frida::DeviceManager::obtain(&FRIDA);
    let mut local_device = device_manager.get_device_by_type(frida::DeviceType::Local).unwrap();
    let pid = local_device.spawn(
        format!("{}\\H5_Game_MCCS_PEST_SKILLS.exe", path.to_str().unwrap()), 
        &SpawnOptions::default()
    ).unwrap();

    let session = local_device.attach(pid);
    match session {
        Ok(ref session) => {
            let script_string = std::fs::read_to_string("D:\\Homm5GOG\\gog_runtime\\src\\frida.js").unwrap();
            let script = session.create_script(&script_string, &mut ScriptOption::default()).unwrap();
            script.load().unwrap();
            script.handle_message(&mut Homm5Handler{}).unwrap();
            local_device.resume(pid).unwrap();
        },
        Err(e) => {println!("Error: {}", e.to_string())}
    }

    loop {
        let mut v = vec![];
        stdin().read(&mut v).unwrap();

        // if let None = local_device.enumerate_processes().iter().find(|p| p.get_name() == "H5_Game_MCCS_PEST_SKILLS.exe") {
        //     println!("Finished?");
        //     break;
        // }
    }
    // let processes = local_device.enumerate_processes();
    // for process in &processes {
    //     println!("Process name: {}", process.get_name());
    // }
    // if let Some(process) = processes.iter().find(|p| p.get_name() == "H5_Game_MCCS_PEST_SKILLS.exe") {
    //     let pid = process.get_pid();
    //     let session = local_device.attach(pid).unwrap();
    //     println!("Attached");
    // }
    // println!("Pid: {}", pid);
    // match session {
    //     Ok(session) => {

    //     },
    //     Err(e) => {
    //         println!("Error: {}", e.to_string())
    //     }
    // }

    //execute_parser(path);
}
