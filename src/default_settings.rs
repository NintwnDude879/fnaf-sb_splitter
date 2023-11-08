use asr::{
    settings::Gui,
    settings::gui::Title,
};

#[allow(non_snake_case)]
#[derive(Gui)]
#[default = false]
pub struct Settings {
    ///Split Settings
    split_settings: Title,

        #[heading_level = 1]
        ///Arcade Splits
        arcade_splits: Title,

            #[heading_level = 2]
            ///BB Arcade
            bb_arcade: Title,

                ///Start Arcade
                bb_start: bool,

                ///Finish Arcade
                bb_end: bool,

            #[heading_level = 2]
            ///Monty Golf
            mg_arcade: Title,

                ///Start Arcade
                mg_start: bool,

                ///Finish Hole 1
                hole_1: bool,

                ///Finish Hole 2
                hole_2: bool,

                ///Finish Hole 3
                hole_3: bool,

                ///Finish Hole 4
                hole_4: bool,

                ///Finish Hole 5
                hole_5: bool,

                ///Finish Hole 6
                hole_6: bool,

                ///Finish Hole 7
                hole_7: bool,

                ///Finish Hole 8
                hole_8: bool,

                ///Finish Hole 9
                hole_9: bool,

                ///Finish Arcade
                mg_end: bool,

            #[heading_level = 2]
            ///Princess Quest
            pq_arcades: Title,

                #[heading_level = 3]
                ///Princess Quest 1
                pq_1: Title,

                    ///Start Arcade
                    pq_1_start: bool,

                    ///2nd Room
                    pq_1_1: bool,

                    ///Exit Starting Room
                    pq_1_2: bool,

                    ///3rd Room
                    pq_1_3: bool,

                    ///Key Door
                    pq_1_4: bool,

                    ///Crossroads
                    pq_1_5: bool,

                    ///Right Door
                    pq_1_6: bool,

                    ///Enter Graveyard
                    pq_1_7: bool,

                    ///Staircase
                    pq_1_8: bool,

                    ///Final Room
                    pq_1_9: bool,

                    ///Finish Arcade
                    pq_1_end: bool,

                #[heading_level = 3]
                ///Princess Quest 2
                pq_2: Title,

                    ///Start Arcade
                    pq_2_start: bool,

                    ///Exit Starting Room
                    pq_2_1: bool,

                    ///2nd Room
                    pq_2_2: bool,

                    ///Balls Room
                    pq_2_3: bool,

                    ///4th Room
                    pq_2_4: bool,

                    ///Split Puzzle Room
                    pq_2_5: bool,

                    ///Big Torch Room
                    pq_2_6: bool,

                    ///Hallway
                    pq_2_7: bool,

                    ///Big Split Puzzle Room
                    pq_2_8: bool,

                    ///Bedroom
                    pq_2_9: bool,

                    ///Enter Final Room
                    pq_2_10: bool,

                    ///Finish Arcade
                    pq_2_end: bool,


                #[heading_level = 3]
                ///Princess Quest 3
                pq_3: Title,

                    ///Start Arcade
                    pq3_start: bool,

                    ///Hallway
                    pq3_1: bool,

                    ///Hub Room
                    pq3_2: bool,

                    ///Conveyor Room
                    pq3_3: bool,

                    ///Split Puzzle Room (Glitchtrap Plush)
                    pq3_4: bool,

                    ///Flamin' Hot Foxy
                    pq3_5: bool,

                    ///Prize Counter
                    pq3_6: bool,

                    ///Enter Final Area
                    pq3_7: bool,

                    ///Use Key
                    pq3_end: bool,

        #[heading_level = 1]
        ///Counting Splits
        counter_splits: Title,

            #[heading_level = 2]
            ///Daycare Generators
            daycare_gens: Title,

                ///Generator 1
                daycare_gen_1: bool,

                ///Generator 2
                daycare_gen_2: bool,

                ///Generator 3
                daycare_gen_3: bool,

                ///Generator 4
                daycare_gen_4: bool,

                ///Generator 5
                daycare_gen_5: bool,

            #[heading_level = 2]
            ///Fazerblast Flags
            fazer_flags: Title,

                ///Flag 1
                fazer_flag_1: bool,

                ///Flag 2
                fazer_flag_2: bool,

