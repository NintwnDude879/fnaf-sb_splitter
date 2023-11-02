use asr::{
    settings::Gui,
    settings::gui::Title,
};

#[derive(Gui)]
pub struct Settings {
    ///Split Settings
    #[heading_level = 1]
    _split_settings: Title,

        #[heading_level = 1]
        ///Arcade Splits
        _arcade_splits: Title,

            #[heading_level = 2]
            ///BB Arcade
            _bb_arcade: Title,

                ///Start Arcade
                #[default = false]
                _bb_start: bool,

                ///Finish Arcade
                #[default = false]
                _bb_end: bool,

            #[heading_level = 2]
            ///Monty Golf
            _mg_arcade: Title,

                ///Start Arcade
                #[default = false]
                _mg_start: bool,

                ///Finish Hole 1
                #[default = false]
                _hole_1: bool,

                ///Finish Hole 2
                #[default = false]
                _hole_2: bool,

                ///Finish Hole 3
                #[default = false]
                _hole_3: bool,

                ///Finish Hole 4
                #[default = false]
                _hole_4: bool,

                ///Finish Hole 5
                #[default = false]
                _hole_5: bool,

                ///Finish Hole 6
                #[default = false]
                _hole_6: bool,

                ///Finish Hole 7
                #[default = false]
                _hole_7: bool,

                ///Finish Hole 8
                #[default = false]
                _hole_8: bool,

                ///Finish Hole 9
                #[default = false]
                _hole_9: bool,

                ///Finish Arcade
                #[default = false]
                _mg_end: bool,

            #[heading_level = 2]
            ///Princess Quest
            _pq_arcades: Title,

                #[heading_level = 3]
                ///Princess Quest 1
                _pq_1: Title,

                    ///Start Arcade
                    #[default = false]
                    _pq_1_start: bool,

                    ///2nd Room
                    #[default = false]
                    _pq_1_1: bool,

                    ///Exit Starting Room
                    #[default = false]
                    _pq_1_2: bool,

                    ///3rd Room
                    #[default = false]
                    _pq_1_3: bool,

                    ///Key Door
                    #[default = false]
                    _pq_1_4: bool,

                    ///Crossroads
                    #[default = false]
                    _pq_1_5: bool,

                    ///Right Door
                    #[default = false]
                    _pq_1_6: bool,

                    ///Enter Graveyard
                    #[default = false]
                    _pq_1_7: bool,

                    ///Staircase
                    #[default = false]
                    _pq_1_8: bool,

                    ///Final Room
                    #[default = false]
                    _pq_1_9: bool,

                    ///Finish Arcade
                    #[default = false]
                    _pq_1_end: bool,

                #[heading_level = 3]
                ///Princess Quest 2
                _pq_2: Title,

                    ///Start Arcade
                    #[default = false]
                    _pq_2_start: bool,

                    ///Exit Starting Room
                    #[default = false]
                    _pq_2_1: bool,

                    ///2nd Room
                    #[default = false]
                    _pq_2_2: bool,

                    ///Balls Room
                    #[default = false]
                    _pq_2_3: bool,

                    ///4th Room
                    #[default = false]
                    _pq_2_4: bool,

                    ///Split Puzzle Room
                    #[default = false]
                    _pq_2_5: bool,

                    ///Big Torch Room
                    #[default = false]
                    _pq_2_6: bool,

                    ///Hallway
                    #[default = false]
                    _pq_2_7: bool,

                    ///Big Split Puzzle Room
                    #[default = false]
                    _pq_2_8: bool,

                    ///Bedroom
                    #[default = false]
                    _pq_2_9: bool,

                    ///Enter Final Room
                    #[default = false]
                    _pq_2_10: bool,

                    ///Finish Arcade
                    #[default = false]
                    _pq_2_end: bool,


                #[heading_level = 3]
                ///Princess Quest 3
                _pq_3: Title,

                    ///Start Arcade
                    #[default = false]
                    _pq3_start: bool,

                    ///Hallway
                    #[default = false]
                    _pq3_1: bool,

                    ///Hub Room
                    #[default = false]
                    _pq3_2: bool,

                    ///Conveyor Room
                    #[default = false]
                    _pq3_3: bool,

                    ///Split Puzzle Room (Glitchtrap Plush)
                    #[default = false]
                    _pq3_4: bool,

                    ///Flamin' Hot Foxy
                    #[default = false]
                    _pq3_5: bool,

