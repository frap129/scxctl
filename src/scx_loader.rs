use super::modes::Mode;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::{Connection, Proxy};
use dbus::Error;
use std::time::Duration;

const SCHED_PREFIX: &str = "scx_";
const SERVICE_NAME: &str = "org.scx.Loader";
const OBJECT_PATH: &str = "/org/scx/Loader";

enum ScxProperties {
    CurrentScheduler,
    SchedulerMode,
    SupportedSchedulers,
}

impl ScxProperties {
    fn as_str(&self) -> &str {
        match self {
            ScxProperties::CurrentScheduler => "CurrentScheduler",
            ScxProperties::SchedulerMode => "SchedulerMode",
            ScxProperties::SupportedSchedulers => "SupportedSchedulers",
        }
    }
}

enum ScxMethods {
    StartScheduler,
    StartSchedulerWithArgs,
    SwitchScheduler,
    SwitchSchedulerWithArgs,
    StopScheduler,
}

impl ScxMethods {
    fn as_str(&self) -> &str {
        match self {
            ScxMethods::StartScheduler => "StartScheduler",
            ScxMethods::StartSchedulerWithArgs => "StartSchedulerWithArgs",
            ScxMethods::SwitchScheduler => "SwitchScheduler",
            ScxMethods::SwitchSchedulerWithArgs => "SwitchSchedulerWithArgs",
            ScxMethods::StopScheduler => "StopScheduler",
        }
    }
}

fn ensure_scx_prefix(input: String) -> String {
    if !input.starts_with(SCHED_PREFIX) {
        return format!("{}{}", SCHED_PREFIX, input);
    }
    input
}

fn remove_scx_prefix(input: String) -> String {
    if input.starts_with(SCHED_PREFIX) {
        return input[SCHED_PREFIX.len()..].to_string();
    }
    input
}

pub struct ScxLoader<'a> {
    proxy: Proxy<'a, &'a Connection>,
}

impl<'a> ScxLoader<'a> {
    pub fn new(conn: &'a Connection) -> Result<Self, Error> {
        Ok(ScxLoader {
            proxy: conn.with_proxy(SERVICE_NAME, OBJECT_PATH, Duration::from_millis(5000)),
        })
    }

    pub fn get_supported_schedulers(&self) -> Result<Vec<String>, Error> {
        let supported_scheds: Vec<String> = self
            .proxy
            .get(SERVICE_NAME, ScxProperties::SupportedSchedulers.as_str())?;
        Ok(supported_scheds
            .into_iter()
            .map(|s| remove_scx_prefix(s))
            .collect())
    }

    pub fn get_sched(&self) -> Result<String, Error> {
        let current_sched = self
            .proxy
            .get(SERVICE_NAME, ScxProperties::CurrentScheduler.as_str())?;
        Ok(remove_scx_prefix(current_sched))
    }

    pub fn get_mode(&self) -> Result<Mode, Error> {
        let raw_mode: u32 = self
            .proxy
            .get(SERVICE_NAME, ScxProperties::SchedulerMode.as_str())?;
        Ok(Mode::from_u32(raw_mode).unwrap())
    }
    pub fn start(&self, sched: String, mode: Option<Mode>) -> Result<(String, Mode), Error> {
        let mode = mode.unwrap_or_else(|| self.get_mode().unwrap());
        let _: () = self.proxy.method_call(
            SERVICE_NAME,
            ScxMethods::StartScheduler.as_str(),
            (ensure_scx_prefix(sched), mode.as_u32()),
        )?;
        Ok((remove_scx_prefix(self.get_sched()?), self.get_mode()?))
    }

    pub fn start_with_args(
        &self,
        sched: String,
        args: Vec<String>,
    ) -> Result<(String, Vec<String>), Error> {
        let _: () = self.proxy.method_call(
            SERVICE_NAME,
            ScxMethods::StartSchedulerWithArgs.as_str(),
            (ensure_scx_prefix(sched), args.clone()),
        )?;
        Ok((remove_scx_prefix(self.get_sched()?), args))
    }

    pub fn switch(
        &self,
        sched: Option<String>,
        mode: Option<Mode>,
    ) -> Result<(String, Mode), Error> {
        let sched = sched.unwrap_or_else(|| self.get_sched().unwrap());
        let mode = mode.unwrap_or_else(|| self.get_mode().unwrap());

        let _: () = self.proxy.method_call(
            SERVICE_NAME,
            ScxMethods::SwitchScheduler.as_str(),
            (ensure_scx_prefix(sched), mode.as_u32()),
        )?;
        Ok((remove_scx_prefix(self.get_sched()?), self.get_mode()?))
    }

    pub fn switch_with_args(
        &self,
        sched: Option<String>,
        args: Vec<String>,
    ) -> Result<(String, Vec<String>), Error> {
        let sched = sched.unwrap_or_else(|| self.get_sched().unwrap());
        let _: () = self.proxy.method_call(
            SERVICE_NAME,
            ScxMethods::SwitchSchedulerWithArgs.as_str(),
            (ensure_scx_prefix(sched), args.clone()),
        )?;
        Ok((remove_scx_prefix(self.get_sched()?), args))
    }

    pub fn stop(&self) -> Result<(), Error> {
        self.proxy
            .method_call(SERVICE_NAME, ScxMethods::StopScheduler.as_str(), ())
    }
}