                ///Flag 3
                fazer_flag_3: bool,

            #[heading_level = 2]
            ///Monty Bucket Balls
            monty_bucket: Title,

                ///10 Balls
                monty_balls_10: bool,

                ///20 Balls
                monty_balls_20: bool,

                ///25 Balls (bucket full)
                monty_balls_25: bool,

            #[heading_level = 2]
            ///Sewer Generators
            sewer_gens: Title,

                ///Generator 1
                sewer_gen_1: bool,

                ///Generator 2
                sewer_gen_2: bool,

                ///Generator 3
                sewer_gen_3: bool,

            #[heading_level = 2]
            ///West Arcade Generators
            west_arcade_gens: Title,

                ///Generator 1
                west_arcade_gen_1: bool,

                ///Generator 2
                west_arcade_gen_2: bool,

                ///Generator 3
                west_arcade_gen_3: bool,

                ///Generator 4
                west_arcade_gen_4: bool,

                ///Generator 5
                west_arcade_gen_5: bool,

        #[heading_level = 1]
        ///Deload Splits
        deload_splits: Title,

            ///Backstage
            #[heading_level = 2]
            backstage_deloads: Title,

                ///Foxy Cutout Deload
                foxy_cutout_deload: bool,

            ///Daycare
            #[heading_level = 2]
            daycare_deloads: Title,

                ///Arcade Deload
                arcade_deload: bool,

                ///Theatre Deload
                theatre_deload: bool,

            ///Kid's Cove Sublobby
            #[heading_level = 2]
            kids_cove_lobby_deloads: Title,

                ///Fence Deload
                kids_cove_fence_deload: bool,


            ///Monty Golf Sublobby
            #[heading_level = 2]
            monty_golf_lobby_deloads: Title,

                ///Fence Deload
                monty_golf_fence_deload: bool,

            ///Prize Counter
            #[heading_level = 2]
            prize_counter_deloads: Title,

                ///Counter Deload
                counter_deload: bool,

            ///Rockstar Row
            #[heading_level = 2]
            rockstar_row_deloads: Title,

                ///Chica Greenroom Deload
                chica_greenroom_deload: bool,

                ///Curtain Deload
                curtain_deload: bool,

                ///Roxy Cutout Deload
                roxy_cutout_deload: bool,

                ///Tunnel Door Deload
                tunnel_door_deload: bool,

            ///Roxy Raceway
            #[heading_level = 2]
            roxy_raceway_deloads: Title,

                ///Afton Rock Column Deload
                afton_rocks_deload: bool,

                ///Garage Fencejump Deload
                garage_fence_deload: bool,

                ///Roxy's Eyes Deload
                roxy_eyes_deload: bool,

            ///Roxy Raceway Sublobby
            #[heading_level = 2]
            roxy_raceway_lobby_deloads: Title,

                ///Balloon Deload
                balloon_deload: bool,

            ///Roxy Salon
            #[heading_level = 2]
            roxy_salon_deloads: Title,

                ///Plant Deload
                plant_deload: bool,

        #[heading_level = 1]
        ///Ending Splits
        ending_splits: Title,

            ///Vanny Ending
            vanny_ending: bool,

            ///Car Battery Ending
            car_battery_ending: bool,

            ///Escape Ending
            escape_ending: bool,

            ///Fire Escape Ending
            fire_escape_ending: bool,

            ///Princess Quest Ending
            pq_ending: bool,

            #[heading_level = 2]
            ///Afton Ending
            afton_ending: Title,

                ///Button 1
                afton_button_1: bool,

                ///Button 2
                afton_button_2: bool,

                ///Button 3
                afton_button_3: bool,

                ///Button 4
                afton_button_4: bool,

                ///Button 5
                afton_button_5: bool,

                ///Button 6
                afton_button_6: bool,

                ///Button 7
                afton_button_7: bool,

                ///Button 8 / Ending
                afton_button_8: bool,

        #[heading_level = 1]
        ///Item Splits
        item_splits: Title,

            #[heading_level = 2]
            ///General Items
            general_items: Title,

                #[heading_level = 3]
                ///Collectibles
                collectibles: Title,

                    #[heading_level = 4]
                    ///Backstage
                    backstage_collectibles: Title,

                        ///El Chip Pinata
                        CommonCollectible_13: bool,

