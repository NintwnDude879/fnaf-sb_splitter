//Five Nights at Freddy's: Security Breach Autosplitter | v2.3.0
//Created by (Discord usernames) daltone_21 and nintendude_sr

//Original autosplitter created by patrogue
//Special thanks to CheatingMuppet, Cheat The Game, and the "Speedrun Tool Development" Discord server
    /*(especially apple1417 and just_ero, you two are the fucking GOAT)*/
//for making tutorials, helping us understand how to use Cheat Engine, and assisting in development of this ASL
//
//This was ported to Rust starting on 10/31/23, finished porting on [N/A]

#![no_std]
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::undocumented_unsafe_blocks,
    rust_2018_idioms
)]

mod default_settings;
use default_settings::Settings;

use asr::{
    //file_format::pe,
    future::{next_tick, retry},
    settings::Gui,
    //signature::Signature,
    timer::{self, TimerState},
    //watcher::Watcher,
    /*Address,*/ Process,
};

asr::panic_handler!();
asr::async_main!(stable);

const PROCESS_NAME: &str = &"fnaf9-Win64-Shipping.exe";

async fn main() {
    let mut settings = Settings::register();

    loop {
        // Hook to the target process
        let process = retry(||Process::attach(PROCESS_NAME)).await;

        startup();
        process
            .until_closes(async {
                // Once the target has been found and attached to, set up some default watchers
                let mut watchers = Watchers::default();

                // Perform memory scanning to look for the addresses we need
                let addresses = Addresses::init(&process).await;

                loop {
                    // Splitting logic. Adapted from OG LiveSplit:
                    // Order of execution
                    // 1. update() will always be run first. There are no conditions on the execution of this action.
                    // 2. If the timer is currently either running or paused, then the isLoading, gameTime, and reset actions will be run.
                    // 3. If reset does not return true, then the split action will be run.
                    // 4. If the timer is currently not running (and not paused), then the start action will be run.
                    settings.update();
                    update_loop(&process, &addresses, &mut watchers);

                    let timer_state = timer::state();
                    if timer_state == TimerState::Running || timer_state == TimerState::Paused {
                        if let Some(is_loading) = is_loading(&watchers, &settings) {
                            if is_loading {
                                timer::pause_game_time()
                            } else {
                                timer::resume_game_time()
                            }
                        }

                        if reset(&watchers, &settings) {
                            timer::reset()
                        } else if split(&watchers, &settings) {
                            timer::split()
                        }
                    }

                    if timer::state() == TimerState::NotRunning && start(&watchers, &settings) {
                        timer::start();
                        timer::pause_game_time();

                        if let Some(is_loading) = is_loading(&watchers, &settings) {
                            if is_loading {
                                timer::pause_game_time()
                            } else {
                                timer::resume_game_time()
                            }
                        }
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}

#[derive(Default)]
struct Watchers {
    //watcher_name: Watcher<i64>,
    //var_name: i64,
}

struct Addresses {
    //base_address: Address,
}

impl Addresses {
    async fn init(_process: &Process) -> Self {
//        let main_module = {
//            let main_module_base = retry(|| {
//                PROCESS_NAME.get_module_address(p).ok()
//            })
//            .await;
//            let main_module_size =
//                retry(|| pe::read_size_of_image(process, main_module_base)).await;
//            (main_module_base, main_module_size as u64)
//        };
//
//        const SIG: Signature<5> = Signature::new("76 0C 48 8B 0D");
//        let mut ptr = retry(|| SIG.scan_process_range(process, main_module)).await + 5;
//        ptr = ptr + 0x4 + retry(|| process.read::<i32>(ptr)).await;
//
        Self { /* base_address: ptr */ }
    }
}

fn startup() {

}

fn update_loop(_proc: &Process, _addresses: &Addresses, _watchers: &mut Watchers) {

}

fn split(_watchers: &Watchers, _settings: &Settings) -> bool {
    false
}

fn start(_watchers: &Watchers, _settings: &Settings) -> bool {
    false
}

fn reset(_watchers: &Watchers, _settings: &Settings) -> bool {
    false
}

fn is_loading(_watchers: &Watchers, _settings: &Settings) -> Option<bool> {
    Some(true)
}
/*

startup {
    vars.CompletedSplits = new HashSet<string>();
}

init {
    #region Set version (and a few variables)
        //Sets the version of the game upon startup
        int gameSize = modules.First().ModuleMemorySize;
        refreshRate = 60;

        switch (gameSize){
            default: {
                vars.version = 100; // Unsupported
                if (!settings["Unsupported version warning"]) break;
                MessageBox.Show("Sorry, it seems like the version of Security Breach that you're using isn't currently supported!\n\n"+
                "If this seems like a mistake, or you would like to suggest an additional version to support, please go to https://forms.gle/jxidK6RFToEXzUDe7 or contact either Daltone#2617 or Nintendude#0447 on Discord.\n\n"+
                "Sorry for the inconvenience.", "Warning: Version Not Supported", MessageBoxButtons.OK, MessageBoxIcon.Error).ToString();
                break;
            }
            case 0x48AE000: vars.version = 1.04; break;
            case 0x48AF000: vars.version = 1.05; break;
            case 0x48B0000: vars.version = 1.07; break;
            case 0x48B8000: vars.version = 1.11; break;
        }

        print("Version = " + vars.version);

        const int CLASS_OFFSET = 0x10;
        const int CHILD_OFFSET = 0x50;
        const int NEXT_OFFSET = 0x20;
        const int NAME_OFFSET = 0x28;
        const int INTERNAL_OFFSET = 0x4C;
        const int SUPERFIELD_OFFSET = 0x40;
        vars.offsets = new Dictionary<string, int>();
        vars.fnames = new Dictionary<long, string>();
        vars.interactibleName = "";
        vars.cachedPos = new Vector3f();
        vars.foundLeave = false;
        vars.aftonButtons = 0;
    #endregion

    #region Declare functions

        #region Sigscan adjacent/Unreal Engine introspection related funcs
            vars.GetStaticPointerFromSig = (Func<string, int, IntPtr>) ( (signature, instructionOffset) => {
                var scanner = new SignatureScanner(game, modules.First().BaseAddress, (int)modules.First().ModuleMemorySize);
                var pattern = new SigScanTarget(signature);
                var location = scanner.Scan(pattern);
                if (location == IntPtr.Zero) return IntPtr.Zero;
                int offset = game.ReadValue<int>((IntPtr)location + instructionOffset);
                return (IntPtr)location + offset + instructionOffset + 0x4;
            });

            vars.GetNameFromFName = (Func<long, string>) ( longKey => {
                if (vars.fnames.ContainsKey(longKey)) return vars.fnames[longKey];
                int key = (int)(longKey & uint.MaxValue);
                int partial = (int)(longKey >> 32);
                int chunkOffset = key >> 16;
                int nameOffset = (ushort)key;
                IntPtr namePoolChunk = memory.ReadValue<IntPtr>((IntPtr)vars.FNamePool + (chunkOffset+2) * 0x8);
                Int16 nameEntry = game.ReadValue<Int16>((IntPtr)namePoolChunk + 2 * nameOffset);
                int nameLength = nameEntry >> 6;
                string output = game.ReadString((IntPtr)namePoolChunk + 2 * nameOffset + 2, nameLength);
                string outputParsed = (partial == 0) ? output : output + "_" + partial.ToString();
                vars.fnames[longKey] = outputParsed;
                return outputParsed;
            });

            vars.GetPropertyOffset = (Func<IntPtr, string, IntPtr>) ((address, name) => {
                var _class = game.ReadPointer(address + CLASS_OFFSET);
                for (
                    ;
                    _class != IntPtr.Zero;
                    _class = game.ReadPointer(_class + SUPERFIELD_OFFSET)
                ){
                    for (IntPtr property = game.ReadPointer(_class + CHILD_OFFSET);
                        property != IntPtr.Zero;
                        property = game.ReadPointer(property + NEXT_OFFSET)
                    ){
                        string propName = vars.GetNameFromFName(game.ReadValue<long>(property + NAME_OFFSET));
                        if (propName == name){
                            int offset = game.ReadValue<int>(property + INTERNAL_OFFSET);
                            print("Found property \""
                            + name
                            + "\" at offset 0x"
                            + offset.ToString("X")
                            );

                            vars.offsets[name] = offset;
                            return property;
                        }
                    }
                }

                print("Couldn't find property \""+name+"\".");
                return IntPtr.Zero;
            });

        #endregion

        #region Player state related funcs
            vars.checkSphereNoBool = (Func<Vector3f, bool>)((pos)
                => vars.watchers["pos"].Current.Distance(pos) < 300f);

            vars.checkItem = (Func<string, Vector3f, bool>)((name, item)
                => settings[name] && vars.checkSphereNoBool(item));

            vars.checkBoxNoBool = (Func<Vector3f, Vector3f, bool>)((point1, point2) => {
                /* This first section is just to allow you to pick any two points directly opposite each other
                on a cuboid and still allow for the rest of the code to work, it's really just for convenience's sake*/

                // Calculate which X/Y/Z is the lower of the two points, and set the upper/lower bound point along that axis accordingly
                Vector3f LB = new Vector3f(Math.Min(point1.X, point2.X), Math.Min(point1.Y, point2.Y), Math.Min(point1.Z, point2.Z));
                Vector3f UB = new Vector3f(Math.Max(point1.X, point2.X), Math.Max(point1.Y, point2.Y), Math.Max(point1.Z, point2.Z));

                // Actually calculate if you are in the bounds of the defined cuboid
                // Includes a check to see if you've already completed this split (uses HashSet<string>, initialized in startup{})
                if (LB.X <= vars.watchers["pos"].Current.X && vars.watchers["pos"].Current.X <= UB.X
                && LB.Y <= vars.watchers["pos"].Current.Y && vars.watchers["pos"].Current.Y <= UB.Y
                && LB.Z <= vars.watchers["pos"].Current.Z && vars.watchers["pos"].Current.Z <= UB.Z){
                    return true;
                }
                return false;
            });

            vars.checkBox = (Func<string, Vector3f, Vector3f, bool>)((name, point1, point2)
                => settings[name] && vars.checkBoxNoBool(point1, point2) && vars.CompletedSplits.Add(name));

            vars.checkOldBoxNoBool = (Func<Vector3f, Vector3f, bool>)((point1, point2) => {
                /* This first section is just to allow you to pick any two points directly opposite each other
                on a cuboid and still allow for the rest of the code to work, it's really just for convenience's sake*/

                // Calculate which X/Y/Z is the lower of the two points, and set the upper/lower bound point along that axis accordingly
                Vector3f LB = new Vector3f(Math.Min(point1.X, point2.X), Math.Min(point1.Y, point2.Y), Math.Min(point1.Z, point2.Z));
                Vector3f UB = new Vector3f(Math.Max(point1.X, point2.X), Math.Max(point1.Y, point2.Y), Math.Max(point1.Z, point2.Z));

                //Checks to see if the old position is outside a cuboid
                if (LB.X <= vars.watchers["pos"].Old.X && vars.watchers["pos"].Old.X <= UB.X
                &&  LB.Y <= vars.watchers["pos"].Old.Y && vars.watchers["pos"].Old.Y <= UB.Y
                &&  LB.Z <= vars.watchers["pos"].Old.Z && vars.watchers["pos"].Old.Z <= UB.Z){
                    return true;
                }
                return false;
            });


            vars.checkElevs = (Func<bool>)(()
                => vars.checkBoxNoBool(new Vector3f(24192,  49679,  360), new Vector3f(23814,  50161, 752))     //Afton
                || vars.checkBoxNoBool(new Vector3f(328,    27856,  1421),new Vector3f(759,    27381, 1819))    //Kitchen (atrium)
                || vars.checkBoxNoBool(new Vector3f(328,    27856, -10),  new Vector3f(759,    27381, 408))     //Kitchen (kitchen)
                || vars.checkBoxNoBool(new Vector3f(-11670, 40159,  1461),new Vector3f(-12176, 40711, 1816))    //Monty Golf (atrium)
                || vars.checkBoxNoBool(new Vector3f(-14204, 42656,  1474),new Vector3f(-14759, 43240, 1847))    //Monty Golf (monty golf)
                || vars.checkBoxNoBool(new Vector3f(-2669,  28897,  2038),new Vector3f(-2108,  28359, 2408))    //Atrium/Lobby (left side from atrium pov)
                || vars.checkBoxNoBool(new Vector3f(-1295,  28903,  2038),new Vector3f(-738,   28369, 2408))    //Atrium/Lobby 2.0 (right side from atrium pov)
                || vars.checkBoxNoBool(new Vector3f(5446,   37421,  3212),new Vector3f(5975,   36909, 3598))    //Bonnie Bowl
                || vars.checkBoxNoBool(new Vector3f(7725,   34759,  1450),new Vector3f(8232,   34243, 1819))    //Fazerblast
                || vars.checkBoxNoBool(new Vector3f(4289,   29381,  3289),new Vector3f(5530,   30187, 3555))    //West Arcade (atrium)
                || vars.checkBoxNoBool(new Vector3f(5699,   28832,  2036),new Vector3f(4640,   28066, 2407))    //West Arcade (west arcade)
                || vars.checkBoxNoBool(new Vector3f(-5071,  52079,  1911),new Vector3f(-5584,  52281,-1160))    //Chica Room (both encompassed in big box, same for next 3)
                || vars.checkBoxNoBool(new Vector3f(-2812,  53483,  1683),new Vector3f(-1937,  52803,-1160))    //Monty Room
                || vars.checkBoxNoBool(new Vector3f(-370,   52814,  1920),new Vector3f(-1155,  53030,-1193))    //Roxy Room
                || vars.checkBoxNoBool(new Vector3f(2348,   52554,  1870),new Vector3f(2073,   52156,-1179))    //Freddy Room
            );

            vars.checkPQPositionNoBool = (Func<double, double, double, double, bool>)((xLB, xUB, yLB, yUB)
                =>  xLB <= vars.watchers["pos"].Current.Y && vars.watchers["pos"].Current.Y <= xUB
                &&  yLB <= vars.watchers["pos"].Current.X && vars.watchers["pos"].Current.X <= yUB);

            vars.checkPQPosition = (Func<string, double, double, double, double, bool>)((name, xLB, xUB, yLB, yUB)
                => settings[name]
                && vars.checkPQPositionNoBool(xLB, xUB, yLB, yUB)
                && vars.CompletedSplits.Add(name));

            //checks in a circle (radius 200), upper and lower Z bound used
            vars.checkGen = (Func<string, float, float, float, float, bool>)((name, x, y, zLower, zUpper) => {
                if (!settings[name]
                || zLower > vars.watchers["pos"].Current.Z || vars.watchers["pos"].Current.Z > zUpper) return false;

                Vector3f oPos = vars.watchers["pos"].Old, cPos = vars.watchers["pos"].Current, target = new Vector3f(x, y, 0);
                return ((oPos.DistanceXY(target) <= 200f && cPos.DistanceXY(target) > 200f) && vars.CompletedSplits.Add(name));
            });

            vars.checkTimeNoBool = (Func<int, int, bool>)((hour, minute)
                => vars.getHour() == hour && vars.getMinute() == minute);

            vars.checkTime = (Func<string, int, int, bool>)((name, hour, minute)
                => settings[name]
                && vars.checkTimeNoBool(hour, minute)
                && vars.CompletedSplits.Add(name));

        #endregion

        #region Miscellaneous functions
            vars.getHour = (Func<int>)(() => {
                if (vars.watchers["clockTime"].Current < 0){
                    return -1;
                }
                return (int)vars.watchers["clockTime"].Current/3600;
            });

            vars.getOldHour = (Func<int>)(() => {
                if (vars.watchers["clockTime"].Old < 0){
                    return -1;
                }
                return (int)vars.watchers["clockTime"].Old/3600;
            });

            vars.getMinute = (Func<int>)(() => {
                if ((int)vars.watchers["clockTime"].Current%3600/60 < 0){
                    return 60+(int)vars.watchers["clockTime"].Current%3600/60;
                }
                return (int)vars.watchers["clockTime"].Current%3600/60;
            });

            vars.getOldMinute = (Func<int>)(() => {
                if ((int)vars.watchers["clockTime"].Old%3600/60 < 0){
                    return 60+(int)vars.watchers["clockTime"].Old%3600/60;
                }
                return (int)vars.watchers["clockTime"].Old%3600/60;
            });

            vars.conditionalFindProperty = (Action<IntPtr, string>)((address, name) => {
                if (!vars.offsets.ContainsKey(name)) vars.GetPropertyOffset(address, name);
            });

            vars.cacheCurrentPos = (Action)(()
            => vars.cachedPos = new Vector3f(vars.watchers["pos"].Current.X, vars.watchers["pos"].Current.Y, vars.watchers["pos"].Current.Z));

            vars.findLeave = (Func<bool>)(() => {
                if (!vars.foundLeave){
                    if (!vars.offsets.ContainsKey("FinalChoice")){
                        print("Finding 'FinalChoice;'");
                        if (vars.GetPropertyOffset(vars.watchers[2].Current, "FinalChoice") == IntPtr.Zero){
                            return false;
                        }
                    }

                    IntPtr finalChoice = game.ReadPointer((IntPtr)vars.watchers[2].Current + (int)vars.offsets["FinalChoice"]);

                    if (!vars.offsets.ContainsKey("Leave")){
                        print("Finding 'Leave';");
                        if (vars.GetPropertyOffset(finalChoice, "Leave") == IntPtr.Zero){
                            return false;
                        }
                    }
                    vars.foundLeave = true;
                }
                if (vars.watchers[3].Current.GetType() != typeof(int)){
                    vars.watchers[3] = new MemoryWatcher<int>(new DeepPointer(vars.watchers[2].Current + vars.offsets["FinalChoice"], vars.offsets["Leave"])){
                        Name = "leaveButton" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull
                    };
                }
                return true;
            });

            vars.checkLeave = (Func<bool>)(()
            => (int)vars.watchers["leaveButton"].Current == 0 && (int)vars.watchers["leaveButton"].Old != 0
                && vars.watchers["worldCheck"].Current != 0);

            vars.resetVariables = (Action)(() => {
                //These 2 watchers are addresses which change while the game is running, and which change depending on what the player is interacting with.
                //Make sure they are not garbage data when reading.
                vars.interactibleName = "";
                vars.watchers[0] = new MemoryWatcher<bool>((IntPtr)null){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
                vars.watchers[1] = new MemoryWatcher<bool>((IntPtr)null){ Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };

                vars.montyBalls = 0;
                vars.fbFlags = 0;
                vars.aftonButtons = 0;

                //Used to keep certain splits from repeating (reset)
                vars.CompletedSplits.Clear();

                //Arcade Splits
                vars.arcade = "N/A";
                //monty golf
                vars.nHole = 0;
                //pq2
                vars.pq2_8 = false;

                vars.onMenu = false;
            });

            vars.printCompletedSplits = (Action)(() => {
                foreach (string str in vars.CompletedSplits){
                    print(str);
                }
            });
        #endregion

    #endregion

    #region Sigscanning
        // FNamePool's base address doesn't get accessed at all (for some reason) but it's base address + 8 does
        // Because of this, you can just sigscan for the base address + 8 and then subtract the 8 to get the real pointer
        vars.FNamePool      = vars.GetStaticPointerFromSig("8B 05 ??????03"     //mov eax, [badFNamePool]
                                                          +"FF C0"              //inc eax
                                                          +"C1 E9 10"           //shr ecx,10
                                                          +"3B C8"              //cmp ecx,eax
                                                          +"0F92 C0"            //setb al
                                                          +"C3 CC"              //ret
                                                          , 2)
                                                          -8;

        vars.UWorld         = vars.GetStaticPointerFromSig("E8 ????????"        //call ????????
                                                          +"48 8B 88 ??0?0000"  //mov rcx,[rax+???]
                                                          +"48 89 0D ??????02"  //mov [UWorld],rcx
                                                          , 15);

        vars.GEngine        = vars.GetStaticPointerFromSig("48 8B 05 ????????"  //mov rax,[GEngine]
                                                          +"48 8B D1"           //mov rdx,rcx
                                                          +"48 8B 88 F80A0000"  //mov rcx,[rax+AF8]
                                                          +"48 85 C9"           //test rcx,rcx
                                                          +"74 07"              //je
                                                          +"48 8B 01"           //mov rax,[rcx]
                                                          +"48 FF 60 40"        //jmp qword ptr [rax+40]
                                                          , 3);

        vars.isLoading      = vars.GetStaticPointerFromSig("48 2B E1"           //sub rsp,rcx
                                                          +"C7 45 ?? FFFFFFFF"  //mov [rbp+??],FFFFFFFF
                                                          +"44 8B F7"           //mov r14d,edi
                                                          +"0FB6 3D ??????0?"   //movzx edi,byte ptr [isLoading]
                                                          , 16);

        if (vars.UWorld == IntPtr.Zero || vars.GEngine == IntPtr.Zero || vars.FNamePool == IntPtr.Zero || vars.isLoading == IntPtr.Zero){
            throw new Exception("UWorld/GameEngine/FNamePool not initialized - trying again");
        }

        vars.GetPropertyOffset(game.ReadPointer((IntPtr)vars.GEngine), "GameInstance");
        vars.GetPropertyOffset(game.ReadPointer((IntPtr)vars.GEngine), "TransitionType");
        vars.GetPropertyOffset(game.ReadPointer((IntPtr)vars.UWorld), "AuthorityGameMode");
        if (vars.version < 1.11){
            vars.freddyThing                 = new DeepPointer(vars.UWorld, 0x188, 0xE0, 0x38, 0xB8);
            vars.clockTime                   = new DeepPointer(vars.GEngine, vars.offsets["GameInstance"], 0xE0, 0x80, 0xC4);
        }
        else if (vars.version == 1.11){
            vars.freddyThing                 = new DeepPointer(vars.UWorld, vars.offsets["AuthorityGameMode"], 0x310, 0x120, 0x18C);
            vars.clockTime                   = new DeepPointer(vars.GEngine, vars.offsets["GameInstance"], 0xE0, 0x80, 0xC4);
        }
        else {
            vars.freddyThing                 = new DeepPointer(vars.UWorld, vars.offsets["AuthorityGameMode"], 0x310, 0x120, 0x18C);
            vars.clockTime                   = new DeepPointer(vars.GEngine, vars.offsets["GameInstance"], 0xF0, 0x80, 0xC4);
        }
    #endregion

    #region Declare MemoryWatcherList
        vars.watchers = new MemoryWatcherList {
            //These are at the top so they will always be index 0 or 1 in this list. DO NOT CHANGE UNLESS YOU KNOW THE RAMIFICATIONS.

            new MemoryWatcher<bool>((IntPtr)null) { Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
            new MemoryWatcher<int>((IntPtr)null) { Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
            new MemoryWatcher<IntPtr>(new DeepPointer(vars.UWorld, vars.offsets["AuthorityGameMode"], 0x318, 0x4E0, 0xE8, 0x0)) { Name = "leaveThing" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
            new MemoryWatcher<bool>((IntPtr)null) { Name = "leaveButton" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },

            //Freddy's Power OR Freddy Thingie (1.11+)
            new MemoryWatcher<int>(vars.freddyThing) { Name = "freddyThing" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },

            //Player Info
            //GEngine.GameInstance.LocalPlayers[0].PlayerController.Pawn.CollisionComponent.Position[0x1D0]
            new MemoryWatcher<Vector3f>(new DeepPointer(vars.GEngine, vars.offsets["GameInstance"], 0x38, 0x0, 0x30, 0x258, 0x298, 0x1D0)) { Name = "pos" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
            new MemoryWatcher<float>(new DeepPointer(vars.GEngine, vars.offsets["GameInstance"], 0x38, 0x0, 0x30, 0x268, 0x298, 0x1D4)) { Name = "worldCheck", FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },

            //Arcade pointers
            new MemoryWatcher<int>(new DeepPointer(vars.UWorld, vars.offsets["AuthorityGameMode"], 0x378, 0x270, 0x230, 0x40)) { Name = "golfStrokeCount" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
            new MemoryWatcher<bool>(new DeepPointer(vars.GEngine, vars.offsets["GameInstance"], 0x38, 0x0, 0x30, 0x258, 0x3F9)) { Name = "pq3Attack" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },

            //Counter pointers
            new MemoryWatcher<int>(new DeepPointer(vars.UWorld, 0x98, 0x40, 0x128, 0xA8, 0x580, 0x290, 0x14)) { Name = "DGens" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },

            //Keeps track of items
            new MemoryWatcher<int>(new DeepPointer(vars.UWorld, 0x188, 0xE0, 0x38, 0xC0)) { Name = "securityBadgeCount" },
            new MemoryWatcher<int>(new DeepPointer(vars.UWorld, 0x188, 0xE0, 0x38, 0x138)) { Name = "itemCount" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
            new MemoryWatcher<int>(new DeepPointer(vars.UWorld, 0x98, 0x8A0, 0x128, 0xB8, 0x128, 0x328, 0x3C8)) { Name = "splashScreen" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },

            //In-Game Clock (keeps track of time in seconds, you need to do math to figure out hour & minute)
            new MemoryWatcher<float>(vars.clockTime) { Name = "clockTime" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },

            //Used to pause the timer
            new MemoryWatcher<bool>(new DeepPointer(vars.GEngine, vars.offsets["TransitionType"])) { Name = "pause" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
            new MemoryWatcher<int>(new DeepPointer(vars.UWorld, vars.offsets["AuthorityGameMode"], 0x1A8, 0x20, 0x100, 0xA0, 0x228)) { Name = "menu", FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
            new MemoryWatcher<bool>(vars.isLoading) { Name = "isLoading" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },

            //Experimental elevator fix that only requires 3 pointers (instead of 12)
            //UWorld.AuthorityGameMode.GregoryPawn.PlayerInteractComponent.ClosestInteractible
            new MemoryWatcher<IntPtr>(new DeepPointer(vars.UWorld, vars.offsets["AuthorityGameMode"], 0x318, 0x4E0, 0xC8)) { Name = "closestInteractibleAddress" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
            //UWorld.AuthorityGameMode.GregoryPawn.PlayerInteractComponent.ClosestInteractible.Name
            new MemoryWatcher<long>(new DeepPointer(vars.UWorld, vars.offsets["AuthorityGameMode"], 0x318, 0x4E0, 0xC8, 0x18)) { Name = "closestInteractibleFName" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull },
        };
    #endregion
}

update {
    vars.watchers.UpdateAll(game);
    #region Change 'lastInteractible' watcher based on what you last interacted with
    //If the player is interacting with a desired interactible, cache it into lastInteractable (raw IntPtr, be careful)
    string currentName = vars.GetNameFromFName(vars.watchers["closestInteractibleFName"].Current);
    string currInteract = (string)vars.interactibleName;
    IntPtr currentAddress = vars.watchers["closestInteractibleAddress"].Current;
    //Any elevator button
    if (currentName.Contains("ElevatorButton")){
        vars.watchers[0] = new MemoryWatcher<bool>(currentAddress+0x2E8){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "elevButton";
    }
    //Vanny Ending button
    else if (currentName.Contains("DestroyVannyEndingTrigger")
    && currInteract != "vannyButton"){
        vars.watchers[0] = new MemoryWatcher<bool>(currentAddress+0x240){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "vannyButton";
        vars.cacheCurrentPos();
    }
    //Monty cannon balls counter (requires an internal variable to keep track of # of balls in bucket)
    else if (currentName.Contains("BallCannon")){
        vars.conditionalFindProperty(currentAddress, "NumberTargetsHit");
        vars.watchers[0] = new MemoryWatcher<int>(currentAddress+vars.offsets["NumberTargetsHit"]){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "montyCannon";
    }
    //Fazerblast flag watcher (requires an internal variable to keep track of # of flags captured)
    else if (currentName.Contains("Fazerblast_CaptureFlag")){
        vars.conditionalFindProperty(currentAddress, "CanStartCapture");
        vars.watchers[0] = new MemoryWatcher<bool>(currentAddress+vars.offsets["CanStartCapture"]){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "fazerblastFlag";
    }
    //Burntrap button watcher (requires an internal variable to keep track of # of flags captured)
    else if (currentName.Contains("BurntrapButton")){
        vars.watchers[0] = new MemoryWatcher<bool>(currentAddress+0x2E8){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "burntrapButton";
    }
    //Pizzaplex Cameras button (intro sequence)
    else if (currentName.Contains("BB_UtilityStart")){
        vars.watchers[1] = new MemoryWatcher<bool>(currentAddress+0x2E8){ Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "cameraButton";
        vars.cacheCurrentPos();
    }
    //Daycare pass upgrade machine
    else if (currentName.Contains("FazPassUpgradeMachine")){
        vars.watchers[1] = new MemoryWatcher<bool>(currentAddress+0x338){ Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "daycareMachine";
        vars.cacheCurrentPos();
    }
    //Flashlight (daycare)
    else if (currentName.Contains("Flashlight")){
        vars.conditionalFindProperty(currentAddress, "FlashlightAvailable");
        vars.watchers[1] = new MemoryWatcher<bool>(currentAddress+vars.offsets["FlashlightAvailable"]){ Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "flashlight";
        vars.cacheCurrentPos();
    }
    //Chica's Voicebox (specific weird edge case, don't worry about it)
    else if (vars.watchers[1].Current.GetType() != typeof(float) && currentName.Contains("ChicaSewer")){
        vars.conditionalFindProperty(currentAddress, "PlayerInteractHoldComponent");
        vars.watchers[0] = new MemoryWatcher<long>((IntPtr)null){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.watchers[1] = new MemoryWatcher<float>(game.ReadPointer(currentAddress+(int)vars.offsets["PlayerInteractHoldComponent"])+0xD0){ Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "chicaSewer";
        vars.cacheCurrentPos();
    }
    //Any message collectible
    else if (currentName.Contains("Message")
    || currentName.Contains("Clue")
    || currentName.Contains("Bag")
    || currentName.Contains("Complaint")){
        vars.watchers[0] = new MemoryWatcher<long>(currentAddress+0x25C){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.watchers[1] = new MemoryWatcher<float>(game.ReadPointer(currentAddress+0x248)+0xD0){ Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "message";
        vars.cacheCurrentPos();
    }
    //Any Collectible (that is not a message) (equipment, etc.)
    else if (currentName.Contains("Collect")
    || currentName.Contains("SecurityBadge")
    || currentName.Contains("Ticket")
    || currentName.Contains("Pass")
    || currentName.Contains("MrHippoMagnet")
    || currentName.Contains("MontyMysteryMix")
    || currentName.Contains("MazerciseControlKey")){
        vars.watchers[0] = new MemoryWatcher<long>(currentAddress+0x25C){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.watchers[1] = new MemoryWatcher<float>(new DeepPointer(currentAddress+0x248, 0xD0)){ Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "collectible";
        vars.cacheCurrentPos();
    }


    //If the player is out of range of the interactable, reset cached interactable address
    //(ensures the player doesn't get splits/pauses from the game putting something into the same spot in memory after the interactable has unloaded)
    if (vars.interactibleName == "elevButton" && !vars.checkElevs()){
        vars.watchers[0] = new MemoryWatcher<bool>((IntPtr)null){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "";
    }
    else if (vars.interactibleName == "montyCannon" && !vars.checkBoxNoBool(new Vector3f(-15577, 46830, 2893), new Vector3f(-22025, 39450, 3507))){
        vars.watchers[0] = new MemoryWatcher<bool>((IntPtr)null){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "";
    }
    else if (vars.interactibleName == "fazerblastFlag" && !vars.checkBoxNoBool(new Vector3f(17998, 28715, 984), new Vector3f(13200, 33755, 2888))){
        vars.watchers[0] = new MemoryWatcher<bool>((IntPtr)null){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "";
    }
    else if (vars.interactibleName == "burntrapButton" && !vars.checkBoxNoBool(new Vector3f(24373, 43303, -8034), new Vector3f(29296, 38254, -8815))){
        vars.watchers[0] = new MemoryWatcher<bool>((IntPtr)null){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
        vars.interactibleName = "";
    }
    else if (!vars.checkSphereNoBool(vars.cachedPos)){
        if (vars.interactibleName == "vannyButton"){
            vars.watchers[0] = new MemoryWatcher<bool>((IntPtr)null){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
            vars.interactibleName = "";
        }
        else if (vars.interactibleName == "message"
        ||       vars.interactibleName == "collectible"){
            vars.watchers[0] = new MemoryWatcher<bool>((IntPtr)null){ Name = "lastInteractible" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
            vars.watchers[1] = new MemoryWatcher<int>((IntPtr)null){ Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
            vars.interactibleName = "";
        }
        else if (vars.interactibleName == "daycareMachine"
        ||       vars.interactibleName == "cameraButton"
        ||       vars.interactibleName == "flashlight"
        ||       vars.interactibleName == "chicaSewer"){
                vars.watchers[1] = new MemoryWatcher<int>((IntPtr)null){ Name = "canCollect" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
                vars.interactibleName = "";
        }
    }
    if (!vars.checkBoxNoBool(new Vector3f(-2238, 19846, 1442), new Vector3f(-1943, 19521, 1746))     //Lobby 1
        &&!vars.checkBoxNoBool(new Vector3f(-1437, 19846, 1442), new Vector3f(-1144, 19521, 1746))   //Lobby 2
        &&!vars.checkBoxNoBool(new Vector3f(-1789, 22700, 3268), new Vector3f(-1595, 22620, 3529))   //Fire ending
        &&!vars.checkBoxNoBool(new Vector3f(-3194, 19196, 0), new Vector3f(-2911, 18959, 312))       //Car battery ending
    ){
        vars.watchers[3] = new MemoryWatcher<bool>((IntPtr)null) { Name = "leaveButton" , FailAction = MemoryWatcher.ReadFailAction.SetZeroOrNull };
    }
    #endregion

    //Reset variables on starting a new game (even if you don't reset for new game)
    if (vars.getHour() == -1 && vars.getMinute() == 0 && vars.getOldHour() != -1 && vars.getOldMinute() != 0){
        vars.resetVariables();
    }
    //if (vars.watchers["closestInteractibleAddress"].Current != vars.watchers["closestInteractibleAddress"].Old)
    //    print(vars.watchers["closestInteractibleAddress"].Current.ToString("X"));
    //
    //if (vars.watchers["closestInteractibleAddress"].Current != vars.watchers["closestInteractibleAddress"].Old
    //&& vars.interactibleName == "collectible" && vars.watchers["lastInteractible"].Current != 0)
    //    print(vars.GetNameFromFName(vars.watchers["lastInteractible"].Current));
}

start {
    vars.resetVariables();
    //Start conditions (time, Freddy power, freddyThing)
    if (vars.getHour() == -1 && vars.getMinute() == 0){
        if (vars.version < 1.11){
            if (vars.watchers["freddyThing"].Old == 100 && vars.watchers["freddyThing"].Current == 30){
                print("Start Timer (pre 1.11)");
                return true;
            }
        }
        else {
            if (vars.watchers["freddyThing"].Old == 0 && vars.watchers["freddyThing"].Current == 1){
                print("Start Timer (1.11+)");
                return true;
            }
        }
    }
}

reset {
    //Resets timer upon starting new game/loading a game from the starting file
    if (settings["Reset Settings"] && vars.getOldHour() != -1 && vars.checkTime("Reset On New Game", -1, 0)){
        print("Reset on New Game");
        return true;
    }
    return false;
}

isLoading {
    if (!settings["In-Game Time Settings"]) return false;

    if (vars.watchers["worldCheck"].Current != 0 || vars.onMenu){
        if (vars.arcade != "N/A"){
            vars.arcade = "N/A";
            print("Arcade: " + vars.arcade);
        }
    }

    #region Check whether you just entered an arcade
    else if (vars.arcade == "N/A"){
        if (vars.checkOldBoxNoBool(new Vector3f(-17000, 27200, 2000), new Vector3f(-16500, 27600, 2300))){
            vars.arcade = "BB Arcade";
        }
        else if (vars.checkOldBoxNoBool(new Vector3f(-18200, 44100, 900), new Vector3f(-17900, 44300, 1100))){
            vars.arcade = "Monty Golf";
        }
        else if (vars.watchers["pos"].Current.X == 0 && vars.watchers["pos"].Current.Y == 0){
            if (vars.checkOldBoxNoBool(new Vector3f(7000, 46500, 2100), new Vector3f(8500, 48000, 2300))){
                vars.arcade = "Princess Quest 1";
            }
            else if (vars.checkOldBoxNoBool(new Vector3f(7500, 20300, 3200), new Vector3f(9000, 21000, 3400))){
                vars.arcade = "Princess Quest 2";
            }
            else if (vars.checkOldBoxNoBool(new Vector3f(17750, 28775, 2500), new Vector3f(18000, 29000, 2700))){
                vars.arcade = "Princess Quest 3";
            }
        }

        if (vars.arcade != "N/A"){
            print("Arcade: " + vars.arcade);
        }
    }
    #endregion

    if (vars.interactibleName == "elevButton" && vars.watchers["lastInteractible"].Current){
        if (!vars.watchers["lastInteractible"].Old){
            print("Elevator Pause");
        }
        return true;
    }

    if (settings["Stop Timer On Menu"]){
        if (vars.watchers["pos"].Current.Y == 0 && vars.arcade == "N/A"){
            if (!vars.onMenu){
                print("Stop Timer On Menu");
            }
            vars.onMenu = true;
        }
        else if (vars.watchers["worldCheck"].Current != 0 || vars.arcade != "N/A"){
            vars.onMenu = false;
        }
        if (vars.onMenu){
            return true;
        }
    }

    if (settings["Stop Timer When Paused"] && vars.watchers["pause"].Current && vars.watchers["worldCheck"].Current != 0){
        if (!vars.watchers["pause"].Old){
            print("Stop Timer When Paused");
        }
        return true;
    }

    if (settings["Stop Timer When Loading"]){
        if (vars.watchers["isLoading"].Current){
            if (!vars.watchers["isLoading"].Old){
                print("Stop Timer When Loading");
            }
            return true;
        }
    }
    return false;
}

split {
    if (!settings["Split Settings"]) return false;

    #region Arcade exit splits
        if (vars.arcade == "N/A"){
            if (vars.CompletedSplits.Contains("bb_start") && settings["bb_end"]
            && vars.CompletedSplits.Add("bb_end")){
                print("Exit BB Arcade");
                return true;
            }
            if (vars.CompletedSplits.Contains("mg_start") && settings["mg_end"]
            && vars.CompletedSplits.Add("mg_end")){
                print("Exit MG Arcade");
                return true;
            }
            if (vars.CompletedSplits.Contains("pq1_start") && settings["pq1_end"]
            && vars.CompletedSplits.Add("pq1_end")){
                print("Exit PQ1 Arcade");
                return true;
            }
            if (vars.CompletedSplits.Contains("pq2_start") && settings["pq2_end"]
            && vars.CompletedSplits.Add("pq2_end")){
                print("Exit PQ2 Arcade");
                return true;
            }
        }
    #endregion

    string arcade = vars.arcade;
    if (arcade == "N/A"){
    #region Counter splits
        if (vars.watchers["DGens"].Current > vars.watchers["DGens"].Old && settings["D_Generator " + vars.watchers["DGens"].Current]){
            print("DGen " + vars.watchers["DGens"].Current);
            return true;
        }
        if (vars.interactibleName == "fazerblastFlag" && vars.watchers["lastInteractible"].Old  && !vars.watchers["lastInteractible"].Current){
            vars.fbFlags++;
            if (settings["Flag " + vars.fbFlags]){
                print("Flag " + vars.fbFlags);
                return true;
            }
        }
        if (vars.interactibleName == "montyCannon" && vars.watchers["lastInteractible"].Old < vars.watchers["lastInteractible"].Current){
            vars.montyBalls++;
            if (settings[vars.montyBalls + " Balls"]){
                print(vars.montyBalls + " Balls");
                return true;
            }
        }
        if (vars.checkGen("S_Generator 1", -1515, 16575, -10000, -2500)) return true;
        if (vars.checkGen("S_Generator 2", -10525, 21155, -10000, -2500)) return true;
        if (vars.checkGen("S_Generator 3", -3785, 16480, -10000, -2500)) return true;
        if (vars.checkGen("WA_Generator 1", 10440, 28375, 2140, 2290)) return true;
        if (vars.checkGen("WA_Generator 2", 8920, 23075, 2100, 2250)) return true;
        if (vars.checkGen("WA_Generator 3", 2125, 25970, 2150, 2300)) return true;
        if (vars.checkGen("WA_Generator 4", 3030, 27210, 3290, 3440)) return true;
        if (vars.checkGen("WA_Generator 5", 9205, 20770, 3290, 3440)) return true;
        if (vars.interactibleName == "burntrapButton" && vars.watchers["lastInteractible"].Old && !vars.watchers["lastInteractible"].Current){
            vars.aftonButtons++;
            if (settings["Button " + vars.aftonButtons]){
                print("Button " + vars.aftonButtons);
                return true;
            }
            if (settings["Afton Ending"] && vars.aftonButtons == 8){
                print("Button 8 / End");
                return true;
            }
        }
    #endregion

    #region Positional/Deload splits
        if (vars.watchers["pos"].Current.X != vars.watchers["pos"].Old.X
        || vars.watchers["pos"].Current.Y != vars.watchers["pos"].Old.Y
        || vars.watchers["pos"].Current.Z != vars.watchers["pos"].Old.Z){
            //Deloads
            if (vars.checkBox("Foxy Cutout Deload", new Vector3f(-4942, 53000, 1790), new Vector3f(-4769, 52900, 2000))) return true;
            if (vars.checkBox("Arcade Deload", new Vector3f(-13600, 30000, 1821.75f), new Vector3f(-13300, 31800, 2000))) return true;
            if (vars.checkBox("Theatre Deload", new Vector3f(-20000, 32377.5f, 2516), new Vector3f(-19500, 34800, 2600))) return true;
            if (vars.checkBox("KCD_Fence Deload", new Vector3f(-10270, 31000, 2062), new Vector3f(-9038, 36403, 3000))) return true;
            if (vars.checkBox("MGD_Fence Deload", new Vector3f(-10270, 38460, 2062), new Vector3f(-9038, 42100, 3000))) return true;
            if (vars.checkBox("Counter Deload", new Vector3f(-3750, 28250, 3750), new Vector3f(-3250, 28750, 4000))) return true;
            if (vars.checkBox("Chica Greenroom Deload", new Vector3f(-4700, 52300, 1993), new Vector3f(-4200, 52700, 2500))) return true;
            if (vars.checkBox("Curtain Deload", new Vector3f(5150, 44450, 1960), new Vector3f(5350, 44650, 2100))) return true;
            if (vars.checkBox("Roxy Cutout Deload", new Vector3f(3700, 44300, 1877), new Vector3f(3850, 44700, 1950))) return true;
            if (vars.checkBox("Tunnel Door Deload", new Vector3f(-1500, 49250, 1750), new Vector3f(-1300, 49492, 1900))) return true;
            if (vars.checkBox("Afton Rock Column Deload", new Vector3f(24000, 48000, 2411.5f), new Vector3f(25500, 49500, 2800))) return true;
            if (vars.checkBox("Garage Fence Jump", new Vector3f(18000, 38800, 2411.5f), new Vector3f(19500, 39100, 2800))) return true;
            if (vars.checkBox("Roxy's Eyes Deload", new Vector3f(19500, 50750, 988), new Vector3f(20500, 51150, 1100))) return true;
            if (vars.checkBox("Balloon Deload", new Vector3f(8300, 38000, 2708), new Vector3f(9000, 39000, 3000))) return true;
            if (vars.checkBox("Plant Deload", new Vector3f(9000, 41800, 2708), new Vector3f(10500, 42000, 3000))) return true;
            //Positions
            if (vars.checkBox("Enter Bonnie Bowl", new Vector3f(5970, 37000, 3200), new Vector3f(6280, 37300, 3700))) return true;
            if (vars.checkTime("Enter Daycare", 0, 30)){
                print("12:30AM");
                return true;
            }
            if (vars.checkBox("Enter El Chips", new Vector3f(-8700, 34600, 3200), new Vector3f(-8445, 35700, 3700))) return true;
            if (vars.checkBox("Fazerblast Spiral Stairs", new Vector3f(13100, 31830, 350), new Vector3f(14600, 33330, 750))) return true;
            if (vars.checkBox("Rail Outside Fazerblast", new Vector3f(6800, 35586, 1500), new Vector3f(7550, 35637.4f, 2150))) return true;
            if (vars.checkBox("Exit Afton Elevator", new Vector3f(24170, 49932, -6100), new Vector3f(24465, 49499, 5800))) return true;
            if (vars.checkBox("First Aid Vanessa Cutscene", new Vector3f(4505, 44910, -1140), new Vector3f(4254, 45093, -1429))) return true;
            if (vars.checkBox("Freddy Rail Jump", new Vector3f(2250, 46900, 400), new Vector3f(2850, 47500, 900))) return true;
            if (vars.checkBox("Monty Chase", new Vector3f(2900, 29898.825f, 0), new Vector3f(3400, 29500, 300))) return true;
            if (vars.checkBox("STR-ATR-W Stairs", new Vector3f(5400, 37500, -1230), new Vector3f(6000, 38000, -1150))) return true;
            if (vars.checkBox("STR-LB Stairs", new Vector3f(5000, 24500, 150), new Vector3f(6000, 25000, 400))) return true;
            if (vars.checkBox("Enter West Arcade", new Vector3f(5155, 27863, 2060), new Vector3f(5683, 28139, 2260))){
                vars.pEnWestArcade = false;
                vars.pExWestArcade = true;
                return true;
            }
            if (vars.checkBox("Exit West Arcade", new Vector3f(4793, 30290, 3400), new Vector3f(4528, 30085, 3250))){
                vars.pExWestArcade = false;
                return true;
            }
        }
    #endregion

    #region Ending splits
        //splits based on ending cutscenes
        if (settings["CB_B"] && vars.checkBoxNoBool(new Vector3f(-3194, 19196, 0), new Vector3f(-2911, 18959, 312))
            && vars.checkTimeNoBool(6, 0) && vars.findLeave()){
            if (vars.checkLeave()){
                print("Car Escape Button");
                return true;
            }
        }
        if (settings["E_B"] && vars.checkTimeNoBool(6, 0) &&
        (vars.checkBoxNoBool(new Vector3f(-2238, 19846, 1442), new Vector3f(-1943, 19521, 1746))
        ||vars.checkBoxNoBool(new Vector3f(-1437, 19846, 1442), new Vector3f(-1144, 19521, 1746)))
        && vars.findLeave()){
            if (vars.checkLeave()){
                print("Escape Button");
                return true;
            }
        }
        if (settings["F_B"] && vars.checkBoxNoBool(new Vector3f(-1789, 22700, 3268), new Vector3f(-1595, 22620, 3529))
        && vars.checkTimeNoBool(6, 0) && vars.findLeave()){
            if (vars.checkLeave()){
                print("Fire Escape Button");
                return true;
            }
        }
        if (settings["V_B"] && vars.interactibleName == "vannyButton"
        && !vars.watchers["lastInteractible"].Current && vars.watchers["lastInteractible"].Old){
            print("Vanny Button");
            return true;
        }
        //Afton ending splits are up in "Counter splits" because you have to count each button pressed
        //PQ3 Ending split is handled with other PQ splits, check down there
    #endregion

    #region Item splits
        //Chica's voicebox is weird. Investigate yourself if you want to know more.
        if (vars.watchers["canCollect"].Old.GetType() == typeof(float)
        && vars.watchers["canCollect"].Old >= 0.97f){
            string currentName = vars.GetNameFromFName(vars.watchers["lastInteractible"].Current);
            if (vars.interactibleName == "chicaSewer" && settings["Chica's Voicebox"]
            && vars.CompletedSplits.Add("ChicaVoiceBox_C")){
                return true;
            }
            if (vars.interactibleName == "message"){
                if (settings["ChicaVoiceBox_M"] && currentName.Contains("ChicaVoiceBox")
                && vars.CompletedSplits.Add("ChicaVoiceBox_M")){
                    return true;
                }
                else if (settings["RoxyEyes_M"] && currentName.Contains("RoxyEyes")
                && vars.CompletedSplits.Add("RoxyEyes_M")){
                    return true;
                }
                else if (settings["MontyClaws_M"] && currentName.Contains("MontyClaws")
                && vars.CompletedSplits.Add("MontyClaws_M")){
                    return true;
                }
                else if (settings[currentName] && vars.CompletedSplits.Add(currentName)){
                    return true;
                }
            }
            if (vars.interactibleName == "collectible"){
                if (settings["RoxyEyes_C"] && currentName.Contains("RoxyEyes")
                && vars.CompletedSplits.Add("RoxyEyes_C")){
                    return true;
                }
                else if (settings["MontyClaws_C"] && currentName.Contains("MontyClaws")
                && vars.CompletedSplits.Add("MontyClaws_C")){
                    return true;
                }
                else if (settings[currentName]
                && vars.CompletedSplits.Add(currentName)){
                    return true;
                }
            }
        }
        else if (vars.watchers["canCollect"].Old.GetType() == typeof(bool)
        && vars.interactibleName == "flashlight" && settings["Flashlight"]
        && vars.watchers["canCollect"].Old && !vars.watchers["canCollect"].Current){
            print("Flashlight");
            return true;
        }
        //extraneous items:
        //Fazerblasters
        //Daycare Pass
        //Cameras
        //Repaired Head
        //Badges
        if (vars.watchers["itemCount"].Current > vars.watchers["itemCount"].Old){
            var oldName = vars.GetNameFromFName(vars.watchers["closestInteractibleFName"].Old);
            if (settings["Grey Fazerblaster"] && oldName.Contains("LaserGunCollectible_Game")) return true;
            if (settings["Golden Fazerblaster"] && oldName.Contains("LaserGunCollectible_Prize")) return true;
            if (settings["E_Utility Tunnels"] && oldName == "BB_UtilityStart") return true;
        }
        if (settings["Daycare Pass"] && vars.interactibleName == "daycareMachine"
        && !vars.watchers["canCollect"].Old && vars.watchers["canCollect"].Current) return true;
        if (settings["E_West Arcade"] && vars.checkTime("Repaired Head", 5, 30)){
            print("Repaired Head");
            return true;
        }
        if (settings["Pizzaplex Cameras"] && vars.interactibleName == "cameraButton"
        && !vars.watchers["canCollect"].Old && vars.watchers["canCollect"].Current) return true;
        if (vars.watchers["securityBadgeCount"].Current > vars.watchers["securityBadgeCount"].Old){
            if (settings["Security Badge " + vars.watchers["securityBadgeCount"].Current]){
                print("Security Badge " + vars.watchers["securityBadgeCount"].Current);
                return true;
            }
        }
    #endregion

    #region Time splits
        if (!vars.onMenu && (vars.getHour() != vars.getOldHour() || vars.getMinute() != vars.getOldMinute())){
            if (vars.checkTime("Exit Vents (11:30PM)", -1, 30)){
                print("11:30PM");
                return true;
            }
            if (vars.checkTime("Freddy Recharge (11:45PM)", -1, 45)){
                print("11:45PM");
                return true;
            }
            if (vars.watchers["worldCheck"].Current != 0
            &&  vars.checkTime("Front Entrance Closure (12:00AM)", 0, 0)){
                print("12:00AM");
                return true;
            }
            if (vars.checkTime("Enter Daycare (12:30AM)", 0, 30)){
                print("12:30AM");
                return true;
            }
            if (vars.checkTime("Daycare Nighttime (12:55AM)", 0, 55)){
                print("12:55AM");
                return true;
            }
            if (vars.checkTime("Daycare Vanny Cutscene (1:00AM)", 1, 0)){
                print("1:00AM");
                return true;
            }
            if (vars.checkTime("Mini Music Man Chase (1:15AM)", 1, 15)){
                print("1:15AM");
                return true;
            }
            if (vars.checkTime("Pizzabot (1:30AM)", 1, 30)){
                print("1:30AM");
                return true;
            }
            if (vars.checkTime("White Woman Abduction (2:00AM)", 2, 0)){
                print("2:00AM");
                return true;
            }
            if (vars.checkTime("Dead Fred (2:15AM)", 2, 15)){
                print("2:15AM");
                return true;
            }
            if (vars.checkTime("Backstage Pass (2:30AM)", 2, 30)){
                print("2:30AM");
                return true;
            }
            if (vars.checkTime("Use Showtime Disk (2:45AM)", 2, 45)){
                print("2:45AM");
                return true;
            }
            if (vars.checkTime("Freddy Abduction Recharge (3:00AM)", 3, 0)){
                print("3:00AM");
                return true;
            }
            if (vars.checkTime("Vanessa Repair Cutscene (3:15AM)", 3, 15)){
                print("3:15AM");
                return true;
            }
            if (vars.checkTime("Freddy Power Upgrade (3:30AM)", 3, 30)){
                print("3:30AM");
                return true;
            }
            if (vars.checkTime("Party Pass Recharge (4:00AM)", 4, 0)){
                print("4:00AM");
                return true;
            }
            if (vars.checkTime("Golden Fazerblaster (4:15AM)", 4, 15)){
                print("4:15AM");
                return true;
            }
            if (settings["Monty Mix / Mazercise Key (4:30AM)"]){
                if (vars.watchers["splashScreen"].Current > vars.watchers["splashScreen"].Old){
                    if (vars.checkItem("Monty Mix / Mazercise Key (4:30AM)", new Vector3f(15060, 30205, 3425))
                    || vars.checkItem("Monty Mix / Mazercise Key (4:30AM)", new Vector3f(-17450, 31605, 70))){
                        print("Monty Mix");
                        return true;
                    }
                }
            }
            if (vars.checkTime("Leave Sewers (4:40AM)", 4, 40)){
                print("4:40AM");
                return true;
            }
            if (vars.checkTime("Freddy Upgrade Recharge (5:00AM)", 5, 0)){
                print("5:00AM");
                return true;
            }
            if (vars.checkTime("Damaged Head (5:15AM)", 5, 15)){
                print("5:15AM");
                return true;
            }
            if (vars.checkTime("Repaired Head (5:30AM)", 5, 30)){
                print("5:30AM");
                return true;
            }
            if (vars.checkTime("Finish Roxy Sequence (5:40AM)", 5, 40)){
                print("5:40AM");
                return true;
            }
            if (vars.checkTime("Freddy Eye Upgrade Nighttime (5:50AM)", 5, 50)){
                print("5:50AM");
                return true;
            }
            if (vars.checkTime("Reach Exit Door (6:00AM)", 6, 0)){
                print("6:00AM");
                return true;
            }
        }
    #endregion
    }

    #region Arcade splits
        if (vars.arcade == "BB Arcade" && settings["bb_start"] && vars.CompletedSplits.Add("bb_start")){
            print("bb_start");
            return true;
        }

        if (vars.arcade == "Monty Golf"){
            if (settings["mg_start"] && vars.CompletedSplits.Add("mg_start")){
                print("mg_start");
                return true;
            }
            if (vars.watchers["golfStrokeCount"].Current == 0){
                vars.nHole = 0;
            }
            if (vars.watchers["golfStrokeCount"].Current > vars.watchers["golfStrokeCount"].Old){
                vars.nHole++;
                if (settings["Finish Hole " + vars.nHole.ToString()]){
                    print("Finish Hole " + vars.nHole.ToString());
                    return true;
                }
            }
        }

        #region Princess Quest
            if (vars.arcade == "Princess Quest 1"){
                if (settings["pq1_start"] && vars.CompletedSplits.Add("pq1_start")){
                    return true;
                }
                if (vars.checkPQPosition("pq1_1", 785, 1215,    -160,  160)){
                    print("pq1_1");
                    return true;
                }
                if (vars.checkPQPosition("pq1_2", 1715, 2530,   -160,  160)){
                    print("pq1_2");
                    return true;
                }
                if (vars.checkPQPosition("pq1_3", 3055, 3800,   -160,  160)){
                    print("pq1_3");
                    return true;
                }
                if (vars.checkPQPosition("pq1_4", 1715, 2530,    600,  1425)){
                    print("pq1_4");
                    return true;
                }
                if (vars.checkPQPosition("pq1_5", 1900, 2340,    1860, 2180)){
                    print("pq1_5");
                    return true;
                }
                if (vars.checkPQPosition("pq1_6", 2860, 4695,    1780, 2810)){
                    print("pq1_6");
                    return true;
                }
                if (vars.checkPQPosition("pq1_7", 5220, 6515,    2150, 2780)){
                    print("pq1_7");
                    return true;
                }
                if (vars.checkPQPosition("pq1_8", 950, 1380,     1865, 2300)){
                    print("pq1_8");
                    return true;
                }
                if (vars.checkPQPosition("pq1_9", 2020, 2210,    3425, 5125)){
                    print("pq1_9");
                    return true;
                }
            }
            if (vars.arcade == "Princess Quest 2"){
                if (settings["pq2_start"] && vars.CompletedSplits.Add("pq2_start")){
                    print("pq2_start");
                    return true;
                }
                if (vars.checkPQPosition("pq2_1", 2800, 3250,   -1040, -735)){
                    print("pq2_1");
                    return true;
                }
                if (vars.checkPQPosition("pq2_2", 4300, 4840,   -2800, -2420)){
                    print("pq2_2");
                    return true;
                }
                if (vars.checkPQPosition("pq2_3", 2805, 3155,   -1340, -1110)){
                    print("pq2_3");
                    return true;
                }
                if (vars.checkPQPosition("pq2_4", 2415, 3290,   -3375, -2745)){
                    print("pq2_4");
                    return true;
                }
                if (vars.checkPQPosition("pq2_5", 2955, 3365,    745,   1125)){
                    vars.pq2_8 = true;
                    if (settings["pq2_5"] && vars.CompletedSplits.Add("pq2_5")){
                        print("pq2_5");
                        return true;
                    }
                    print("pq2_5");
                    return true;
                }
                if (vars.checkPQPosition("pq2_6", 1070, 2205,    830,   1470)){
                    print("pq2_6");
                    return true;
                }
                if (vars.checkPQPosition("pq2_7", 5,    1975,   -185,   190)){
                    print("pq2_7");
                    return true;
                }
                if (vars.pq2_8 && vars.checkPQPosition("pq2_8", 2725, 3340, -315, 320)){
                    vars.pq2_8 = false;
                    print("pq2_8");
                    return true;
                }
                if (vars.checkPQPosition("pq2_9", 3920, 4345,    350,   655)){
                    print("pq2_9");
                    return true;
                }
                if (vars.checkPQPosition("pq2_10",4845, 5045,    725,   925)){
                    print("pq2_10");
                    return true;
                }
            }

            if (vars.arcade == "Princess Quest 3"){
                if (settings["pq3_start"] && vars.CompletedSplits.Add("pq3_start")){
                    print("pq3_start");
                    return true;
                }
                if (vars.checkPQPosition("pq3_1",         2195, 2315,     -3625,    -1965)){
                    print("pq3_1");
                    return true;
                }
                if (vars.checkPQPosition("pq3_2",         1705, 2135,     -1340,    -895)){
                    print("pq3_2");
                    return true;
                }
                if (vars.checkPQPosition("pq3_3",         2445, 5210,     -1360,    -990)){
                    print("pq3_3");
                    return true;
                }
                if (vars.checkPQPosition("pq3_4",         4865, 5490,     -210,      365)){
                    print("pq3_4");
                    return true;
                }
                if (vars.checkPQPosition("pq3_5",         500,  1325,     -400,      1045)){
                    print("pq3_5");
                    return true;
                }
                if (vars.checkPQPosition("pq3_6",         1865, 1980,     -1505,    -1380)){
                    print("pq3_6");
                    return true;
                }
                if (vars.checkPQPosition("pq3_7",         1940, 2055,     -260,      0)){
                    print("pq3_7");
                    return true;
                }
                if ((settings["pq3_endArcade"] || settings["pq3_endEndings"])
                && !vars.watchers["pq3Attack"].Old && vars.watchers["pq3Attack"].Current
                && vars.checkPQPositionNoBool(1800, 2200, 1635.34f, 1700)
                && vars.CompletedSplits.Add("pq3_end")){
                    print("pq3_end");
                    return true;
                }
            }
        #endregion

    #endregion

} */
