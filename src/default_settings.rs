use asr::{
    settings::Gui,
    settings::gui::Title,
};

#[derive(Gui)]
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
                #[default = false]
                bb_start: bool,

                ///Finish Arcade
                #[default = false]
                bb_end: bool,

            #[heading_level = 2]
            ///Monty Golf
            mg_arcade: Title,

                ///Start Arcade
                #[default = false]
                mg_start: bool,

                ///Finish Hole 1
                #[default = false]
                hole_1: bool,

                ///Finish Hole 2
                #[default = false]
                hole_2: bool,

                ///Finish Hole 3
                #[default = false]
                hole_3: bool,

                ///Finish Hole 4
                #[default = false]
                hole_4: bool,

                ///Finish Hole 5
                #[default = false]
                hole_5: bool,

                ///Finish Hole 6
                #[default = false]
                hole_6: bool,

                ///Finish Hole 7
                #[default = false]
                hole_7: bool,

                ///Finish Hole 8
                #[default = false]
                hole_8: bool,

                ///Finish Hole 9
                #[default = false]
                hole_9: bool,

                ///Finish Arcade
                #[default = false]
                mg_end: bool,

            #[heading_level = 2]
            ///Princess Quest
            pq_arcades: Title,

                #[heading_level = 3]
                ///Princess Quest 1
                pq_1: Title,

                    ///Start Arcade
                    #[default = false]
                    pq_1_start: bool,

                    ///2nd Room
                    #[default = false]
                    pq_1_1: bool,

                    ///Exit Starting Room
                    #[default = false]
                    pq_1_2: bool,

                    ///3rd Room
                    #[default = false]
                    pq_1_3: bool,

                    ///Key Door
                    #[default = false]
                    pq_1_4: bool,

                    ///Crossroads
                    #[default = false]
                    pq_1_5: bool,

                    ///Right Door
                    #[default = false]
                    pq_1_6: bool,

                    ///Enter Graveyard
                    #[default = false]
                    pq_1_7: bool,

                    ///Staircase
                    #[default = false]
                    pq_1_8: bool,

                    ///Final Room
                    #[default = false]
                    pq_1_9: bool,

                    ///Finish Arcade
                    #[default = false]
                    pq_1_end: bool,

                #[heading_level = 3]
                ///Princess Quest 2
                pq_2: Title,

                    ///Start Arcade
                    #[default = false]
                    pq_2_start: bool,

                    ///Exit Starting Room
                    #[default = false]
                    pq_2_1: bool,

                    ///2nd Room
                    #[default = false]
                    pq_2_2: bool,

                    ///Balls Room
                    #[default = false]
                    pq_2_3: bool,

                    ///4th Room
                    #[default = false]
                    pq_2_4: bool,

                    ///Split Puzzle Room
                    #[default = false]
                    pq_2_5: bool,

                    ///Big Torch Room
                    #[default = false]
                    pq_2_6: bool,

                    ///Hallway
                    #[default = false]
                    pq_2_7: bool,

                    ///Big Split Puzzle Room
                    #[default = false]
                    pq_2_8: bool,

                    ///Bedroom
                    #[default = false]
                    pq_2_9: bool,

                    ///Enter Final Room
                    #[default = false]
                    pq_2_10: bool,

                    ///Finish Arcade
                    #[default = false]
                    pq_2_end: bool,


                #[heading_level = 3]
                ///Princess Quest 3
                pq_3: Title,

                    ///Start Arcade
                    #[default = false]
                    pq3_start: bool,

                    ///Hallway
                    #[default = false]
                    pq3_1: bool,

                    ///Hub Room
                    #[default = false]
                    pq3_2: bool,

                    ///Conveyor Room
                    #[default = false]
                    pq3_3: bool,

                    ///Split Puzzle Room (Glitchtrap Plush)
                    #[default = false]
                    pq3_4: bool,

                    ///Flamin' Hot Foxy
                    #[default = false]
                    pq3_5: bool,

                    ///Prize Counter
                    #[default = false]
                    pq3_6: bool,

                    ///Enter Final Area
                    #[default = false]
                    pq3_7: bool,

                    ///Use Key
                    #[default = false]
                    pq3_end: bool,

        #[heading_level = 1]
        ///Counting Splits
        counter_splits: Title,

            #[heading_level = 2]
            ///Daycare Generators
            daycare_gens: Title,

                ///Generator 1
                #[default = false]
                daycare_gen_1: bool,

                ///Generator 2
                #[default = false]
                daycare_gen_2: bool,

                ///Generator 3
                #[default = false]
                daycare_gen_3: bool,

                ///Generator 4
                #[default = false]
                daycare_gen_4: bool,

                ///Generator 5
                #[default = false]
                daycare_gen_5: bool,

            #[heading_level = 2]
            ///Fazerblast Flags
            fazer_flags: Title,

                ///Flag 1
                #[default = false]
                fazer_flag_1: bool,

                ///Flag 2
                #[default = false]
                fazer_flag_2: bool,

                ///Flag 3
                #[default = false]
                fazer_flag_3: bool,

            #[heading_level = 2]
            ///Monty Bucket Balls
            monty_bucket: Title,

                ///10 Balls
                #[default = false]
                monty_balls_10: bool,

                ///20 Balls
                #[default = false]
                monty_balls_20: bool,

                ///25 Balls (bucket full)
                #[default = false]
                monty_balls_25: bool,

            #[heading_level = 2]
            ///Sewer Generators
            sewer_gens: Title,

                ///Generator 1
                #[default = false]
                sewer_gen_1: bool,

                ///Generator 2
                #[default = false]
                sewer_gen_2: bool,

                ///Generator 3
                #[default = false]
                sewer_gen_3: bool,

            #[heading_level = 2]
            ///West Arcade Generators
            west_arcade_gens: Title,

                ///Generator 1
                #[default = false]
                west_arcade_gen_1: bool,

                ///Generator 2
                #[default = false]
                west_arcade_gen_2: bool,

                ///Generator 3
                #[default = false]
                west_arcade_gen_3: bool,

                ///Generator 4
                #[default = false]
                west_arcade_gen_4: bool,

                ///Generator 5
                #[default = false]
                west_arcade_gen_5: bool,

        #[heading_level = 1]
        ///Deload Splits
        deload_splits: Title,

            ///Backstage
            #[heading_level = 2]
            backstage_deloads: Title,

                ///Foxy Cutout Deload
                #[default = false]
                foxy_cutout_deload: bool,

            ///Daycare
            #[heading_level = 2]
            daycare_deloads: Title,

                ///Arcade Deload
                #[default = false]
                arcade_deload: bool,

                ///Theatre Deload
                #[default = false]
                theatre_deload: bool,

            ///Kid's Cove Sublobby
            #[heading_level = 2]
            kids_cove_lobby_deloads: Title,

                ///Fence Deload
                #[default = false]
                kids_cove_fence_deload: bool,


            ///Monty Golf Sublobby
            #[heading_level = 2]
            monty_golf_lobby_deloads: Title,

                ///Fence Deload
                #[default = false]
                monty_golf_fence_deload: bool,

            ///Prize Counter
            #[heading_level = 2]
            prize_counter_deloads: Title,

                ///Counter Deload
                #[default = false]
                counter_deload: bool,

            ///Rockstar Row
            #[heading_level = 2]
            rockstar_row_deloads: Title,

                ///Chica Greenroom Deload
                #[default = false]
                chica_greenroom_deload: bool,

                ///Curtain Deload
                #[default = false]
                curtain_deload: bool,

                ///Roxy Cutout Deload
                #[default = false]
                roxy_cutout_deload: bool,

                ///Tunnel Door Deload
                #[default = false]
                tunnel_door_deload: bool,

            ///Roxy Raceway
            #[heading_level = 2]
            roxy_raceway_deloads: Title,

                ///Afton Rock Column Deload
                #[default = false]
                afton_rocks_deload: bool,

                ///Garage Fencejump Deload
                #[default = false]
                garage_fence_deload: bool,

                ///Roxy's Eyes Deload
                #[default = false]
                roxy_eyes_deload: bool,

            ///Roxy Raceway Sublobby
            #[heading_level = 2]
            roxy_raceway_lobby_deloads: Title,

                ///Balloon Deload
                #[default = false]
                balloon_deload: bool,

            ///Roxy Salon
            #[heading_level = 2]
            roxy_salon_deloads: Title,

                ///Plant Deload
                #[default = false]
                plant_deload: bool,

        #[heading_level = 1]
        ///Ending Splits
        ending_splits: Title,

            ///Vanny Ending
            #[default = false]
            vanny_ending: bool,

            ///Car Battery Ending
            #[default = false]
            car_battery_ending: bool,

            ///Escape Ending
            #[default = false]
            escape_ending: bool,

            ///Fire Escape Ending
            #[default = false]
            fire_escape_ending: bool,

            ///Princess Quest Ending
            #[default = false]
            pq_ending: bool,

            #[heading_level = 2]
            ///Afton Ending
            afton_ending: Title,

                ///Button 1
                #[default = false]
                afton_button_1: bool,

                ///Button 2
                #[default = false]
                afton_button_2: bool,

                ///Button 3
                #[default = false]
                afton_button_3: bool,

                ///Button 4
                #[default = false]
                afton_button_4: bool,

                ///Button 5
                #[default = false]
                afton_button_5: bool,

                ///Button 6
                #[default = false]
                afton_button_6: bool,

                ///Button 7
                #[default = false]
                afton_button_7: bool,

                ///Button 8 / Ending
                #[default = false]
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
                        #[default = false]
                        CommonCollectible_13: bool,

                        ///Freddy Icon Shirt
                        #[default = false]
                        c_shirt_freddyicon: bool,

                        ///Glam Chica Figure
                        #[default = false]
                        UncommonCollectible_7: bool,

                    #[heading_level = 4]
                    ///Basement Kitchen
                    basement_kitchen_collectibles: Title,

                        ///Freddy Magnet
                        #[default = false]
                        c_magnet_freddy: bool,

                        ///Golden Chica
                        #[default = false]
                        c_golden_chica: bool,

                    #[heading_level = 4]
                    ///Bonnie Bowl
                    bonnie_bowl_collectibles: Title,

                        ///Bonnie Plush
                        #[default = false]
                        c_plush_bonnie: bool,

                        ///Golden Monty
                        #[default = false]
                        c_golden_monty: bool,

                    #[heading_level = 4]
                    ///Chica's Bakery
                    chica_bakery_collectibles: Title,

                        ///Monty Magnet
                        #[default = false]
                        c_magnet_monty: bool,

                    #[heading_level = 4]
                    ///Daycare
                    daycare_collectibles: Title,

                        ///Freddy Mask
                        #[default = false]
                        RareCollectible_1: bool,

                        ///Frozen Chica Treat
                        #[default = false]
                        CommonCollectible_4: bool,

                        ///Glam Freddy Figure
                        #[default = false]
                        UncommonCollectible_6: bool,

                        ///Golden Moon
                        #[default = false]
                        c_golden_moon: bool,

                        ///Old Poster
                        #[default = false]
                        c_poster1: bool,

                        ///Plush Baby
                        #[default = false]
                        c_plush_baby: bool,

                    #[heading_level = 4]
                    ///El Chips
                    el_chips_collectibles: Title,

                        ///Chica Balloon
                        #[default = false]
                        CommonCollectible_14: bool,

                    #[heading_level = 4]
                    ///Fazerblast
                    fazerblast_collectibles: Title,

                        ///Freddy Balloon
                        #[default = false]
                        CommonCollectible_15: bool,

                        ///Old Poster
                        #[default = false]
                        c_poster4: bool,

                        ///Space Chica Keychain
                        #[default = false]
                        c_key_spchica: bool,

                        ///Space Roxy Keychain
                        #[default = false]
                        c_key_sproxy: bool,

                    #[heading_level = 4]
                    ///Fazerblast Sublobby
                    fazer_lobby_collectibles: Title,

                        ///Freddy Pinata
                        #[default = false]
                        CommonCollectible_10: bool,

                        ///Space Freddy Keychain
                        #[default = false]
                        c_key_spfreddy: bool,

                    #[heading_level = 4]
                    ///Kid's Cove Sublobby
                    kids_cove_lobby_collectibles: Title,

                        ///Golden Sun
                        #[default = false]
                        c_golden_sun: bool,

                        ///Moon Plush
                        #[default = false]
                        UncommonCollectible_5: bool,

                    #[heading_level = 4]
                    ///Laundry
                    laundry_collectibles: Title,

                        ///Glam Roxy Plush
                        #[default = false]
                        UncommonCollectible_1: bool,

                        ///Old Poster
                        #[default = false]
                        c_poster2: bool,

                        ///Star Shirt
                        #[default = false]
                        c_shirt_star: bool,

                    #[heading_level = 4]
                    ///Lobby
                    lobby_collectibles: Title,

                        ///Chica Name Shirt
                        #[default = false]
                        CommonCollectible_8: bool,

                        ///Cupcake Pinata
                        #[default = false]
                        CommonCollectible_18: bool,

                        ///Freddy Name Shirt
                        #[default = false]
                        CommonCollectible_6: bool,

                        ///Glam Chica Plush
                        #[default = false]
                        UncommonCollectible_2: bool,

                        ///Sun Plush
                        #[default = false]
                        CommonCollectible_1: bool,

                    #[heading_level = 4]
                    ///Main Atrium
                    atrium_collectibles: Title,

                        ///Monty Pinata
                        #[default = false]
                        CommonCollectible_20: bool,

                        ///Roxy Name Shirt
                        #[default = false]
                        CommonCollectible_9: bool,

                    #[heading_level = 4]
                    ///Monty Golf
                    monty_golf_collectibles: Title,

                        ///Go Kart
                        #[default = false]
                        c_gokart: bool,

                        ///Tragedy Mask
                        #[default = false]
                        c_mask_tragedy: bool,

                    #[heading_level = 4]
                    ///Monty Golf Sublobby
                    monty_golf_lobby_collectibles: Title,

                        ///Frozen Monty Treat
                        #[default = false]
                        CommonCollectible_2: bool,

                        ///Glam Monty Figure
                        #[default = false]
                        UncommonCollectible_8: bool,

                    #[heading_level = 4]
                    ///Parts & Service
                    parts_n_service_collectibles: Title,

                        ///Glam Roxy Figure
                        #[default = false]
                        UncommonCollectible_9: bool,

                    #[heading_level = 4]
                    ///Prize Counter
                    prize_counter_collectibles: Title,

                        ///Frozen Freddy Treat 
                        #[default = false]
                        CommonCollectible_3: bool,

                        ///Monty Mask
                        #[default = false]
                        RareCollectible_4: bool,

                    #[heading_level = 4]
                    ///Rockstar Row
                    rockstar_row_collectibles: Title,

                        ///Chica Mask
                        #[default = false]
                        RareCollectible_2: bool,

                        ///Four Block Shirt
                        #[default = false]
                        c_shirt_four: bool,

                        ///Golden Freddy
                        #[default = false]
                        c_golden_freddy: bool,

                        ///Monty Name Shirt
                        #[default = false]
                        CommonCollectible_12: bool,

                        ///Pizzaplex Logo Shirt
                        #[default = false]
                        CommonCollectible_7: bool,

                    #[heading_level = 4]
                    ///Roxy Raceway
                    roxy_raceway_collectibles: Title,

                        ///Monty Balloon
                        #[default = false]
                        CommonCollectible_16: bool,

                        ///Space Monty Keychain
                        #[default = false]
                        c_key_spmonty: bool,

                    #[heading_level = 4]
                    ///Roxy Raceway Sublobby
                    roxy_raceway_lobby_collectibles: Title,

                        ///Frozen Roxy Treat
                        #[default = false]
                        CommonCollectible_5: bool,

                        ///Roxy Balloon
                        #[default = false]
                        CommonCollectible_17: bool,

                        ///Roxy Mask
                        #[default = false]
                        RareCollectible_3: bool,

                    #[heading_level = 4]
                    ///Roxy Salon
                    roxy_salon_collectibles: Title,

                        ///Roxy Magnet
                        #[default = false]
                        c_magnet_roxy: bool,

                        ///Roxy Pinata
                        #[default = false]
                        CommonCollectible_11: bool,

                    #[heading_level = 4]
                    ///Salads & Sides
                    salads_n_sides_collectibles: Title,

                        ///Freddy Face Shirt
                        #[default = false]
                        c_shirt_freddyface: bool,

                        ///Pinata
                        #[default = false]
                        c_pinata_general: bool,

                    #[heading_level = 4]
                    ///Sewers
                    sewers_collectibles: Title,

                        ///Old Poster
                        #[default = false]
                        c_poster3: bool,

                    #[heading_level = 4]
                    ///Utility Tunnels
                    utility_tunnels_collectibles: Title,

                        ///Chica Pinata
                        #[default = false]
                        CommonCollectible_19: bool,

                        ///Comedy Mask
                        #[default = false]
                        c_mask_comedy: bool,

                        ///Foxy Plush
                        #[default = false]
                        c_plush_foxy: bool,

                        ///Glam Freddy Plush
                        #[default = false]
                        UncommonCollectible_3: bool,

                        ///Golden Roxy
                        #[default = false]
                        c_golden_roxy: bool,

                        ///Moon Figure
                        #[default = false]
                        UncommonCollectible_11: bool,

                        ///Sun Figure
                        #[default = false]
                        UncommonCollectible_10: bool,

                    #[heading_level = 4]
                    ///Warehouse
                    warehouse_collectibles: Title,

                        ///Nightmare Plush
                        #[default = false]
                        c_plush_night: bool,

                    #[heading_level = 4]
                    ///West Arcade
                    west_arcade_collectibles: Title,

                        ///Glam Monty Plush
                        #[default = false]
                        UncommonCollectible_4: bool,

                #[heading_level = 3]
                ///Equipment
                equipment: Title,

                    #[heading_level = 4]
                    ///Backstage
                    backstage_equipment: Title,

                        ///Backstage Pass
                        #[default = false]
                        BackstagePass: bool,

                        ///Flashlight Upgrade
                        #[default = false]
                        FlashlightUpgrade_3: bool,

                    #[heading_level = 4]
                    ///Basement Kitchen
                    basement_kitchen_equipment: Title,

                        ///Freddy Fizzy Faz
                        #[default = false]
                        GregoryUpgrade_Stamina_3: bool,

                    #[heading_level = 4]
                    ///Bonnie Bowl
                    bonnie_bowl_equipment: Title,

                        ///Monty Mystery Mix
                        #[default = false]
                        MontyMysteryMix: bool,

                    #[heading_level = 4]
                    ///Chica's Bakery
                    chica_bakery_equipment: Title,

                        ///Hoodie
                        #[default = false]
                        GregoryUpgrade_Stealth: bool,

                    #[heading_level = 4]
                    ///Daycare
                    daycare_equipment: Title,

                        ///Flashlight
                        #[default = false]
                        Flashlight: bool,

                        ///Flashlight Upgrade
                        #[default = false]
                        FlashlightUpgrade_2: bool,

                        ///Mazercise Control Key
                        #[default = false]
                        MazerciseControlKey: bool,

                    #[heading_level = 4]
                    ///El Chips
                    el_chips_equipment: Title,

                        ///Monty Fizzy Faz
                        #[default = false]
                        GregoryUpgrade_Stamina_4: bool,

                    #[heading_level = 4]
                    ///Fazerblast
                    fazerblast_equipment: Title,

                        ///Bowling Pass
                        #[default = false]
                        BowlingTicket: bool,

                        ///Golden Fazerblaster
                        #[default = false]
                        gold_blaster: bool,

                        ///Grey Fazerblaster
                        #[default = false]
                        grey_blaster: bool,

                    #[heading_level = 4]
                    ///Lobby
                    lobby_equipment: Title,

                        ///Chica Fizzy Faz
                        #[default = false]
                        GregoryUpgrade_Stamina_2: bool,

                        ///Daycare Pass
                        #[default = false]
                        daycare_pass: bool,

                        ///Entrance Pass
                        #[default = false]
                        CompPass: bool,

                        ///Mr. Hippo Magnet
                        #[default = false]
                        MrHippoMagnet: bool,

                        ///Screwdriver
                        #[default = false]
                        ScrewDriver: bool,

                    #[heading_level = 4]
                    ///Main Atrium
                    atrium_equipment: Title,

                        ///Freddy Upgrade
                        #[default = false]
                        FreddyUpgrade_1: bool,

                        ///Mapbot's Map
                        #[default = false]
                        Map: bool,

                    #[heading_level = 4]
                    ///Monty Golf
                    monty_golf_equipment: Title,

                        ///Fazcam
                        #[default = false]
                        FlashBeacon: bool,

                        ///Flashlight Upgrade
                        #[default = false]
                        FlashlightUpgrade_1: bool,

                        ///Mazercise Pass
                        #[default = false]
                        MazerciseTicket: bool,

                        ///Monty's Claws
                        #[default = false]
                        MontyClaws_C: bool,

                    #[heading_level = 4]
                    ///Rockstar Row
                    rockstar_row_equipment: Title,

                        ///Fazwatch
                        #[default = false]
                        Fazwatch: bool,

                        ///Party Pass
                        #[default = false]
                        PartyPassChicaRoom: bool,

                        ///Photo Pass
                        #[default = false]
                        PhotoPass: bool,

                    #[heading_level = 4]
                    ///Roxy Raceway
                    roxy_raceway_equipment: Title,

                        ///Damaged Head
                        #[default = false]
                        DamagedHead: bool,

                        ///Dance Pass
                        #[default = false]
                        DancePass: bool,

                        ///Freddy Upgrade
                        #[default = false]
                        FreddyUpgrade_2: bool,

                        ///Roxy Fizzy Faz
                        #[default = false]
                        GregoryUpgrade_Stamina_1: bool,

                        ///Roxy's Eyes
                        #[default = false]
                        RoxyEyes_C: bool,

                    #[heading_level = 4]
                    ///Roxy Salon
                    roxy_salon_equipment: Title,

                        ///Shoes
                        #[default = false]
                        GregoryUpgrade_Shoes: bool,

                    #[heading_level = 4]
                    ///Sewers
                    sewers_equipment: Title,

                        ///Chica's Voicebox
                        #[default = false]
                        chica_voicebox: bool,

                    #[heading_level = 4]
                    ///Utility Tunnels
                    utility_tunnels_equipment: Title,

                        ///Mapbot's Map
                        #[default = false]
                        UtilityHallwayMap: bool,

                        ///Pizzaplex Cameras
                        #[default = false]
                        pizzaplex_cameras: bool,

                    #[heading_level = 4]
                    ///West Arcade
                    west_arcade_equipment: Title,

                        ///Repaired Head
                        #[default = false]
                        repaired_head: bool,


                #[heading_level = 3]
                ///Message Bags
                message_bags: Title,

                    ///AC Inspection
                    #[default = false]
                    LilMusicMan1: bool,

                    ///All Staff Meeting
                    #[default = false]
                    StaffParty2: bool,

                    ///AR-CADE MAINT LOG
                    #[default = false]
                    ArcadeGlitches3: bool,

                    ///ARCADE CONSPIRACY
                    #[default = false]
                    ArcadeGlitches8: bool,

                    ///BBW MAINT LOG
                    #[default = false]
                    ArcadeGlitches1: bool,

                    ///BEHIND THE MAZE
                    #[default = false]
                    MontyBoss2: bool,

                    ///BETTER EMPLOYEES
                    #[default = false]
                    RayMessage: bool,

                    ///CFF MAINT LOG
                    #[default = false]
                    ArcadeGlitches2: bool,

                    ///Chasing Cars
                    #[default = false]
                    RoxyBoss1: bool,

                    ///CHICA REPORT
                    #[default = false]
                    ChicaBoss4: bool,

                    ///CHICA UPGRADE
                    #[default = false]
                    ChicaVoiceBox_M: bool,

                    ///ROXY UPGRADE
                    #[default = false]
                    RoxyEyes_M: bool,

                    ///MONTY UPGRADE
                    #[default = false]
                    MontyClaws_M: bool,

                    ///COMPACTOR INSTRUCTIONS
                    #[default = false]
                    ChicaBoss3: bool,

                    ///Drink Fizzy Faz!!!
                    #[default = false]
                    StaminaBoosters: bool,

                    ///Easy Money
                    #[default = false]
                    UpgradeMachine1: bool,

                    ///False Alarm
                    #[default = false]
                    _911Call: bool,

                    ///Food Storage
                    #[default = false]
                    KitchenStaffNote: bool,

                    ///HI DAVE
                    #[default = false]
                    Complaint2: bool,

                    ///Hide the Mix
                    #[default = false]
                    ChicaBoss1: bool,

                    ///IT IS HAPPENING AGAIN
                    #[default = false]
                    Complaint3: bool,

                    ///Job Security
                    #[default = false]
                    MontyBoss3: bool,

                    ///MARKED FOR DELETION
                    #[default = false]
                    Vanessa2: bool,

                    ///MISSING
                    #[default = false]
                    BonnieMissing3: bool,

                    ///Monty Mischief
                    #[default = false]
                    MontyBoss4: bool,

                    ///NIGHT SHIFT
                    #[default = false]
                    NightShift: bool,

                    ///Night Terrors
                    #[default = false]
                    NightTerrors: bool,

                    ///No Flash Photography
                    #[default = false]
                    FlashCam: bool,

                    ///NO QUESTIONS ASKED
                    #[default = false]
                    Vanessa1: bool,

                    ///OLD ELEVATOR
                    #[default = false]
                    Sinkhole3: bool,

                    ///OUT OF ORDER
                    #[default = false]
                    ArcadeGlitches6: bool,

                    ///Party Foul
                    #[default = false]
                    Freddy1: bool,

                    ///PINK SLIP
                    #[default = false]
                    Staff1: bool,

                    ///POWER DRAIN
                    #[default = false]
                    Sinkhole2: bool,

                    ///PQ1 MAINT LOG
                    #[default = false]
                    ArcadeGlitches4: bool,

                    ///PQ2 MAINT LOG
                    #[default = false]
                    ArcadeGlitches5: bool,

                    ///QUESTION
                    #[default = false]
                    StaffParty1: bool,

                    ///Re-Theme
                    #[default = false]
                    BonnieMissing2: bool,

                    ///RECYCLED PIZZA?
                    #[default = false]
                    Complaint1: bool,

                    ///RED FLAG
                    #[default = false]
                    ArcadeGlitches7: bool,

                    ///SAFETY CHECK
                    #[default = false]
                    ChicaBoss2: bool,

                    ///SINKHOLE
                    #[default = false]
                    Sinkhole1: bool,

                    ///SORE WINNER
                    #[default = false]
                    RoxyBoss2: bool,

                    ///Stolen Property
                    #[default = false]
                    LilMusicMan2: bool,

                    ///TEST DRIVERS WANTED
                    #[default = false]
                    RoxyBoss3: bool,

                    ///THE ANSWER
                    #[default = false]
                    MontyBoss1: bool,

                    ///TORN PAPER
                    #[default = false]
                    TornPaper: bool,

                    ///ULTIMATE PARTY HOST
                    #[default = false]
                    DJMM: bool,

                    ///Understudy
                    #[default = false]
                    BonnieMissing1: bool,

                #[heading_level = 3]
                ///Special Collectibles
                special_collectibles: Title,

                    ///Chica Magnet (West Arcade / Chica's Cakes)
                    #[default = false]
                    c_magnet_chica: bool,

                #[heading_level = 3]
                ///Retro CDs
                retro_cds: Title,

                    ///Backstage Podium
                    #[default = false]
                    RetroCDCollectible_9: bool,

                    ///Bonnie Bowl
                    #[default = false]
                    RetroCDCollectible_8: bool,

                    ///Chica's Bakery
                    #[default = false]
                    RetroCDCollectible_3: bool,

                    ///West Atrium Stage
                    #[default = false]
                    RetroCDCollectible_15: bool,

                    ///Fazerblast
                    #[default = false]
                    RetroCDCollectible_7: bool,

                    ///Kid's Cove
                    #[default = false]
                    RetroCDCollectible_4: bool,

                    ///Main Atrium
                    #[default = false]
                    RetroCDCollectible_10: bool,

                    ///Mazercise
                    #[default = false]
                    RetroCDCollectible_14: bool,

                    ///Monty Golf
                    #[default = false]
                    RetroCDCollectible_13: bool,

                    ///Rockstar Row Foxy
                    #[default = false]
                    RetroCDCollectible_1: bool,

                    ///Rockstar Row Helpy
                    #[default = false]
                    RetroCDCollectible_11: bool,

                    ///Roxy Raceway
                    #[default = false]
                    RetroCDCollectible_12: bool,

                    ///Roxy Salon
                    #[default = false]
                    RetroCDCollectible_5: bool,

                    ///Utility Tunnels Couch
                    #[default = false]
                    RetroCDCollectible_2: bool,

                    ///Utility Tunnels Foxy Plush
                    #[default = false]
                    RetroCDCollectible_6: bool,

                    ///West Arcade
                    #[default = false]
                    RetroCDCollectible_16: bool,

            #[heading_level = 2]
            ///Security Badges
            security_badges: Title,

                ///Security Badge 1
                #[default = false]
                security_badge_1: bool,

                ///Security Badge 2
                #[default = false]
                security_badge_2: bool,

                ///Security Badge 3
                #[default = false]
                security_badge_3: bool,

                ///Security Badge 4
                #[default = false]
                security_badge_4: bool,

                ///Security Badge 5
                #[default = false]
                security_badge_5: bool,

                ///Security Badge 6
                #[default = false]
                security_badge_6: bool,

                ///Security Badge 7
                #[default = false]
                security_badge_7: bool,

                ///Security Badge 8
                #[default = false]
                security_badge_8: bool,

        #[heading_level = 1]
        ///Positional Splits
        positional_splits: Title,

            #[heading_level = 2]
            ///Bonnie Bowl
            bonnie_bowl_position: Title,

                ///Enter Bonnie Bowl
                #[default = false]
                enter_bonnie_bowl: bool,

            #[heading_level = 2]
            ///Daycare
            daycare_position: Title,

                ///Enter Daycare
                #[default = false]
                enter_daycare: bool,

            #[heading_level = 2]
            ///El Chips
            el_chips_position: Title,

                ///Enter El Chips
                #[default = false]
                enter_el_chips: bool,

            #[heading_level = 2]
            ///Fazerblast
            fazerblast_position: Title,

                ///Fazerblast Spiral Stairs
                #[default = false]
                fazerblast_stairs: bool,

            #[heading_level = 2]
            ///Fazerblast Lobby
            fazerblast_lobby_position: Title,

                ///Rail Outside Fazerblast
                #[default = false]
                fazerblast_lobby_rail: bool,

            #[heading_level = 2]
            ///Underground / Afton Cave
            afton_position: Title,

                ///Exit Afton Elevator
                #[default = false]
                afton_elevator_position: bool,

            #[heading_level = 2]
            ///Utility Tunnels
            utility_tunnels_position: Title,

                ///First Aid Vanessa Cutscene
                #[default = false]
                first_aid_vanessa_scene: bool,

                ///Freddy Rail Jump
                #[default = false]
                freddy_rail_jump: bool,

                ///Monty Chase Start
                #[default = false]
                monty_chase_start: bool,

                ///STR-ATR-W Stairs
                #[default = false]
                str_atr_w_stairs: bool, 

                ///STR-LB Stairs
                #[default = false]
                str_lb_stairs: bool,

            #[heading_level = 2]
            ///West Arcade
            west_arcade_position: Title,

                ///Enter West Arcade
                #[default = false]
                enter_west_arcade: bool,

                ///Exit West Arcade
                #[default = false]
                exit_west_arcade: bool,

        #[heading_level = 1]
        ///Time Splits
        time_splits: Title,

            ///Exit Vents (11:30PM)
            #[default = false]
            time_11_30: bool,

            ///Freddy Recharge (11:45PM)
            #[default = false]
            time_11_45: bool,

            ///Front Entrance Closure(12:00AM)
            #[default = false]
            time_12_00: bool,

            ///Enter Daycare (12:30AM)
            #[default = false]
            time_12_30: bool,

            ///Daycare Nighttime (12:55AM)
            #[default = false]
            time_12_55: bool,

            ///Daycare Vanny Cutscene (1:00AM)
            #[default = false]
            time_1_00: bool,

            ///Mini Music Man Chase (1:15AM)
            #[default = false]
            time_1_15: bool,

            ///Pizzabot (1:30AM)
            #[default = false]
            time_1_30: bool,

            ///White Woman Jumpscare (2:00AM)
            #[default = false]
            time_2_00: bool,

            ///Dead Fred (2:15AM)
            #[default = false]
            time_2_15: bool,

            ///Backstage Passs (2:30AM)
            #[default = false]
            time_2_30: bool,

            ///Use Showtime Disk (2:45AM)
            #[default = false]
            time_2_45: bool,

            ///Freddy Abduction Recharge (3:00AM)
            #[default = false]
            time_3_00: bool,

            ///Vanessa Repair Cutscene (3:15AM)
            #[default = false]
            time_3_15: bool,

            ///Freddy Power Upgrade (3:30AM)
            #[default = false]
            time_3_30: bool,

            ///Party Pass Recharge (4:00AM)
            #[default = false]
            time_4_00: bool,

            ///Golden Fazerblaster (4:15AM)
            #[default = false]
            time_4_15: bool,

            ///Monty Mix / Mazercise Key (4:30AM)
            #[default = false]
            time_4_30: bool,

            ///Leave Sewers (4:40AM)
            #[default = false]
            time_4_40: bool,

            ///Freddy Upgrade Recharge (5:00AM)
            #[default = false]
            time_5_00: bool,

            ///Damaged Head (5:15AM)
            #[default = false]
            time_5_15: bool,

            ///Repaired Head (5:30AM)
            #[default = false]
            time_5_30: bool,

            ///Finish Roxy Sequence (5:40AM)
            #[default = false]
            time_5_40: bool,

            ///Freddy Eye Upgrade Nighttime (5:50AM)
            #[default = false]
            time_5_50: bool,

            ///Reach Exit Door (6:00AM)
            #[default = false]
            time_6_00: bool,

    #[heading_level = 1]
    ///In-Game Time Settings
    igt_settings: Title,

        ///Elevator Pauses
        #[default = false]
        elevator_pause: bool,

        ///Stop Timer On Menu
        #[default = false]
        menu_pause: bool,

        ///Stop Timer When Loading
        #[default = false]
        load_pause: bool,

        ///Stop Timer When Paused
        #[default = false]
        pause_pause: bool,

    #[heading_level = 1]
    ///Reset Settings
    reset_settings: Title,

        ///
        #[default = false]
        

    ///Unsupported version warning
    #[default = false]
    unknown_version_warn: bool,
}