                        ///Freddy Icon Shirt
                        c_shirt_freddyicon: bool,

                        ///Glam Chica Figure
                        UncommonCollectible_7: bool,

                    #[heading_level = 4]
                    ///Basement Kitchen
                    basement_kitchen_collectibles: Title,

                        ///Freddy Magnet
                        c_magnet_freddy: bool,

                        ///Golden Chica
                        c_golden_chica: bool,

                    #[heading_level = 4]
                    ///Bonnie Bowl
                    bonnie_bowl_collectibles: Title,

                        ///Bonnie Plush
                        c_plush_bonnie: bool,

                        ///Golden Monty
                        c_golden_monty: bool,

                    #[heading_level = 4]
                    ///Chica's Bakery
                    chica_bakery_collectibles: Title,

                        ///Monty Magnet
                        c_magnet_monty: bool,

                    #[heading_level = 4]
                    ///Daycare
                    daycare_collectibles: Title,

                        ///Freddy Mask
                        RareCollectible_1: bool,

                        ///Frozen Chica Treat
                        CommonCollectible_4: bool,

                        ///Glam Freddy Figure
                        UncommonCollectible_6: bool,

                        ///Golden Moon
                        c_golden_moon: bool,

                        ///Old Poster
                        c_poster1: bool,

                        ///Plush Baby
                        c_plush_baby: bool,

                    #[heading_level = 4]
                    ///El Chips
                    el_chips_collectibles: Title,

                        ///Chica Balloon
                        CommonCollectible_14: bool,

                    #[heading_level = 4]
                    ///Fazerblast
                    fazerblast_collectibles: Title,

                        ///Freddy Balloon
                        CommonCollectible_15: bool,

                        ///Old Poster
                        c_poster4: bool,

                        ///Space Chica Keychain
                        c_key_spchica: bool,

                        ///Space Roxy Keychain
                        c_key_sproxy: bool,

                    #[heading_level = 4]
                    ///Fazerblast Sublobby
                    fazer_lobby_collectibles: Title,

                        ///Freddy Pinata
                        CommonCollectible_10: bool,

                        ///Space Freddy Keychain
                        c_key_spfreddy: bool,

                    #[heading_level = 4]
                    ///Kid's Cove Sublobby
                    kids_cove_lobby_collectibles: Title,

                        ///Golden Sun
                        c_golden_sun: bool,

                        ///Moon Plush
                        UncommonCollectible_5: bool,

                    #[heading_level = 4]
                    ///Laundry
                    laundry_collectibles: Title,

                        ///Glam Roxy Plush
                        UncommonCollectible_1: bool,

                        ///Old Poster
                        c_poster2: bool,

                        ///Star Shirt
                        c_shirt_star: bool,

                    #[heading_level = 4]
                    ///Lobby
                    lobby_collectibles: Title,

                        ///Chica Name Shirt
                        CommonCollectible_8: bool,

                        ///Cupcake Pinata
                        CommonCollectible_18: bool,

                        ///Freddy Name Shirt
                        CommonCollectible_6: bool,

                        ///Glam Chica Plush
                        UncommonCollectible_2: bool,

                        ///Sun Plush
                        CommonCollectible_1: bool,

                    #[heading_level = 4]
                    ///Main Atrium
                    atrium_collectibles: Title,

                        ///Monty Pinata
                        CommonCollectible_20: bool,

                        ///Roxy Name Shirt
                        CommonCollectible_9: bool,

                    #[heading_level = 4]
                    ///Monty Golf
                    monty_golf_collectibles: Title,

                        ///Go Kart
                        c_gokart: bool,

                        ///Tragedy Mask
                        c_mask_tragedy: bool,

                    #[heading_level = 4]
                    ///Monty Golf Sublobby
                    monty_golf_lobby_collectibles: Title,

                        ///Frozen Monty Treat
                        CommonCollectible_2: bool,

                        ///Glam Monty Figure
                        UncommonCollectible_8: bool,

                    #[heading_level = 4]
                    ///Parts & Service
                    parts_n_service_collectibles: Title,

                        ///Glam Roxy Figure
                        UncommonCollectible_9: bool,

                    #[heading_level = 4]
                    ///Prize Counter
                    prize_counter_collectibles: Title,

                        ///Frozen Freddy Treat
                        CommonCollectible_3: bool,

