// This code was autogenerated with `dbus-codegen-rust --client nonblock --destination org.freedesktop.Notifications --path /org/freedesktop/Notifications`, see https://github.com/diwic/dbus-rs
use dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;

pub trait OrgFreedesktopDBusProperties {
    fn get(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> nonblock::MethodReply<arg::Variant<Box<dyn arg::RefArg + 'static>>>;
    fn get_all(&self, interface_name: &str) -> nonblock::MethodReply<arg::PropMap>;
    fn set(
        &self,
        interface_name: &str,
        property_name: &str,
        value: arg::Variant<Box<dyn arg::RefArg>>,
    ) -> nonblock::MethodReply<()>;
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgFreedesktopDBusProperties
    for nonblock::Proxy<'a, C>
{
    fn get(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> nonblock::MethodReply<arg::Variant<Box<dyn arg::RefArg + 'static>>> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "Get",
            (interface_name, property_name),
        )
        .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>,)| Ok(r.0))
    }

    fn get_all(&self, interface_name: &str) -> nonblock::MethodReply<arg::PropMap> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "GetAll",
            (interface_name,),
        )
        .and_then(|r: (arg::PropMap,)| Ok(r.0))
    }

    fn set(
        &self,
        interface_name: &str,
        property_name: &str,
        value: arg::Variant<Box<dyn arg::RefArg>>,
    ) -> nonblock::MethodReply<()> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "Set",
            (interface_name, property_name, value),
        )
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopDBusIntrospectable for nonblock::Proxy<'a, C>
{
    fn introspect(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> nonblock::MethodReply<()>;
    fn get_machine_id(&self) -> nonblock::MethodReply<String>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgFreedesktopDBusPeer
    for nonblock::Proxy<'a, C>
{
    fn ping(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> nonblock::MethodReply<String> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait OrgDunstprojectCmd0 {
    fn context_menu_call(&self) -> nonblock::MethodReply<()>;
    fn notification_action(&self, number: u32) -> nonblock::MethodReply<()>;
    fn notification_clear_history(&self) -> nonblock::MethodReply<()>;
    fn notification_close_last(&self) -> nonblock::MethodReply<()>;
    fn notification_close_all(&self) -> nonblock::MethodReply<()>;
    fn notification_list_history(&self) -> nonblock::MethodReply<Vec<arg::PropMap>>;
    fn notification_pop_history(&self, id: u32) -> nonblock::MethodReply<()>;
    fn notification_remove_from_history(&self, id: u32) -> nonblock::MethodReply<()>;
    fn notification_show(&self) -> nonblock::MethodReply<()>;
    fn rule_enable(&self, name: &str, state: i32) -> nonblock::MethodReply<()>;
    fn ping(&self) -> nonblock::MethodReply<()>;
    fn paused(&self) -> nonblock::MethodReply<bool>;
    fn setpaused(&self, value: bool) -> nonblock::MethodReply<()>;
    fn displayed_length(&self) -> nonblock::MethodReply<u32>;
    fn history_length(&self) -> nonblock::MethodReply<u32>;
    fn waiting_length(&self) -> nonblock::MethodReply<u32>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgDunstprojectCmd0
    for nonblock::Proxy<'a, C>
{
    fn context_menu_call(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.dunstproject.cmd0", "ContextMenuCall", ())
    }

    fn notification_action(&self, number: u32) -> nonblock::MethodReply<()> {
        self.method_call("org.dunstproject.cmd0", "NotificationAction", (number,))
    }

    fn notification_clear_history(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.dunstproject.cmd0", "NotificationClearHistory", ())
    }

    fn notification_close_last(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.dunstproject.cmd0", "NotificationCloseLast", ())
    }

    fn notification_close_all(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.dunstproject.cmd0", "NotificationCloseAll", ())
    }

    fn notification_list_history(&self) -> nonblock::MethodReply<Vec<arg::PropMap>> {
        self.method_call("org.dunstproject.cmd0", "NotificationListHistory", ())
            .and_then(|r: (Vec<arg::PropMap>,)| Ok(r.0))
    }

    fn notification_pop_history(&self, id: u32) -> nonblock::MethodReply<()> {
        self.method_call("org.dunstproject.cmd0", "NotificationPopHistory", (id,))
    }

    fn notification_remove_from_history(&self, id: u32) -> nonblock::MethodReply<()> {
        self.method_call(
            "org.dunstproject.cmd0",
            "NotificationRemoveFromHistory",
            (id,),
        )
    }

    fn notification_show(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.dunstproject.cmd0", "NotificationShow", ())
    }

    fn rule_enable(&self, name: &str, state: i32) -> nonblock::MethodReply<()> {
        self.method_call("org.dunstproject.cmd0", "RuleEnable", (name, state))
    }

    fn ping(&self) -> nonblock::MethodReply<()> {
        self.method_call("org.dunstproject.cmd0", "Ping", ())
    }

    fn paused(&self) -> nonblock::MethodReply<bool> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.dunstproject.cmd0",
            "paused",
        )
    }

    fn displayed_length(&self) -> nonblock::MethodReply<u32> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.dunstproject.cmd0",
            "displayedLength",
        )
    }

    fn history_length(&self) -> nonblock::MethodReply<u32> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.dunstproject.cmd0",
            "historyLength",
        )
    }

    fn waiting_length(&self) -> nonblock::MethodReply<u32> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.dunstproject.cmd0",
            "waitingLength",
        )
    }

    fn setpaused(&self, value: bool) -> nonblock::MethodReply<()> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::set(
            &self,
            "org.dunstproject.cmd0",
            "paused",
            value,
        )
    }
}

