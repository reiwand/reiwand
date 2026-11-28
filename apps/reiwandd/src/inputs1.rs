use crate::callers::{Caller, caller};
use crate::compositor::lock;
use crate::daemon::Daemon;
use crate::error::{Result, denied, failed, invalid};
use crate::inputs::{Hub, hub};
use reiwand_core::model::Input;
use std::sync::{Arc, Weak};
use zbus::message::Header;

mod give;
mod ids;
mod pictures;
mod told;
pub(crate) use give::start;
use ids::parse;
#[cfg(test)]
pub(crate) use ids::scene_id;

pub struct Inputs(pub Weak<Daemon>);

impl Inputs {
    fn parts(&self) -> Result<(Arc<Daemon>, &'static Arc<Hub>)> {
        let stopping = || failed("the daemon is stopping");
        Ok((self.0.upgrade().ok_or_else(stopping)?, hub().ok_or_else(stopping)?))
    }
}

#[zbus::interface(name = "io.github.reiwand.Inputs1")]
impl Inputs {
    async fn grant(
        &self,
        scene: String,
        input: String,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<()> {
        self.give((bus, &header), &scene, None, &input).await
    }

    async fn grant_package(
        &self,
        scene: String,
        key: String,
        input: String,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<()> {
        self.give((bus, &header), &scene, Some(&key), &input).await
    }

    async fn decide(
        &self,
        scene: String,
        key: String,
        allow: Vec<String>,
        deny: Vec<String>,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<()> {
        self.decide_all((bus, &header), &scene, &key, (allow, deny)).await
    }

    async fn revoke(&self, scene: String, input: String) -> Result<()> {
        let (d, hub) = self.parts()?;
        let input = parse(&input)?;
        d.run(move |d| pictures::revoke(d, hub, &scene, input)).await
    }

    #[zbus(out_args("json"))]
    async fn get_inputs(&self) -> Result<String> {
        let (d, hub) = self.parts()?;
        told::inputs(&d, hub)
    }

    async fn simulate_media(
        &self,
        output: String,
        track: String,
        cover: Vec<u8>,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<()> {
        if !caller(bus, &header).await.trusted() {
            return Err(denied("only the app, the tray and reiwand-ctl simulate a media player"));
        }
        told::simulate(output, track, cover).await
    }

    async fn simulate_touch(
        &self,
        output: String,
        x: f64,
        y: f64,
        buttons: u32,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<()> {
        if !caller(bus, &header).await.trusted() {
            return Err(denied("only the app, the tray and reiwand-ctl simulate a pointer"));
        }
        let inside = [x, y].iter().all(|v| (0.0..=1.0).contains(v));
        let pos = inside.then_some([x as f32, y as f32]);
        crate::inputs::surface_pointer(&output, pos, buttons.min(255) as u8);
        Ok(())
    }

    async fn simulate_trigger(
        &self,
        output: String,
        trigger: String,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<()> {
        if !caller(bus, &header).await.trusted() {
            return Err(denied("only the app, the tray and reiwand-ctl fire a trigger"));
        }
        if trigger.is_empty() || trigger.len() > 256 {
            return Err(invalid("a trigger has 1 to 256 bytes, such as event:boom or prop:flash"));
        }
        self.parts()?.0.compositor.fire(&output, &trigger);
        Ok(())
    }

    #[zbus(out_args("json"))]
    async fn bands(
        &self,
        output: String,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<String> {
        if !caller(bus, &header).await.trusted() {
            return Err(denied("only the app, the tray and reiwand-ctl read the levels of the sound"));
        }
        Ok(crate::inputs::bands(&output).to_string())
    }

    async fn set_clicks(
        &self,
        output: String,
        on: bool,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<()> {
        let (d, _) = self.parts()?;
        if on && !caller(bus, &header).await.trusted() {
            return Err(denied("only the app, the tray and reiwand-ctl let clicks reach a wallpaper"));
        }
        let set = move |c: &mut reiwand_core::Config| {
            c.set_clicks(output.trim(), on);
            Ok(())
        };
        d.run(move |d| d.update_config(set)).await
    }

    async fn touch(
        &self,
        output: String,
        x: f64,
        y: f64,
        buttons: u32,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<()> {
        if caller(bus, &header).await != Caller::Shell || crate::flatpak::app_id().is_some() {
            return Err(denied("only the Plasma wallpaper of reiwandd reports clicks"));
        }
        let hub = self.parts()?.1;
        if !lock(&hub.control).wants(Input::Clicks).contains(&output) {
            return Err(denied(format!("{output} does not take clicks now")));
        }
        let inside = [x, y].iter().all(|v| (0.0..=1.0).contains(v));
        if inside && hub.away(&output) {
            return Err(denied(format!("the pointer is not on {output}")));
        }
        let pos = inside.then_some([x as f32, y as f32]);
        crate::inputs::surface_pointer(&output, pos, buttons.min(255) as u8);
        Ok(())
    }

    async fn set_pointer(
        &self,
        x: i32,
        y: i32,
        #[zbus(header)] header: Header<'_>,
        #[zbus(connection)] bus: &zbus::Connection,
    ) -> Result<()> {
        if !crate::callers::reports_pointer(caller(bus, &header).await) {
            return Err(denied(match crate::flatpak::app_id() {
                Some(_) => {
                    "only the KWin script of reiwandd reports the pointer: inside the Flatpak, reiwand-ctl cannot be told apart from other programs"
                }
                None => "only the KWin script of reiwandd and reiwand-ctl report the pointer",
            }));
        }
        self.parts()?.1.set_pointer(x.into(), y.into());
        Ok(())
    }
}