                        ///Monty Mask
                        RareCollectible_4: bool,

                    #[heading_level = 4]
                    ///Rockstar Row
                    rockstar_row_collectibles: Title,

                        ///Chica Mask
                        RareCollectible_2: bool,

                        ///Four Block Shirt
                        c_shirt_four: bool,

                        ///Golden Freddy
                        c_golden_freddy: bool,

                        ///Monty Name Shirt
                        CommonCollectible_12: bool,

                        ///Pizzaplex Logo Shirt
                        CommonCollectible_7: bool,

                    #[heading_level = 4]
                    ///Roxy Raceway
                    roxy_raceway_collectibles: Title,

                        ///Monty Balloon
                        CommonCollectible_16: bool,

                        ///Space Monty Keychain
                        c_key_spmonty: bool,

                    #[heading_level = 4]
                    ///Roxy Raceway Sublobby
                    roxy_raceway_lobby_collectibles: Title,

                        ///Frozen Roxy Treat
                        CommonCollectible_5: bool,

                        ///Roxy Balloon
                        CommonCollectible_17: bool,

                        ///Roxy Mask
                        RareCollectible_3: bool,

                    #[heading_level = 4]
                    ///Roxy Salon
                    roxy_salon_collectibles: Title,

                        ///Roxy Magnet
                        c_magnet_roxy: bool,

                        ///Roxy Pinata
                        CommonCollectible_11: bool,

                    #[heading_level = 4]
                    ///Salads & Sides
                    salads_n_sides_collectibles: Title,

                        ///Freddy Face Shirt
                        c_shirt_freddyface: bool,

                        ///Pinata
                        c_pinata_general: bool,

                    #[heading_level = 4]
                    ///Sewers
                    sewers_collectibles: Title,

                        ///Old Poster
                        c_poster3: bool,

                    #[heading_level = 4]
                    ///Utility Tunnels
                    utility_tunnels_collectibles: Title,

                        ///Chica Pinata
                        CommonCollectible_19: bool,

                        ///Comedy Mask
                        c_mask_comedy: bool,

                        ///Foxy Plush
                        c_plush_foxy: bool,

                        ///Glam Freddy Plush
                        UncommonCollectible_3: bool,

                        ///Golden Roxy
                        c_golden_roxy: bool,

                        ///Moon Figure
                        UncommonCollectible_11: bool,

                        ///Sun Figure
                        UncommonCollectible_10: bool,

                    #[heading_level = 4]
                    ///Warehouse
                    warehouse_collectibles: Title,

                        ///Nightmare Plush
                        c_plush_night: bool,

                    #[heading_level = 4]
                    ///West Arcade
                    west_arcade_collectibles: Title,

                        ///Glam Monty Plush
                        UncommonCollectible_4: bool,

                #[heading_level = 3]
                ///Equipment
                equipment: Title,

                    #[heading_level = 4]
                    ///Backstage
                    backstage_equipment: Title,

                        ///Backstage Pass
                        BackstagePass: bool,

                        ///Flashlight Upgrade
                        FlashlightUpgrade_3: bool,

                    #[heading_level = 4]
                    ///Basement Kitchen
                    basement_kitchen_equipment: Title,

                        ///Freddy Fizzy Faz
                        GregoryUpgrade_Stamina_3: bool,

                    #[heading_level = 4]
                    ///Bonnie Bowl
                    bonnie_bowl_equipment: Title,

                        ///Monty Mystery Mix
                        MontyMysteryMix: bool,

                    #[heading_level = 4]
                    ///Chica's Bakery
                    chica_bakery_equipment: Title,

                        ///Hoodie
                        GregoryUpgrade_Stealth: bool,

                    #[heading_level = 4]
                    ///Daycare
                    daycare_equipment: Title,

                        ///Flashlight
                        Flashlight: bool,

                        ///Flashlight Upgrade
                        FlashlightUpgrade_2: bool,

                        ///Mazercise Control Key
                        MazerciseControlKey: bool,

                    #[heading_level = 4]
                    ///El Chips
                    el_chips_equipment: Title,

                        ///Monty Fizzy Faz
                        GregoryUpgrade_Stamina_4: bool,

                    #[heading_level = 4]
                    ///Fazerblast
                    fazerblast_equipment: Title,