                    ///Prize Counter
                    #[default = false]
                    _pq3_6: bool,

                    ///Enter Final Area
                    #[default = false]
                    _pq3_7: bool,

                    ///Use Key
                    #[default = false]
                    _pq3_end: bool,

        #[heading_level = 1]
        ///Counting Splits
        _counter_splits: Title,

            #[heading_level = 2]
            ///Daycare Generators
            _daycare_gens: Title,

                ///Generator 1
                #[default = false]
                _daycare_gen_1: bool,

                ///Generator 2
                #[default = false]
                _daycare_gen_2: bool,

                ///Generator 3
                #[default = false]
                _daycare_gen_3: bool,

                ///Generator 4
                #[default = false]
                _daycare_gen_4: bool,

                ///Generator 5
                #[default = false]
                _daycare_gen_5: bool,

            #[heading_level = 2]
            ///Fazerblast Flags
            _fazer_flags: Title,

                ///Flag 1
                #[default = false]
                _fazer_flag_1: bool,

                ///Flag 2
                #[default = false]
                _fazer_flag_2: bool,

                ///Flag 3
                #[default = false]
                _fazer_flag_3: bool,

            #[heading_level = 2]
            ///Monty Bucket Balls
            _monty_bucket: Title,

                ///10 Balls
                #[default = false]
                _monty_balls_10: bool,

                ///20 Balls
                #[default = false]
                _monty_balls_20: bool,

                ///25 Balls (bucket full)
                #[default = false]
                _monty_balls_25: bool,

            #[heading_level = 2]
            ///Sewer Generators
            _sewer_gens: Title,

                ///Generator 1
                #[default = false]
                _sewer_gen_1: bool,

                ///Generator 2
                #[default = false]
                _sewer_gen_2: bool,

                ///Generator 3
                #[default = false]
                _sewer_gen_3: bool,

            #[heading_level = 2]
            ///West Arcade Generators
            _west_arcade_gens: Title,

                ///Generator 1
                #[default = false]
                _west_arcade_gen_1: bool,

                ///Generator 2
                #[default = false]
                _west_arcade_gen_2: bool,

                ///Generator 3
                #[default = false]
                _west_arcade_gen_3: bool,

                ///Generator 4
                #[default = false]
                _west_arcade_gen_4: bool,

                ///Generator 5
                #[default = false]
                _west_arcade_gen_5: bool,

        #[heading_level = 1]
        ///Deload Splits
        _deload_splits: Title,

            ///Backstage
            #[heading_level = 2]
            _backstage_deloads: Title,

                ///Foxy Cutout Deload
                #[default = false]
                _foxy_cutout_deload: bool,

            ///Daycare
            #[heading_level = 2]
            _daycare_deloads: Title,

                ///Arcade Deload
                #[default = false]
                _arcade_deload: bool,

                ///Theatre Deload
                #[default = false]
                _theatre_deload: bool,

            ///Kid's Cove Sublobby
            #[heading_level = 2]
            _kids_cove_lobby_deloads: Title,

                ///Fence Deload
                #[default = false]
                _kids_cove_fence_deload: bool,


            ///Monty Golf Sublobby
            #[heading_level = 2]
            _monty_golf_lobby_deloads: Title,

                ///Fence Deload
                #[default = false]
                _monty_golf_fence_deload: bool,

            ///Prize Counter
            #[heading_level = 2]
            _prize_counter_deloads: Title,

                ///Counter Deload
                #[default = false]
                _counter_deload: bool,

            ///Rockstar Row
            #[heading_level = 2]
            _rockstar_row_deloads: Title,

                ///Chica Greenroom Deload
                #[default = false]
                _chica_greenroom_deload: bool,

                ///Curtain Deload
                #[default = false]
                _curtain_deload: bool,

                ///Roxy Cutout Deload
                #[default = false]
                _roxy_cutout_deload: bool,

                ///Tunnel Door Deload
                #[default = false]
                _tunnel_door_deload: bool,

            ///Roxy Raceway
            #[heading_level = 2]
            _roxy_raceway_deloads: Title,

                ///Afton Rock Column Deload
                #[default = false]
                _afton_rocks_deload: bool,

                ///Garage Fencejump Deload
                #[default = false]
                _garage_fence_deload: bool,

                ///Roxy's Eyes Deload
                #[default = false]
                _roxy_eyes_deload: bool,

