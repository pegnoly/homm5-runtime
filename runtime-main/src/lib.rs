use std::{io::{stdin, Read}, path::PathBuf, sync::LazyLock};

use frida::{Device, DeviceManager, Frida, Message, Process, Script, ScriptHandler, ScriptOption, SpawnOptions};

static FRIDA: LazyLock<Frida> = LazyLock::new(|| unsafe { Frida::obtain() });
const EXE_NAME: &'static str = "H5_Game_MCCS_PEST_SKILLS.exe";

struct Homm5Handler;

impl ScriptHandler for Homm5Handler {
    fn on_message(&mut self, message: &Message, data: Option<Vec<u8>>) {
        println!("Message: {:?}", message)
    }
}

pub struct RuntimeRunner {
    game_bin_path: PathBuf,
    process_id: i32 
}

impl RuntimeRunner {
    pub fn new(path: PathBuf) -> Self {
        RuntimeRunner {
            game_bin_path: path,
            process_id: -1
        }
    }

    pub fn run(&mut self) {
        std::env::set_current_dir(&self.game_bin_path).unwrap();
        let device_manager = frida::DeviceManager::obtain(&FRIDA);
        let mut local_device = device_manager.get_device_by_type(frida::DeviceType::Local).unwrap();
        match local_device.resume(self.process_id as u32) {
            Ok(()) => {
                println!("Process already exists");
            },
            Err(err) => {

                kill_helpers(&mut local_device);

                let pid = local_device.spawn(
                    format!("{}{}", &self.game_bin_path.to_str().unwrap(), EXE_NAME), 
                    &SpawnOptions::default()
                ).unwrap();

                self.process_id = pid as i32;
            
                let session = local_device.attach(pid);
                match session {
                    Ok(ref session) => {
                        let script_string = 
                        r#"
                            var mod = Process.enumerateModules()[0];
                            var mthid = Process.enumerateThreads()[0].id;
        
                            console.log('attaching CastCombatSpell');
                            Interceptor.attach(ptr(0x4E0C20), {
                                onEnter: function(args) {
                                    var spell_ctx = this.context.ecx;
                                    var spell_id = spell_ctx.add(16).readU32();
                                    var attacker = spell_ctx.add(32).readPointer();
                                    var target = spell_ctx.add(36).readPointer();
                                    console.log('CastCombatSpell: ' + spell_id + ' ' + attacker + ' ' + target);
        
                                    var combat = ptr(0xFE719C).readPointer().add(4).readPointer().add(8).readPointer().sub(1380);  
                                    console.log(combat);
                                    var arena_names = combat.add(672).readPointer().add(320).readPointer();
                                    console.log(arena_names);
                                    var get_ptr = arena_names.readPointer().add(8).readPointer();
                                    console.log(get_ptr);
                                    var get_thing = new NativeFunction(get_ptr, 'pointer', ['pointer', 'pointer'], 'thiscall');
                                    var nametag = get_thing(arena_names, attacker).readPointer().readUtf8String();
                                    var target_tag = get_thing(arena_names, target).readPointer().readUtf8String();
                                    console.log(nametag);
        
        
                                    var dostring_script = new NativeFunction(ptr(0x5526C0), 'bool', ['pointer', 'pointer'], 'thiscall');
                                    var script = Memory.allocUtf8String(
                                        'errorHandler = function() Error(0) end;errorHook(errorHandler);Callback(0, OnSpellCast(\"' + nametag + '#' + spell_id + '#' + target_tag + '\"));');
                                    //var script = Memory.allocUtf8String('OnSpellCast(' + nametag + '-' + spell_id + ');');
        
                                    
                                    console.log('exec callback ' + combat.add(436));
                                    console.log('result: ' + dostring_script(combat.add(436), script));      
                                },
        
                                onLeave: function(args) {
                                    Stalker.unfollow(this.threadId);
                                    console.log('result: ' + this.context.eax);
                                }
                            });
                        "#;
                        let mut script = session.create_script(script_string, &mut ScriptOption::default()).unwrap();
                        script.load().unwrap();
                        script.handle_message(Homm5Handler{}).unwrap();
                        local_device.resume(pid).unwrap();
        
                        while let Some(process) = local_device.enumerate_processes().iter().find(|p| p.get_pid() == pid) {
                            println!("Process exists: {}", process.get_name());
                            let mut v = vec![];
                            stdin().read_exact(&mut v).unwrap();
                        }
                        println!("Process doesn't exist anymore");
                        script.unload().unwrap();
                        session.detach().unwrap();
                    },
                    Err(e) => {println!("Error: {}", e)}
                }
            }
        } 
        // loop {
        //     let mut v = vec![];
        //     stdin().read(&mut v).unwrap();
    
        //     // if let None = local_device.enumerate_processes().iter().find(|p| p.get_name() == "H5_Game_MCCS_PEST_SKILLS.exe") {
        //     //     println!("Finished?");
        //     //     break;
        //     // }
        // }
    }
}


fn kill_helpers(device: &mut Device<'_>) {
    let processes = device.enumerate_processes();
    let helpers = processes.iter().filter_map(|p| if p.get_name().contains("frida-helper") { Some(p.get_pid()) } else { None }).collect::<Vec<u32>>();
    process_deletion(helpers);
}

fn process_deletion(processes: Vec<u32>) {
    let device_manager = frida::DeviceManager::obtain(&FRIDA);
    let mut local_device = device_manager.get_device_by_type(frida::DeviceType::Local).unwrap();
    for p in processes {
        local_device.kill(p).unwrap();
    }
}