                        ///Bowling Pass
                        BowlingTicket: bool,

                        ///Golden Fazerblaster
                        gold_blaster: bool,

                        ///Grey Fazerblaster
                        grey_blaster: bool,

                    #[heading_level = 4]
                    ///Lobby
                    lobby_equipment: Title,

                        ///Chica Fizzy Faz
                        GregoryUpgrade_Stamina_2: bool,

                        ///Daycare Pass
                        daycare_pass: bool,

                        ///Entrance Pass
                        CompPass: bool,

                        ///Mr. Hippo Magnet
                        MrHippoMagnet: bool,

                        ///Screwdriver
                        ScrewDriver: bool,

                    #[heading_level = 4]
                    ///Main Atrium
                    atrium_equipment: Title,

                        ///Freddy Upgrade
                        FreddyUpgrade_1: bool,

                        ///Mapbot's Map
                        Map: bool,

                    #[heading_level = 4]
                    ///Monty Golf
                    monty_golf_equipment: Title,

                        ///Fazcam
                        FlashBeacon: bool,

                        ///Flashlight Upgrade
                        FlashlightUpgrade_1: bool,

                        ///Mazercise Pass
                        MazerciseTicket: bool,

                        ///Monty's Claws
                        MontyClaws_C: bool,

                    #[heading_level = 4]
                    ///Rockstar Row
                    rockstar_row_equipment: Title,

                        ///Fazwatch
                        Fazwatch: bool,

                        ///Party Pass
                        PartyPassChicaRoom: bool,

                        ///Photo Pass
                        PhotoPass: bool,

                    #[heading_level = 4]
                    ///Roxy Raceway
                    roxy_raceway_equipment: Title,

                        ///Damaged Head
                        DamagedHead: bool,

                        ///Dance Pass
                        DancePass: bool,

                        ///Freddy Upgrade
                        FreddyUpgrade_2: bool,

                        ///Roxy Fizzy Faz
                        GregoryUpgrade_Stamina_1: bool,

                        ///Roxy's Eyes
                        RoxyEyes_C: bool,

                    #[heading_level = 4]
                    ///Roxy Salon
                    roxy_salon_equipment: Title,

                        ///Shoes
                        GregoryUpgrade_Shoes: bool,

                    #[heading_level = 4]
                    ///Sewers
                    sewers_equipment: Title,

                        ///Chica's Voicebox
                        chica_voicebox: bool,

                    #[heading_level = 4]
                    ///Utility Tunnels
                    utility_tunnels_equipment: Title,

                        ///Mapbot's Map
                        UtilityHallwayMap: bool,

                        ///Pizzaplex Cameras
                        pizzaplex_cameras: bool,

                    #[heading_level = 4]
                    ///West Arcade
                    west_arcade_equipment: Title,

                        ///Repaired Head
                        repaired_head: bool,


                #[heading_level = 3]
                ///Message Bags
                message_bags: Title,

                    ///AC Inspection
                    LilMusicMan1: bool,

                    ///All Staff Meeting
                    StaffParty2: bool,

                    ///AR-CADE MAINT LOG
                    ArcadeGlitches3: bool,

                    ///ARCADE CONSPIRACY
                    ArcadeGlitches8: bool,

                    ///BBW MAINT LOG
                    ArcadeGlitches1: bool,

                    ///BEHIND THE MAZE
                    MontyBoss2: bool,

                    ///BETTER EMPLOYEES
                    RayMessage: bool,

                    ///CFF MAINT LOG
                    ArcadeGlitches2: bool,

                    ///Chasing Cars
                    RoxyBoss1: bool,

                    ///CHICA REPORT
                    ChicaBoss4: bool,

                    ///CHICA UPGRADE
                    ChicaVoiceBox_M: bool,

                    ///ROXY UPGRADE
                    RoxyEyes_M: bool,

                    ///MONTY UPGRADE
                    MontyClaws_M: bool,

                    ///COMPACTOR INSTRUCTIONS
                    ChicaBoss3: bool,

                    ///Drink Fizzy Faz!!!
                    StaminaBoosters: bool,

                    ///Easy Money
                    UpgradeMachine1: bool,

                    ///False Alarm
                    _911Call: bool,

                    ///Food Storage
                    KitchenStaffNote: bool,

