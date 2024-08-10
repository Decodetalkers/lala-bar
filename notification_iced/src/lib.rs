//! # D-Bus interface proxy for: `org.freedesktop.Notifications`
//!
//! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
//! Source: `Interface '/org/freedesktop/Notifications' from service 'org.freedesktop.Notifications' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::{interface, object_server::SignalContext, zvariant::OwnedValue};

use futures::channel::mpsc::Sender;
use zbus::ConnectionBuilder;

use zbus::zvariant::{SerializeDict, Type};

pub const NOTIFICATION_DELETED_BY_EXPIRED: u32 = 1;
pub const NOTIFICATION_DELETED_BY_USER: u32 = 2;

pub const NOTIFICATION_CLOSED_BY_DBUS: u32 = 3;
pub const NOTIFICATION_CLOSED_BY_UNKNOWN_REASON: u32 = 4;

#[derive(Type, Debug, SerializeDict, OwnedValue, Clone)]
pub struct ImageData {
    width: i32,
    height: i32,
    rowstride: i32,
    has_alpha: bool,
    bits_per_sample: i32,
    channels: i32,
    data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum NotifyMessage {
    UnitAdd(NotifyUnit),
    UnitRemove(u32),
}

#[derive(Debug, Clone)]
pub struct NotifyHint {
    image_data: Option<ImageData>,
}

impl NotifyHint {
    pub fn image_data(&self) -> Option<(i32, i32, Vec<u8>)> {
        self.image_data
            .as_ref()
            .map(|data| (data.width, data.height, data.data.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct NotifyUnit {
    pub app_name: String,
    pub id: u32,
    pub icon: String,
    pub summery: String,
    pub body: String,
    pub actions: Vec<String>,
    pub timeout: i32,
    pub hint: NotifyHint,
}

impl NotifyUnit {
    pub fn inline_reply_support(&self) -> bool {
        self.actions.contains(&"inline-reply".to_owned())
    }
}

#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub spec_version: String,
}

#[derive(Debug)]
pub struct LaLaMako<T: From<NotifyMessage> + Send> {
    capabilities: Vec<String>,
    sender: Sender<T>,
    version: VersionInfo,
}

#[interface(name = "org.freedesktop.Notifications")]
impl<T: From<NotifyMessage> + Send + 'static> LaLaMako<T> {
    // CloseNotification method
    async fn close_notification(
        &mut self,
        #[zbus(signal_context)] ctx: SignalContext<'_>,
        id: u32,
    ) -> zbus::fdo::Result<()> {
        Self::notification_closed(&ctx, id, NOTIFICATION_DELETED_BY_USER)
            .await
            .ok();
        self.sender
            .try_send(NotifyMessage::UnitRemove(id).into())
            .ok();
        Ok(())
    }

    /// GetCapabilities method
    fn get_capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }

    /// GetServerInformation method
    fn get_server_information(&self) -> (String, String, String, String) {
        let VersionInfo {
            name,
            vendor,
            version,
            spec_version,
        } = &self.version;
        (
            name.clone(),
            vendor.clone(),
            version.clone(),
            spec_version.clone(),
        )
    }

    // Notify method
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &mut self,
        app_name: &str,
        id: u32,
        icon: &str,
        summery: &str,
        body: &str,
        actions: Vec<&str>,
        mut hints: std::collections::HashMap<&str, OwnedValue>,
        timeout: i32,
    ) -> zbus::fdo::Result<u32> {
        let image_data: Option<ImageData> =
            hints.remove("image-data").and_then(|v| v.try_into().ok());
        self.sender
            .try_send(
                NotifyMessage::UnitAdd(NotifyUnit {
                    app_name: app_name.to_string(),
                    id,
                    icon: icon.to_string(),
                    summery: summery.to_string(),
                    body: body.to_string(),
                    actions: actions.iter().map(|a| a.to_string()).collect(),
                    timeout,
                    hint: NotifyHint { image_data },
                })
                .into(),
            )
            .ok();
        Ok(0)
    }

    #[zbus(signal)]
    pub async fn action_invoked(
        ctx: &SignalContext<'_>,
        id: u32,
        action_key: &str,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn notification_replied(
        ctx: &SignalContext<'_>,
        id: u32,
        text: &str,
    ) -> zbus::Result<()>;

    // NotificationClosed signal
    #[zbus(signal)]
    pub async fn notification_closed(
        ctx: &SignalContext<'_>,
        id: u32,
        reason: u32,
    ) -> zbus::Result<()>;
}

pub const NOTIFICATION_SERVICE_PATH: &str = "/org/freedesktop/Notifications";
pub const NOTIFICATION_SERVICE_NAME: &str = "/org/freedesktop/Notifications";
pub const NOTIFICATION_SERVICE_INTERFACE: &str = "/org/freedesktop/Notifications";

pub const ACTION_INVOKED: &str = "action_invoked";
pub const NOTIFICATION_CLOSED: &str = "notification_closed";

pub const DEFAULT_ACTION: &str = "default";

pub async fn start_connection<T: From<NotifyMessage> + Send + 'static>(
    sender: Sender<T>,
    capabilities: Vec<String>,
    version: VersionInfo,
) -> Result<zbus::Connection, zbus::Error> {
    ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at(
            "/org/freedesktop/Notifications",
            LaLaMako {
                sender,
                capabilities,
                version,
            },
        )?
        .build()
        .await
}