            ///Roxy Raceway Sublobby
            #[heading_level = 2]
            _roxy_raceway_lobby_deloads: Title,

                ///Balloon Deload
                #[default = false]
                _balloon_deload: bool,

            ///Roxy Salon
            #[heading_level = 2]
            _roxy_salon_deloads: Title,

                ///Plant Deload
                #[default = false]
                _plant_deload: bool,

        #[heading_level = 1]
        ///Ending Splits
        _ending_splits: Title,

            ///Vanny Ending
            #[default = false]
            _vanny_ending: bool,

            ///Car Battery Ending
            #[default = false]
            _car_battery_ending: bool,

            ///Escape Ending
            #[default = false]
            _escape_ending: bool,

            ///Fire Escape Ending
            #[default = false]
            _fire_escape_ending: bool,

            ///Princess Quest Ending
            #[default = false]
            _pq_ending: bool,

            #[heading_level = 2]
            ///Afton Ending
            _afton_ending: Title,

                ///Button 1
                #[default = false]
                _afton_button_1: bool,

                ///Button 2
                #[default = false]
                _afton_button_2: bool,

                ///Button 3
                #[default = false]
                _afton_button_3: bool,

                ///Button 4
                #[default = false]
                _afton_button_4: bool,

                ///Button 5
                #[default = false]
                _afton_button_5: bool,

                ///Button 6
                #[default = false]
                _afton_button_6: bool,

                ///Button 7
                #[default = false]
                _afton_button_7: bool,

                ///Button 8 / Ending
                #[default = false]
                _afton_button_8: bool,

        #[heading_level = 1]
        ///Item Splits
        _item_splits: Title,

            #[heading_level = 2]
            ///General Items
            _general_items: Title,

                #[heading_level = 3]
                ///Collectibles
                _collectibles: Title,

                    #[heading_level = 4]
                    ///Backstage
                    _backstage_collectibles: Title,

                    #[heading_level = 4]
                    ///Basement Kitchen
                    _basement_kitchen_collectibles: Title,

                    #[heading_level = 4]
                    ///Bonnie Bowl
                    _bonnie_bowl_collectibles: Title,

                    #[heading_level = 4]
                    ///Chica's Bakery
                    _chica_bakery_collectibles: Title,

                    #[heading_level = 4]
                    ///Daycare
                    _daycare_collectibles: Title,

                    #[heading_level = 4]
                    ///El Chips
                    _el_chips_collectibles: Title,

                    #[heading_level = 4]
                    ///Fazerblast
                    _fazerblast_collectibles: Title,

                    #[heading_level = 4]
                    ///Fazerblast Sublobby
                    _fazer_lobby_collectibles: Title,

                    #[heading_level = 4]
                    ///Kid's Cove Sublobby
                    _kids_cove_lobby_collectibles: Title,

                    #[heading_level = 4]
                    ///Laundry
                    _laundry_collectibles: Title,

                    #[heading_level = 4]
                    ///Lobby
                    _lobby_collectibles: Title,

                    #[heading_level = 4]
                    ///Main Atrium
                    _atrium_collectibles: Title,

                    #[heading_level = 4]
                    ///Monty Golf
                    _monty_golf_collectibles: Title,

                    #[heading_level = 4]
                    ///Monty Golf Sublobby
                    _monty_golf_lobby_collectibles: Title,

                    #[heading_level = 4]
                    ///Parts & Service
                    _parts_n_service_collectibles: Title,

                    #[heading_level = 4]
                    ///Prize Counter
                    _prize_counter_collectibles: Title,

                    #[heading_level = 4]
                    ///Rockstar Row
                    _rockstar_row_collectibles: Title,

                    #[heading_level = 4]
                    ///Roxy Raceway
                    _roxy_raceway_collectibles: Title,

                    #[heading_level = 4]
                    ///Roxy Raceway Sublobby
                    _roxy_raceway_lobby_collectibles: Title,

                    #[heading_level = 4]
                    ///Roxy Salon
                    _roxy_salon_collectibles: Title,

                    #[heading_level = 4]
                    ///Salads & Sides
                    _salads_n_sides_collectibles: Title,

                    #[heading_level = 4]
                    ///Sewers
                    _sewers_collectibles: Title,

                    #[heading_level = 4]
                    ///Utility Tunnels
                    _utility_tunnels_collectibles: Title,

                    #[heading_level = 4]
                    ///Warehouse
                    _warehouse_collectibles: Title,