                    ///HI DAVE
                    Complaint2: bool,

                    ///Hide the Mix
                    ChicaBoss1: bool,

                    ///IT IS HAPPENING AGAIN
                    Complaint3: bool,

                    ///Job Security
                    MontyBoss3: bool,

                    ///MARKED FOR DELETION
                    Vanessa2: bool,

                    ///MISSING
                    BonnieMissing3: bool,

                    ///Monty Mischief
                    MontyBoss4: bool,

                    ///NIGHT SHIFT
                    NightShift: bool,

                    ///Night Terrors
                    NightTerrors: bool,

                    ///No Flash Photography
                    FlashCam: bool,

                    ///NO QUESTIONS ASKED
                    Vanessa1: bool,

                    ///OLD ELEVATOR
                    Sinkhole3: bool,

                    ///OUT OF ORDER
                    ArcadeGlitches6: bool,

                    ///Party Foul
                    Freddy1: bool,

                    ///PINK SLIP
                    Staff1: bool,

                    ///POWER DRAIN
                    Sinkhole2: bool,

                    ///PQ1 MAINT LOG
                    ArcadeGlitches4: bool,

                    ///PQ2 MAINT LOG
                    ArcadeGlitches5: bool,

                    ///QUESTION
                    StaffParty1: bool,

                    ///Re-Theme
                    BonnieMissing2: bool,

                    ///RECYCLED PIZZA?
                    Complaint1: bool,

                    ///RED FLAG
                    ArcadeGlitches7: bool,

                    ///SAFETY CHECK
                    ChicaBoss2: bool,

                    ///SINKHOLE
                    Sinkhole1: bool,

                    ///SORE WINNER
                    RoxyBoss2: bool,

                    ///Stolen Property
                    LilMusicMan2: bool,

                    ///TEST DRIVERS WANTED
                    RoxyBoss3: bool,

                    ///THE ANSWER
                    MontyBoss1: bool,

                    ///TORN PAPER
                    TornPaper: bool,

                    ///ULTIMATE PARTY HOST
                    DJMM: bool,

                    ///Understudy
                    BonnieMissing1: bool,

                #[heading_level = 3]
                ///Special Collectibles
                special_collectibles: Title,

                    ///Chica Magnet (West Arcade / Chica's Cakes)
                    c_magnet_chica: bool,

                #[heading_level = 3]
                ///Retro CDs
                retro_cds: Title,

                    ///Backstage Podium
                    RetroCDCollectible_9: bool,

                    ///Bonnie Bowl
                    RetroCDCollectible_8: bool,

                    ///Chica's Bakery
                    RetroCDCollectible_3: bool,

                    ///West Atrium Stage
                    RetroCDCollectible_15: bool,

                    ///Fazerblast
                    RetroCDCollectible_7: bool,

                    ///Kid's Cove
                    RetroCDCollectible_4: bool,

                    ///Main Atrium
                    RetroCDCollectible_10: bool,

                    ///Mazercise
                    RetroCDCollectible_14: bool,

                    ///Monty Golf
                    RetroCDCollectible_13: bool,

                    ///Rockstar Row Foxy
                    RetroCDCollectible_1: bool,

                    ///Rockstar Row Helpy
                    RetroCDCollectible_11: bool,

                    ///Roxy Raceway
                    RetroCDCollectible_12: bool,

                    ///Roxy Salon
                    RetroCDCollectible_5: bool,

                    ///Utility Tunnels Couch
                    RetroCDCollectible_2: bool,

                    ///Utility Tunnels Foxy Plush
                    RetroCDCollectible_6: bool,

                    ///West Arcade
                    RetroCDCollectible_16: bool,

            #[heading_level = 2]
            ///Security Badges
            security_badges: Title,

                ///Security Badge 1
                security_badge_1: bool,

                ///Security Badge 2
                security_badge_2: bool,

                ///Security Badge 3
                security_badge_3: bool,

                ///Security Badge 4
                security_badge_4: bool,

                ///Security Badge 5
                security_badge_5: bool,

                ///Security Badge 6
                security_badge_6: bool,

                ///Security Badge 7
                security_badge_7: bool,

                ///Security Badge 8
                security_badge_8: bool,

        #[heading_level = 1]
        ///Positional Splits
        positional_splits: Title,