pub trait OrgFreedesktopNotifications {
    fn get_capabilities(&self) -> nonblock::MethodReply<Vec<String>>;
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: arg::PropMap,
        expire_timeout: i32,
    ) -> nonblock::MethodReply<u32>;
    fn close_notification(&self, id: u32) -> nonblock::MethodReply<()>;
    fn get_server_information(&self) -> nonblock::MethodReply<(String, String, String, String)>;
}

#[derive(Debug)]
pub struct OrgFreedesktopNotificationsNotificationClosed {
    pub id: u32,
    pub reason: u32,
}

impl arg::AppendAll for OrgFreedesktopNotificationsNotificationClosed {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.id, i);
        arg::RefArg::append(&self.reason, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNotificationsNotificationClosed {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNotificationsNotificationClosed {
            id: i.read()?,
            reason: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNotificationsNotificationClosed {
    const NAME: &'static str = "NotificationClosed";
    const INTERFACE: &'static str = "org.freedesktop.Notifications";
}

#[derive(Debug)]
pub struct OrgFreedesktopNotificationsActionInvoked {
    pub id: u32,
    pub action_key: String,
}

impl arg::AppendAll for OrgFreedesktopNotificationsActionInvoked {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.id, i);
        arg::RefArg::append(&self.action_key, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNotificationsActionInvoked {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNotificationsActionInvoked {
            id: i.read()?,
            action_key: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNotificationsActionInvoked {
    const NAME: &'static str = "ActionInvoked";
    const INTERFACE: &'static str = "org.freedesktop.Notifications";
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target = T>> OrgFreedesktopNotifications
    for nonblock::Proxy<'a, C>
{
    fn get_capabilities(&self) -> nonblock::MethodReply<Vec<String>> {
        self.method_call("org.freedesktop.Notifications", "GetCapabilities", ())
            .and_then(|r: (Vec<String>,)| Ok(r.0))
    }

    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<&str>,
        hints: arg::PropMap,
        expire_timeout: i32,
    ) -> nonblock::MethodReply<u32> {
        self.method_call(
            "org.freedesktop.Notifications",
            "Notify",
            (
                app_name,
                replaces_id,
                app_icon,
                summary,
                body,
                actions,
                hints,
                expire_timeout,
            ),
        )
        .and_then(|r: (u32,)| Ok(r.0))
    }

    fn close_notification(&self, id: u32) -> nonblock::MethodReply<()> {
        self.method_call("org.freedesktop.Notifications", "CloseNotification", (id,))
    }

    fn get_server_information(&self) -> nonblock::MethodReply<(String, String, String, String)> {
        self.method_call("org.freedesktop.Notifications", "GetServerInformation", ())
    }
}
