use aos2_env::AoS2Env;
use online_profile::{avatar, title, PlayerOnlineProfile};
use tokio::sync::watch;

use super::{channel::Channel, Error};

#[derive(Debug, Clone)]
pub struct Profile {
    env: AoS2Env,
    profile: Channel<PlayerOnlineProfile>,
}

trait GetFn<T>: Send + Fn(&PlayerOnlineProfile) -> T {}
trait ModifyFn<T>: Send + Fn(&mut PlayerOnlineProfile, T) {}

impl<A, T> GetFn<T> for A where A: Send + Fn(&PlayerOnlineProfile) -> T {}

impl<A, T> ModifyFn<T> for A where A: Send + Fn(&mut PlayerOnlineProfile, T) {}

pub struct Modify<T> {
    profile: watch::Sender<PlayerOnlineProfile>,
    modify: Box<dyn ModifyFn<T>>,
    get: Box<dyn GetFn<T>>,
}

impl Profile {
    pub fn load(env: AoS2Env) -> Result<Self, Error> {
        let profile = PlayerOnlineProfile::load(&env)?.ok_or(Error::MissingProfile)?;
        Ok(Self {
            env,
            profile: Channel::new(profile),
        })
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if self.profile.has_changed() {
            self.profile
                .borrow_and_update()
                .save(&self.env)
                .map_err(Error::Profile)
        } else {
            Ok(())
        }
    }

    #[must_use]
    pub fn modify_title_character(&self) -> Modify<title::Character> {
        Modify {
            profile: self.profile.sender(),
            modify: Box::new(
                |profile: &mut PlayerOnlineProfile, character: title::Character| {
                    profile.title_character_in_background = character;
                },
            ),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.title_character_in_background),
        }
    }

    #[must_use]
    pub fn modify_title_color(&self) -> Modify<title::Color> {
        Modify {
            profile: self.profile.sender(),
            modify: Box::new(|profile: &mut PlayerOnlineProfile, color: title::Color| {
                profile.title_color = color;
            }),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.title_color),
        }
    }

    #[must_use]
    pub fn modify_title_text(&self) -> Modify<title::Text> {
        Modify {
            profile: self.profile.sender(),
            modify: Box::new(|profile: &mut PlayerOnlineProfile, text: title::Text| {
                profile.title_text_id = text;
            }),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.title_text_id),
        }
    }

    #[must_use]
    pub fn modify_avatar_character(&self) -> Modify<avatar::Character> {
        Modify {
            profile: self.profile.sender(),
            modify: Box::new(
                |profile: &mut PlayerOnlineProfile, character: avatar::Character| {
                    profile.avatar_character = character;
                },
            ),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.avatar_character),
        }
    }

    #[must_use]
    pub fn modify_avatar_background(&self) -> Modify<avatar::Background> {
        Modify {
            profile: self.profile.sender(),
            modify: Box::new(
                |profile: &mut PlayerOnlineProfile, background: avatar::Background| {
                    profile.avatar_background = background;
                },
            ),
            get: Box::new(|profile: &PlayerOnlineProfile| profile.avatar_background),
        }
    }
}

impl<T> Modify<T> {
    #[must_use]
    pub fn get(&self) -> T {
        let profile = self.profile.borrow();
        (self.get)(&profile)
    }

    pub fn send(&mut self, value: T) {
        self.profile.send_modify(|profile| {
            (self.modify)(profile, value);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::Modify;

    fn ensure_send<T: Send>() {}

    #[rstest::rstest]
    fn modify_profile_is_send() {
        ensure_send::<Modify<()>>();
    }
}
