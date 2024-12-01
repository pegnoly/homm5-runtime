var mod = Process.enumerateModules()[0];
var mthid = Process.enumerateThreads()[0].id;

// var find_setuivar = Memory.scanSync(mod.base, mod.size, "E8 ?? ?? ?? ?? EB 30 7D 0F");
// if (find_setuivar.length > 0) {
//     console.log('found set_ui_var' + JSON.stringify(find_setuivar));
//     Interceptor.attach(find_setuivar[0]['address'].add(5).add(find_setuivar[0]['address'].add(1).readU32()), {
//         onEnter: function(args) {
//             //console.log(JSON.stringify(this.context));
//             var ui_elem = this.context.ecx;
//             var variable = args[0].readPointer().readUtf8String();
//             var value = args[1].readPointer().readUtf16String();
//             console.log('set_ui_var: ' + ui_elem + ' ' + variable + ' = ' + value);
//             var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//             console.log(handler);
//         },

//         onLeave: function(args) {
//             //Stalker.unfollow(this.threadId);
//             //console.log('result: ' + this.context.eax);
//         }
//     });
// } else { console.log("failed to find set_ui_var"); }


// console.log('attaching creaturecombat');
// Interceptor.attach(ptr(0x982010), {
//     onEnter: function(args) {
//         var creature = this.context.esi.readPointer().readU32();
//         var hero = this.context.edi;
//         console.log('creaturecombat: ' + creature);
//         var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//         console.log(handler);
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });

// console.log('attaching selectedspellnotify');
// Interceptor.attach(ptr(0x685DD0), {
//     onEnter: function(args) {
//         console.log('spellnotify: ' + JSON.stringify(this.context));
//         var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//         console.log(handler);
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });

// console.log('attaching spellthing');
// Interceptor.attach(ptr(0x682E80), {
//     onEnter: function(args) {
//         console.log('spellthing: ' + JSON.stringify(this.context));
//         var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//         console.log(handler);
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });

// console.log('attaching preparecallback');
// Interceptor.attach(ptr(0x570760), {
//     onEnter: function(args) {
//         console.log('callback: ' + JSON.stringify(this.context));
//         var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//         console.log(handler);
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });

// console.log('attaching unitdeath');
// Interceptor.attach(ptr(0x56FA90), {
//     onEnter: function(args) {
//         console.log('death event: ' + this.context.ebx + ' ' + this.context.ebp);
//         //var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//         //console.log(handler);
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });

// console.log('attaching placement_tick');
// Interceptor.attach(ptr(0x56FB70), {
//     onEnter: function(args) {
//         console.log('placement_tick: ' + JSON.stringify(this.context));
//         //var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//         //console.log(handler);
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });

// console.log('attaching postevent thing'); 
// Interceptor.attach(ptr(0x484200), {
//     onEnter: function(args) {
//         console.log('lua_postevent: ' + this.context.edx.readPointer().readUtf8String());
//         //var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//         //console.log(handler);
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });

// console.log('attaching script thing'); 
// Interceptor.attach(ptr(0xD0CBA0), {
//     onEnter: function(args) {
//         console.log('init script: ' + this.context.edx.readUtf8String());
//         //var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//         //console.log(handler);
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });

// getting the instance pointer:
// 0xFE719C[0][1][2]-1380

// doprepare error handler 0x570280
// 8B 49 08                mov     ecx, [ecx+8]
// 8B 01                   mov     eax, [ecx]
// FF 10                   call    dword ptr [eax]
// C2 08 00                retn    8

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
        //Stalker.unfollow(this.threadId);
        //console.log('result: ' + this.context.eax);
    }
});

// console.log('attaching unitmove');
// Interceptor.attach(ptr(0x56FEF5), {
//     onEnter: function(args) {
//         console.log('unitmove: ' + JSON.stringify(this.context));
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });


// console.log('attaching arena event loop');
// Interceptor.attach(ptr(0x54AD10), {
//     onEnter: function(args) {
//         var arena_state = this.context.esi.sub(644).readU32();
//         var combat_ui = this.context.esi.sub(708).readPointer();
//         var script = this.context.esi.sub(1380).readPointer();
//         console.log('state:' + combat_ui);
        
//         //var handler = Thread.backtrace(this.context, Backtracer.ACCURATE);
//         //console.log(handler);
//     },

//     onLeave: function(args) {
//         //Stalker.unfollow(this.threadId);
//         //console.log('result: ' + this.context.eax);
//     }
// });





//TimeView setup
//53 56 8B F1 8B 86 ? ? ? ? 8B 48 08 8B 94 31 ? ? ? ? 8D 8C 31 ? ? ? ? 57 FF 52 48

//set ui var
//E8 ?? ?? ?? ?? EB 30 7D 0F
