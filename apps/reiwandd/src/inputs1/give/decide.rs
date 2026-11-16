use crate::callers::caller;
use crate::compositor::lock;
use crate::error::{Error, Result, invalid};
use crate::inputs::grants_path;
use reiwand_core::model::Input;
use zbus::message::Header;

type Answers = Vec<(Input, bool)>;

impl crate::inputs1::Inputs {
    pub(in crate::inputs1) async fn decide_all(
        &self,
        call: (&zbus::Connection, &Header<'_>),
        scene: &str,
        key: &str,
        (allow, deny): (Vec<String>, Vec<String>),
    ) -> Result<()> {
        let (d, hub) = self.parts()?;
        let scene = crate::inputs1::ids::scene_id(scene)?.to_owned();
        let answers = answers(&allow, &deny)?;
        let (key, title) = lock(&hub.control).package(&scene, (!key.is_empty()).then_some(key))?;
        let (now, asked) = split(caller(call.0, call.1).await.trusted(), answers);
        let (kept, scene_id, key_id) = (hub.clone(), scene.clone(), key.clone());
        d.run(move |d| {
            for (input, on) in now {
                match on {
                    true => kept.grant(&grants_path(d), &scene_id, &key_id, input)?,
                    false => kept.decline(&grants_path(d), &scene_id, &key_id, input)?,
                }
                crate::inputs1::ids::reopen(d, input)?;
            }
            Ok::<(), Error>(())
        })
        .await?;
        if asked.is_empty() {
            return Ok(());
        }
        crate::confirm::decide(call.0, hub.clone(), grants_path(&d), (scene, key, title), asked).await
    }
}

fn answers(allow: &[String], deny: &[String]) -> Result<Answers> {
    let answers: Result<Answers> = allow
        .iter()
        .map(|i| Ok((crate::inputs1::ids::parse(i)?, true)))
        .chain(deny.iter().map(|i| Ok((crate::inputs1::ids::parse(i)?, false))))
        .collect();
    let answers = answers?;
    let clash = |a: &(Input, bool)| answers.iter().any(|b| b.0 == a.0 && b.1 != a.1);
    if answers.iter().any(clash) {
        return Err(invalid("an input cannot be both allowed and left out"));
    }
    Ok(answers)
}

fn split(trusted: bool, answers: Answers) -> (Answers, Vec<Input>) {
    if trusted {
        return (answers, Vec::new());
    }
    let (allow, deny): (Answers, Answers) = answers.into_iter().partition(|a| a.1);
    (deny, allow.into_iter().map(|a| a.0).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_known_caller_decides_at_once_and_another_only_declines_at_once() {
        let answers = answers(&["audio".into(), "network".into()], &["pointer".into()]).unwrap();
        let all = vec![(Input::Audio, true), (Input::Network, true), (Input::Pointer, false)];
        assert_eq!(split(true, answers.clone()), (all, Vec::new()), "a known caller now waits");
        let (now, asked) = split(false, answers);
        assert_eq!(now, vec![(Input::Pointer, false)], "a decline waited for the user");
        assert_eq!(asked, vec![Input::Audio, Input::Network], "an allowed input was not left to the user");
        assert!(super::answers(&["audio".into()], &["audio".into()]).is_err());
    }
}