                    #[heading_level = 4]
                    ///West Arcade
                    _west_arcade_collectibles: Title,

                #[heading_level = 3]
                ///Equipment
                _equipment: Title,

                    #[heading_level = 4]
                    ///Backstage
                    _backstage_equipment: Title,

                    #[heading_level = 4]
                    ///Basement Kitchen
                    _basement_kitchen_equipment: Title,

                    #[heading_level = 4]
                    ///Bonnie Bowl
                    _bonnie_bowl_equipment: Title,

                    #[heading_level = 4]
                    ///Chica's Bakery
                    _chica_bakery_equipment: Title,

                    #[heading_level = 4]
                    ///Daycare
                    _daycare_equipment: Title,

                    #[heading_level = 4]
                    ///El Chips
                    _el_chips_equipment: Title,

                    #[heading_level = 4]
                    ///Fazerblast
                    _fazerblast_equipment: Title,

                    #[heading_level = 4]
                    ///Fazerblast Sublobby
                    _fazer_lobby_equipment: Title,

                    #[heading_level = 4]
                    ///Kid's Cove Sublobby
                    _kids_cove_lobby_equipment: Title,

                    #[heading_level = 4]
                    ///Laundry
                    _laundry_equipment: Title,

                    #[heading_level = 4]
                    ///Lobby
                    _lobby_equipment: Title,

                    #[heading_level = 4]
                    ///Main Atrium
                    _atrium_equipment: Title,

                    #[heading_level = 4]
                    ///Monty Golf
                    _monty_golf_equipment: Title,

                    #[heading_level = 4]
                    ///Monty Golf Sublobby
                    _monty_golf_lobby_equipment: Title,

                    #[heading_level = 4]
                    ///Parts & Service
                    _parts_n_service_equipment: Title,

                    #[heading_level = 4]
                    ///Prize Counter
                    _prize_counter_equipment: Title,

                    #[heading_level = 4]
                    ///Rockstar Row
                    _rockstar_row_equipment: Title,

                    #[heading_level = 4]
                    ///Roxy Raceway
                    _roxy_raceway_equipment: Title,

                    #[heading_level = 4]
                    ///Roxy Raceway Sublobby
                    _roxy_raceway_lobby_equipment: Title,

                    #[heading_level = 4]
                    ///Roxy Salon
                    _roxy_salon_equipment: Title,

                    #[heading_level = 4]
                    ///Salads & Sides
                    _salads_n_sides_equipment: Title,

                    #[heading_level = 4]
                    ///Sewers
                    _sewers_equipment: Title,

                    #[heading_level = 4]
                    ///Utility Tunnels
                    _utility_tunnels_equipment: Title,

                    #[heading_level = 4]
                    ///Warehouse
                    _warehouse_equipment: Title,

                    #[heading_level = 4]
                    ///West Arcade
                    _west_arcade_equipment: Title,

                #[heading_level = 3]
                ///Message Bags
                _message_bags: Title,

                #[heading_level = 3]
                ///Special Collectibles
                _special_collectibles: Title,

                #[heading_level = 3]
                ///Retro CDs
                _retro_cds: Title,

            #[heading_level = 2]
            ///Security Badges
            _security_badges: Title,

        #[heading_level = 1]
        ///Positional Splits
        _positional_splits: Title,

            #[heading_level = 2]
            ///Bonnie Bowl
            _bonnie_bowl_position: Title,

            #[heading_level = 2]
            ///Daycare
            _daycare_position: Title,

            #[heading_level = 2]
            ///El Chips
            _el_chips_position: Title,

            #[heading_level = 2]
            ///Fazerblast
            _fazerblast_position: Title,

            #[heading_level = 2]
            ///Fazerblast Lobby
            _fazerblast_lobby_position: Title,

            #[heading_level = 2]
            ///Underground / Afton Cave
            _afton_position: Title,

            #[heading_level = 2]
            ///Utility Tunnels
            _utility_tunnels_position: Title,

            #[heading_level = 2]
            ///West Arcade
            _west_arcade_position: Title,

        #[heading_level = 1]
        ///Time Splits
        _time_splits: Title,

    #[heading_level = 1]
    ///In-Game Time Settings
    _igt_settings: Title,

    #[heading_level = 1]
    ///Reset Settings
    _reset_settings: Title,

    ///Unsupported version warning
    #[default = false]
    _unknown_version_warn: bool,
}
