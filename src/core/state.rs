use bevy::math::IVec2;

use crate::core::{
    command::{CleaningCommand, CommandSet, MovementCommand},
    dir::Dir,
    map::Map,
    rule::Rule,
    sensor::Sensor,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motion {
    Moved { from: IVec2, to: IVec2 },
    BumpedWall,
    Rotated { from: Dir, to: Dir },
    Exited,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TickResult {
    pub motion: Motion,
    pub collected_trash: bool,
}

#[derive(Debug)]
pub struct State {
    ticks: usize,

    vac_pos: IVec2,
    vac_dir: Dir,

    collected_trash_this_tick: bool,
    hit_wall_last_tick: bool,
    turned_last_tick: bool,

    cleaning: bool,
    trash: Vec<IVec2>,

    is_finished: bool,
}

impl State {
    pub fn new(vac_pos: impl Into<IVec2>, vac_dir: Dir) -> Self {
        Self {
            ticks: 0,

            vac_pos: vac_pos.into(),
            vac_dir,

            collected_trash_this_tick: false,
            hit_wall_last_tick: false,
            turned_last_tick: false,

            cleaning: false,
            trash: Vec::new(),

            is_finished: false,
        }
    }

    pub fn tick(&mut self, map: &Map, rules: &[Rule]) -> TickResult {
        // reset the trash flag manually
        self.collected_trash_this_tick = false;

        // environment check
        let exit = self.evaluate_environment(map);
        if exit {
            // make sure all of the trash has been collected
            if map.trash().iter().all(|pt| self.trash.contains(pt)) {
                self.is_finished = true;
                return TickResult {
                    motion: Motion::Exited,
                    collected_trash: self.collected_trash_this_tick,
                };
            }
        }

        // sensor eval
        let sensors = self.evaluate_sensors(map);

        // reset flags
        let turned_last_tick = self.turned_last_tick;
        self.reset_flags();

        // command computation based on rules
        let mut commands = Rule::compute_commands(rules, &sensors);

        // if we turned last tick, clear the movement command so that we definitely move forward
        if turned_last_tick {
            commands.clear_movement();
        }

        // advance tick counter
        self.ticks += 1;

        let motion = self.apply_commands(commands, map);
        TickResult {
            motion,
            collected_trash: self.collected_trash_this_tick,
        }
    }

    fn reset_flags(&mut self) {
        self.hit_wall_last_tick = false;
        self.turned_last_tick = false;
    }

    fn apply_commands(&mut self, commands: CommandSet, map: &Map) -> Motion {
        match commands.cleaning() {
            Some(CleaningCommand::StartCleaning) => self.cleaning = true,
            Some(CleaningCommand::StopCleaning) => self.cleaning = false,
            None => {}
        }

        // evaluate movement commands and return a motion
        match commands.movement() {
            // NOTE: move forward if no movement was specified
            None => {
                let orig_pos = self.vac_pos;
                // check for a wall collision
                let dest = orig_pos + self.vac_dir.to_ivec();

                if map.has_space(dest) {
                    self.vac_pos = dest;
                    Motion::Moved {
                        from: orig_pos,
                        to: self.vac_pos,
                    }
                } else {
                    self.hit_wall_last_tick = true;
                    Motion::BumpedWall
                }
            }
            Some(MovementCommand::TurnRight) => {
                let orig_dir = self.vac_dir;
                self.vac_dir = orig_dir.rotate_cw();
                self.turned_last_tick = true;
                Motion::Rotated {
                    from: orig_dir,
                    to: self.vac_dir,
                }
            }
            Some(MovementCommand::TurnLeft) => {
                let orig_dir = self.vac_dir;
                self.vac_dir = orig_dir.rotate_ccw();
                self.turned_last_tick = true;
                Motion::Rotated {
                    from: orig_dir,
                    to: self.vac_dir,
                }
            }
        }
    }

    fn evaluate_sensors(&self, map: &Map) -> Vec<Sensor> {
        let mut sensors = Vec::new();

        let left = self.vac_pos + self.vac_dir.rotate_ccw().to_ivec();
        if map.has_space(left) {
            sensors.push(Sensor::SpaceLeft);
        }

        let right = self.vac_pos + self.vac_dir.rotate_cw().to_ivec();
        if map.has_space(right) {
            sensors.push(Sensor::SpaceRight);
        }

        if self.hit_wall_last_tick {
            sensors.push(Sensor::HitWall);
        }

        if self.ticks == 0 {
            sensors.push(Sensor::Start);
        }

        sensors
    }

    pub fn vac_pos(&self) -> IVec2 {
        self.vac_pos
    }

    pub fn vac_dir(&self) -> Dir {
        self.vac_dir
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    /// returns true if we're on an exit tile
    fn evaluate_environment(&mut self, map: &Map) -> bool {
        // check if we can collect a trash here:
        // - there must be a trash on the map at our current position
        // - the vac must be in cleaning mode
        // - FIXME: only collect trash once!
        if map.trash().contains(&self.vac_pos) && self.cleaning {
            self.collected_trash_this_tick = true;
            self.trash.push(self.vac_pos);
        }

        self.vac_pos == map.exit()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::command::CleaningCommand;

    use super::*;

    #[test]
    fn test_evaluate_sensors() {
        let map = Map::parse(Map::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        // given that setup, there should be space on the left but not on the right
        let sensors = state.evaluate_sensors(&map);
        assert!(sensors.contains(&Sensor::SpaceLeft));
        assert!(!sensors.contains(&Sensor::SpaceRight));
        assert!(!sensors.contains(&Sensor::HitWall));

        // that was the first tick so we should have also gotten the start sensor
        assert!(sensors.contains(&Sensor::Start));

        state.hit_wall_last_tick = true;
        state.ticks += 1;
        let sensors = state.evaluate_sensors(&map);
        assert!(sensors.contains(&Sensor::HitWall));
        assert!(!sensors.contains(&Sensor::Start));
    }

    #[test]
    fn test_tick() {
        let map = Map::parse(Map::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        let rules = [
            Rule::new(Sensor::SpaceRight, MovementCommand::TurnRight),
            Rule::new(Sensor::SpaceLeft, MovementCommand::TurnLeft),
        ];

        // there's space on the left but not the right,
        // so we should turn left
        let result = state.tick(&map, &rules);
        assert_eq!(state.vac_dir, Dir::North);
        assert_eq!(
            result.motion,
            Motion::Rotated {
                from: Dir::East,
                to: Dir::North
            }
        );
    }

    #[test]
    fn test_hit_wall() {
        let map = Map::parse("S#").unwrap();
        let mut state = State::new((0, 0), Dir::East);

        // rule that reacts to a wall hit
        let rules = [Rule::new(Sensor::HitWall, MovementCommand::TurnRight)];

        // should hit the wall and then turn
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::BumpedWall));

        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Rotated { .. }));
    }

    #[test]
    fn test_no_consecutive_turns() {
        let map = Map::parse(Map::ROOM_4X4).unwrap();
        let mut state = State::new((1, 1), Dir::East);

        // rule that always tries to turn
        let rules = [Rule::new(Sensor::SpaceLeft, MovementCommand::TurnRight)];

        // first tick should turn
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Rotated { .. }));

        // second tick should move forward (restriction enforced)
        let result = state.tick(&map, &rules);
        assert!(!matches!(result.motion, Motion::Rotated { .. }));

        // third tick can turn again
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Rotated { .. }));
    }

    #[test]
    fn test_exit_tile() {
        // start with exit right in front of us
        let map = Map::parse("SE").unwrap();

        // start to the left of the exit, facing right
        let mut state = State::new((0, 0), Dir::East);

        // no rules, should move forward onto the exit
        let rules = [];

        // first tick should move forward
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Moved { .. }));

        // second tick should exit
        let result = state.tick(&map, &rules);
        assert!(state.is_finished);
        assert!(matches!(result.motion, Motion::Exited));
    }

    #[test]
    fn test_trash_collection() {
        // start with trash right in front of us
        let map = Map::parse("ST.").unwrap();
        let mut state = State::new((0, 0), Dir::East);

        // start cleaning at at start
        let rules = [Rule::new(Sensor::Start, CleaningCommand::StartCleaning)];

        // move forward
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Moved { .. }));

        // move again -- should collect trash because cleaning is ON
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Moved { .. }));
        assert!(result.collected_trash);
        assert_eq!(state.trash, vec![(1, 0).into()]);
    }

    #[test]
    fn test_trash_collection_requires_cleaning() {
        // start with trash right in front of us
        let map = Map::parse("ST.").unwrap();
        let mut state = State::new((0, 0), Dir::East);

        // don't do anything
        let rules = [];

        // move forward
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Moved { .. }));

        // move again -- should NOT collect trash because cleaning is OFF
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Moved { .. }));
        assert!(!result.collected_trash);
        assert_eq!(state.trash, vec![]);
    }

    #[test]
    fn test_exit_requires_trash_collection() {
        // start with trash *behind* us, exit in front
        let map = Map::parse("TSE").unwrap();
        let mut state = State::new((1, 0), Dir::East);

        // no rules, should move forward onto the trash
        let rules = [];

        // first tick should move forward
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Moved { .. }));

        // second tick should *not* exit, because we haven't collected
        // all of the trash
        let result = state.tick(&map, &rules);
        assert!(!state.is_finished);
        assert!(!matches!(result.motion, Motion::Exited));
    }

    #[test]
    fn test_trash_collection_while_bumping_wall() {
        // start on trash with wall in front
        let map = Map::parse("ST#").unwrap();
        let mut state = State::new((0, 0), Dir::East);

        // start cleaning at start
        let rules = [Rule::new(Sensor::Start, CleaningCommand::StartCleaning)];

        // move forward onto the trash
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::Moved { .. }));

        // now we're on trash with a wall ahead - should bump AND collect
        let result = state.tick(&map, &rules);
        assert!(matches!(result.motion, Motion::BumpedWall));
        assert!(result.collected_trash);
        assert_eq!(state.trash, vec![(1, 0).into()]);
    }
}
