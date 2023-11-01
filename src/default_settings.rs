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

                #[heading_level = 3]
                ///Princess Quest 3
                _pq_3: Title,

    #[heading_level = 0]
    ///In-Game Time Settings
    _igt_settings: Title,
}