            #[heading_level = 2]
            ///Bonnie Bowl
            bonnie_bowl_position: Title,

                ///Enter Bonnie Bowl
                enter_bonnie_bowl: bool,

            #[heading_level = 2]
            ///Daycare
            daycare_position: Title,

                ///Enter Daycare
                enter_daycare: bool,

            #[heading_level = 2]
            ///El Chips
            el_chips_position: Title,

                ///Enter El Chips
                enter_el_chips: bool,

            #[heading_level = 2]
            ///Fazerblast
            fazerblast_position: Title,

                ///Fazerblast Spiral Stairs
                fazerblast_stairs: bool,

            #[heading_level = 2]
            ///Fazerblast Lobby
            fazerblast_lobby_position: Title,

                ///Rail Outside Fazerblast
                fazerblast_lobby_rail: bool,

            #[heading_level = 2]
            ///Underground / Afton Cave
            afton_position: Title,

                ///Exit Afton Elevator
                afton_elevator_position: bool,

            #[heading_level = 2]
            ///Utility Tunnels
            utility_tunnels_position: Title,

                ///First Aid Vanessa Cutscene
                first_aid_vanessa_scene: bool,

                ///Freddy Rail Jump
                freddy_rail_jump: bool,

                ///Monty Chase Start
                monty_chase_start: bool,

                ///STR-ATR-W Stairs
                str_atr_w_stairs: bool,

                ///STR-LB Stairs
                str_lb_stairs: bool,

            #[heading_level = 2]
            ///West Arcade
            west_arcade_position: Title,

                ///Enter West Arcade
                enter_west_arcade: bool,

                ///Exit West Arcade
                exit_west_arcade: bool,

        #[heading_level = 1]
        ///Time Splits
        time_splits: Title,

            ///Exit Vents (11:30PM)
            time_11_30: bool,

            ///Freddy Recharge (11:45PM)
            time_11_45: bool,

            ///Front Entrance Closure(12:00AM)
            time_12_00: bool,

            ///Enter Daycare (12:30AM)
            time_12_30: bool,

            ///Daycare Nighttime (12:55AM)
            time_12_55: bool,

            ///Daycare Vanny Cutscene (1:00AM)
            time_1_00: bool,

            ///Mini Music Man Chase (1:15AM)
            time_1_15: bool,

            ///Pizzabot (1:30AM)
            time_1_30: bool,

            ///White Woman Jumpscare (2:00AM)
            time_2_00: bool,

            ///Dead Fred (2:15AM)
            time_2_15: bool,

            ///Backstage Passs (2:30AM)
            time_2_30: bool,

            ///Use Showtime Disk (2:45AM)
            time_2_45: bool,

            ///Freddy Abduction Recharge (3:00AM)
            time_3_00: bool,

            ///Vanessa Repair Cutscene (3:15AM)
            time_3_15: bool,

            ///Freddy Power Upgrade (3:30AM)
            time_3_30: bool,

            ///Party Pass Recharge (4:00AM)
            time_4_00: bool,

            ///Golden Fazerblaster (4:15AM)
            time_4_15: bool,

            ///Monty Mix / Mazercise Key (4:30AM)
            time_4_30: bool,

            ///Leave Sewers (4:40AM)
            time_4_40: bool,

            ///Freddy Upgrade Recharge (5:00AM)
            time_5_00: bool,

            ///Damaged Head (5:15AM)
            time_5_15: bool,

            ///Repaired Head (5:30AM)
            time_5_30: bool,

            ///Finish Roxy Sequence (5:40AM)
            time_5_40: bool,

            ///Freddy Eye Upgrade Nighttime (5:50AM)
            time_5_50: bool,

            ///Reach Exit Door (6:00AM)
            time_6_00: bool,

    ///Non-Split Functionality
    igt_settings: Title,

        ///Elevator Pauses
        #[default = true]
        elevator_pause: bool,

        ///Stop Timer On Menu
        #[default = true]
        menu_pause: bool,

        ///Stop Timer When Loading
        #[default = true]
        load_pause: bool,

        ///Stop Timer When Paused
        #[default = true]
        pause_pause: bool,

        ///Reset On New Game
        #[default = true]
        reset_settings: bool,

        ///Unsupported version warning
        #[default = true]
        unknown_version_warn: bool,
}
