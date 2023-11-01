use asr::{
    settings::Gui,
    settings::gui::Title,
};

#[derive(Gui)]
pub struct Settings {
    #[heading_level = 0]
    ///Split Settings
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

            #[heading_level = 2]
            ///Counting Splits
            _counter_splits: Title,

                    #[heading_level = 3]
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

                    #[heading_level = 3]
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

                    #[heading_level = 3]
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

                    #[heading_level = 3]
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

                    #[heading_level = 3]
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

                    #[heading_level = 3]
                    ///Deload Splits
                    _deload_splits: Title,


}
