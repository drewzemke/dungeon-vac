use crate::game::{action::Action, event::Event};

#[derive(Debug)]
pub struct Rule {
    event: Event,
    action: Action,
}

impl Rule {
    pub const fn new(event: Event, action: Action) -> Self {
        Self { event, action }
    }

    pub fn compute_actions(rules: &[Rule], events: &[Event]) -> Vec<Action> {
        let mut actions = Vec::new();

        for rule in rules {
            for event in events {
                // check for rule match
                if rule.event != *event {
                    continue;
                }

                // filter out actions that already have a member of their category in
                // the output actions
                // NOTE: this is trivial at the moment as there's only one category
                let category_already_represented = !actions.is_empty();
                if !category_already_represented {
                    actions.push(rule.action);
                }
            }
        }

        // if no movement action was fired, fall back to moving forward
        // NOTE: update this logic when we add categories
        if actions.is_empty() {
            actions.push(Action::MoveForward);
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_actions_default_move() {
        let rules = [Rule::new(Event::HitWall, Action::TurnRight)];
        let events = [Event::SpaceLeft];

        let actions = Rule::compute_actions(&rules, &events);
        assert_eq!(actions, vec![Action::MoveForward]);
    }

    #[test]
    fn compute_actions_single() {
        let rules = [Rule::new(Event::HitWall, Action::TurnRight)];
        let events = [Event::HitWall];

        let actions = Rule::compute_actions(&rules, &events);
        assert_eq!(actions, vec![Action::TurnRight]);
    }

    #[test]
    fn compute_actions_same_category() {
        let rules = [
            Rule::new(Event::HitWall, Action::TurnRight),
            Rule::new(Event::HitWall, Action::TurnLeft),
        ];
        let events = [Event::HitWall];

        // only the first matching rule should trigger
        let actions = Rule::compute_actions(&rules, &events);
        assert_eq!(actions, vec![Action::TurnRight]);
    }
}
