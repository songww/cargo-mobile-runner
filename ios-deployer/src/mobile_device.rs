use super::mobile_device_errors as errors;
use super::mobile_device_sys as ffi;

use std::collections::HashMap;
use std::ffi::c_void;
use std::ffi::CStr;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Read, Result as IOResult, Write};
use std::os::raw::c_char;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;

use core_foundation::{
    array::CFArray, base::CFRelease, base::CFRetain, base::CFType, base::FromVoid, base::TCFType,
    dictionary::CFDictionary, dictionary::CFMutableDictionary, string::CFString,
    string::CFStringRef,
};

fn as_c_char<P: AsRef<Path>>(path: P) -> *const c_char {
    path.as_ref().as_os_str().as_bytes().as_ptr() as *const _
}

pub enum AMServiceName {
    Afc,
    Backup,
    CrashReportCopy,
    DebugImageMount,
    NotificationProxy,
    PurpleTest,
    SoftwareUpdate,
    Sync,
    Screeshot,
    SyslogRelay,
    SystemProfiler,
}

macro_rules! cfstr {
    ($str:expr) => {
        CFString::from_static_string($str).as_concrete_TypeRef()
    };
}

macro_rules! cfstr1 {
    ($str:expr) => {
        CFString::new($str).as_concrete_TypeRef()
    };
}

macro_rules! cfstr2 {
    ($path:expr) => {
        CFString::new($path.to_str().unwrap()).as_concrete_TypeRef()
    };
}

pub fn amd_set_log_level(level: i32) {
    unsafe { ffi::AMDSetLogLevel(level) }
}

pub fn afc_platform_init() {
    unsafe {
        ffi::AFCPlatformInit();
    }
}

impl AMServiceName {
    fn to_cfstring_ref(&self) -> CFStringRef {
        match self {
            AMServiceName::Afc => cfstr!(ffi::AMSVC_AFC),
            AMServiceName::Backup => cfstr!(ffi::AMSVC_BACKUP),
            AMServiceName::CrashReportCopy => cfstr!(ffi::AMSVC_CRASH_REPORT_COPY),
            AMServiceName::DebugImageMount => cfstr!(ffi::AMSVC_DEBUG_IMAGE_MOUNT),
            AMServiceName::NotificationProxy => cfstr!(ffi::AMSVC_NOTIFICATION_PROXY),
            AMServiceName::PurpleTest => cfstr!(ffi::AMSVC_PURPLE_TEST),
            AMServiceName::SoftwareUpdate => cfstr!(ffi::AMSVC_SOFTWARE_UPDATE),
            AMServiceName::Sync => cfstr!(ffi::AMSVC_SYNC),
            AMServiceName::Screeshot => cfstr!(ffi::AMSVC_SCREENSHOT),
            AMServiceName::SyslogRelay => cfstr!(ffi::AMSVC_SYSLOG_RELAY),
            AMServiceName::SystemProfiler => cfstr!(ffi::AMSVC_SYSTEM_PROFILER),
        }
    }
}

pub struct AMServiceConn(ffi::ServiceConnRef);

impl AMServiceConn {
    pub fn as_ptr(&self) -> *const ffi::service_conn_t {
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> ffi::ServiceConnRef {
        self.0
    }

    fn new() -> Self {
        Self(ptr::null_mut())
    }

    pub fn open_afc_connection(
        &mut self,
        io_timeout: u32,
    ) -> errors::AMDeviceResult<AFCConnection> {
        let mut conn = AFCConnection(ptr::null_mut());
        errors::to_result(unsafe {
            ffi::AFCConnectionOpen(self.0, io_timeout, &mut conn.as_mut_ptr() as *mut _)
        })
        .and_then(|_| Ok(conn))
    }

    pub fn post_notification(
        &mut self,
        notification: &str,
        userinfo: &str,
    ) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe {
            ffi::AMDPostNotification(*self.0, cfstr1!(notification), cfstr1!(userinfo))
        })
    }

    pub fn install_application<P: AsRef<Path>>(
        &mut self,
        path: P,
        options: &HashMap<CFType, CFType>,
        callback: Option<fn(&HashMap<CFType, CFType>) -> i32>,
    ) -> errors::AMDeviceResult<()> {
        let options: CFDictionary<CFType, CFType> = CFMutableDictionary::new().to_immutable();
        errors::to_result(unsafe {
            ffi::AMDeviceInstallApplication(
                *self.0,
                cfstr2!(path.as_ref()),
                options.as_concrete_TypeRef(),
                None,
                ptr::null_mut(),
            )
        })
    }

    pub fn transfer_application<P: AsRef<Path>>(
        &mut self,
        path: P,
        options: &HashMap<CFType, CFType>,
        callback: Option<fn(&HashMap<CFType, CFType>) -> i32>,
    ) -> errors::AMDeviceResult<()> {
        // let options = CFDictionary::from_CFType_pairs(options.iter().collect()).as_concrete_typeref();
        let options: CFDictionary<CFType, CFType> = CFMutableDictionary::new().to_immutable();
        errors::to_result(unsafe {
            ffi::AMDeviceTransferApplication(
                *self.0,
                cfstr2!(path.as_ref()),
                options.as_concrete_TypeRef(),
                None,
                ptr::null_mut(),
            )
        })
    }

    pub fn socket(&mut self) -> ffi::CFSocketNativeHandle {
        unsafe { ffi::AMDServiceConnectionGetSocket(self.0) }
    }

    pub fn invalidate(&mut self) {
        unsafe { ffi::AMDServiceConnectionInvalidate(self.0) }
    }

    /*
    pub fn send(&mut self, data: *const c_void) {
        // unsafe { ffi::AMDServiceConnectionSend(self.0, data, {})
    }
    pub fn receive(&mut self) ->  *const c_void {
        // unsafe { ffi::AMDServiceConnectionReceive(self.0, data, {})
    }
    */
}

pub trait AMDeviceNotificationSubscriber {
    fn on_connected(&mut self, device: &mut AMDevice, msg: u32);
    fn on_disconnected(&mut self, device: &mut AMDevice, msg: u32);
}

pub struct NotificationSubscriber;

impl AMDeviceNotificationSubscriber for NotificationSubscriber {
    fn on_connected(&mut self, device: &mut AMDevice, code: u32) {
        println!(
            "Device connected ({}): {}",
            code,
            device.interface_type().to_string()
        );
    }
    fn on_disconnected(&mut self, device: &mut AMDevice, code: u32) {
        println!(
            "Disconnected {} from {} ({})",
            device.device_identifier(),
            device.interface_type().to_string(),
            code
        );
    }
}

#[derive(Debug)]
#[repr(i32)]
pub enum AMDeviceInterfaceType {
    Usb = 1,
    Wifi = 2,
    Unknown,
}

impl From<i32> for AMDeviceInterfaceType {
    fn from(code: i32) -> AMDeviceInterfaceType {
        match code {
            1 => AMDeviceInterfaceType::Usb,
            2 => AMDeviceInterfaceType::Wifi,
            _ => AMDeviceInterfaceType::Unknown,
        }
    }
}

impl ToString for AMDeviceInterfaceType {
    fn to_string(&self) -> String {
        match self {
            AMDeviceInterfaceType::Usb => "USB".to_string(),
            AMDeviceInterfaceType::Wifi => "WIFI".to_string(),
            AMDeviceInterfaceType::Unknown => "Unknown Connection".to_string(),
        }
    }
}

pub struct AMDevice(ffi::AMDeviceRef);

impl AMDevice {
    fn as_ptr(&self) -> *const ffi::am_device {
        self.0
    }

    fn as_mut_ptr(&mut self) -> ffi::AMDeviceRef {
        self.0
    }

    pub fn connect(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AMDeviceConnect(self.0) })
    }

    pub fn disconnect(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AMDeviceDisconnect(self.0) })
    }

    pub fn retain(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AMDeviceRetain(self.0) })
    }

    pub fn release(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AMDeviceRelease(self.0) })
    }

    pub fn get_value(&mut self, key: &str, arg1: Option<&mut str>) -> String {
        let key = cfstr1!(key);
        let ns = arg1
            .and_then(|ns| Some(ns.as_mut_ptr()))
            .or_else(|| Some(ptr::null_mut()))
            .unwrap();
        let v = unsafe { ffi::AMDeviceCopyValue(self.0, ptr::null_mut(), key) };
        if v.is_null() {
            return "".to_string();
        }

        unsafe { CFString::from_void(v as *const c_void) }.to_string()
    }

    pub fn device_identifier(&mut self) -> String {
        let v = unsafe { ffi::AMDeviceCopyDeviceIdentifier(self.0) };
        unsafe { CFString::from_void(v as *const c_void) }.to_string()
    }

    pub fn devices() -> Vec<AMDevice> {
        let ref_ = unsafe { ffi::AMDCreateDeviceList() };
        let array: CFArray<ffi::AMDeviceRef> = unsafe { CFArray::wrap_under_get_rule(ref_) };
        let devs: Vec<_> = array
            .get_all_values()
            .into_iter()
            .map(|ref_| AMDevice(unsafe { CFRetain(ref_) } as *mut c_void as *mut _))
            .collect();
        // println!("{:?}", devs.len());
        devs
    }

    pub fn notification_subscribe<S: AMDeviceNotificationSubscriber>(subsrciber: S) {
        let mut notify = ffi::am_device_notification {
            unknown0: 0,
            unknown1: 0,
            unknown2: 0,
            callback: None,
            unknown3: 0,
        };

        let mut callback = Some(device_notification_callback);
        unsafe {
            ffi::AMDeviceNotificationSubscribe(
                None,
                0,
                0,
                ptr::null_mut(),
                &mut (&mut notify as *mut _) as *mut _,
            );
        }
    }

    pub fn is_paired(&mut self) -> bool {
        let b = unsafe { ffi::AMDeviceIsPaired(self.0) };
        b == 1
    }

    pub fn validate_pairing(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AMDeviceValidatePairing(self.0) })
    }

    pub fn start_session(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AMDeviceStartSession(self.0) })
    }

    pub fn start_service(
        &mut self,
        service_name: AMServiceName,
    ) -> errors::AMDeviceResult<AMServiceConn> {
        let mut service_conn = AMServiceConn::new();
        errors::to_result(unsafe {
            ffi::AMDeviceStartService(
                self.0,
                service_name.to_cfstring_ref(),
                &mut service_conn.as_mut_ptr() as *mut _,
                ptr::null_mut(),
            )
        })?;
        Ok(service_conn)
    }

    pub fn secure_start_service(
        &mut self,
        service_name: AMServiceName,
    ) -> errors::AMDeviceResult<AMServiceConn> {
        let mut service_conn = AMServiceConn::new();
        errors::to_result(unsafe {
            ffi::AMDeviceSecureStartService(
                self.0,
                service_name.to_cfstring_ref(),
                ptr::null_mut(),
                &mut service_conn.as_mut_ptr() as *mut _,
            )
        })?;
        Ok(service_conn)
    }

    // pub fn start_house_arrest_service(&mut self, identifier: &str, service_conn: &mut AMServiceConn, what) -> errors::AMDeviceResult<()>
    // pub fn create_house_arrest_service(&mut self, identifier: &str, options) ->
    // errors::AMDeviceResult<AFCConnection>;

    pub fn stop_session(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AMDeviceStopSession(self.0) })
    }

    pub fn connection_id(&mut self) -> u32 {
        unsafe { ffi::AMDeviceGetConnectionID(self.0) }
    }

    pub fn enter_recovery(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AMDeviceEnterRecovery(self.0) })
    }

    pub fn deactivate(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AMDeviceDeactivate(self.0) })
    }
    // pub fn activate(&mut self, args: HashMap<>) -> errors::AMDeviceResult<()> {}

    pub fn serialize(&mut self) -> String {
        let v = unsafe { ffi::AMDeviceSerialize(self.0) };
        String::new()
    }

    // pub fn secure_uninstall_application<P: AsRef<Path>>(&mut self, path: P, options: HashMap<>, callback: ) -> errors::AMDeviceResult<()> {
    // }

    // pub fn secure_transfer_path()
    pub fn secure_install_application(&mut self, url: &Path, options: &HashMap<CFType, CFType>) {}
    pub fn secure_install_application_bundle(
        &mut self,
        url: &Path,
        options: &HashMap<CFType, CFType>,
    ) {
    }

    pub fn mount_image(&mut self, image: &str, options: &HashMap<CFType, CFType>) {}

    pub fn lookup_application(&mut self, options: &HashMap<CFType, CFType>) {
        //
    }

    pub fn interface_type(&mut self) -> AMDeviceInterfaceType {
        AMDeviceInterfaceType::from(unsafe { ffi::AMDeviceGetInterfaceType(self.0) })
    }

    pub fn debug(&mut self) -> String {
        format!(
            "AMDevice {{ udid: {}, interface_type: {}, connection_id: {}, is_paired: {} }}",
            &self.device_identifier(),
            &self.interface_type().to_string(),
            &self.connection_id(),
            &self.is_paired(),
            // &self.serialize()
        )
    }
}

impl Drop for AMDevice {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_ptr() as *const _) }
    }
}

pub struct AFCConnection(ffi::AFCConnectionRef);

impl AFCConnection {
    pub fn as_ptr(&self) -> *const ffi::afc_connection {
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> ffi::AFCConnectionRef {
        self.0
    }

    // fn device_info_open()

    pub fn open_directory<'c, P: AsRef<Path>>(
        &'c mut self,
        path: P,
    ) -> errors::AMDeviceResult<AFCDirectory<'c>> {
        // let mut dir = AFCDirectory::new(self);
        let mut dir = ptr::null_mut();
        errors::to_result(unsafe {
            ffi::AFCDirectoryOpen(self.0, as_c_char(path), &mut dir as *mut _)
        })?;
        Ok(AFCDirectory::new(dir, self))
    }

    pub fn open<'c, P: AsRef<Path>>(
        &mut self,
        path: P,
        mode: u64,
    ) -> errors::AMDeviceResult<AFCFile> {
        let mut ref_ = 0 as ffi::afc_file_ref;
        errors::to_result(unsafe {
            ffi::AFCFileRefOpen(self.0, as_c_char(path), mode, &mut ref_ as *mut _)
        })?;
        Ok(AFCFile::new(self, ref_))
    }

    pub fn context(&mut self) -> u32 {
        unsafe { ffi::AFCConnectionGetContext(self.0) }
    }

    pub fn fs_block_size(&mut self) -> u32 {
        unsafe { ffi::AFCConnectionGetFSBlockSize(self.0) }
    }

    pub fn io_timeout(&mut self) -> u32 {
        unsafe { ffi::AFCConnectionGetIOTimeout(self.0) }
    }

    pub fn socket_block_size(&mut self) -> u32 {
        unsafe { ffi::AFCConnectionGetSocketBlockSize(self.0) }
    }

    fn close(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AFCConnectionClose(self.0) })
    }
}

impl Drop for AFCConnection {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

pub struct AFCDirectory<'c> {
    ref_: *mut ffi::afc_directory,
    conn: &'c mut AFCConnection,
}

impl<'c> AFCDirectory<'c> {
    pub fn as_ptr(&self) -> *const ffi::afc_directory {
        self.ref_
    }

    pub fn as_mut_ptr(&mut self) -> *mut ffi::afc_directory {
        self.ref_
    }

    fn new(ref_: *mut ffi::afc_directory, conn: &'c mut AFCConnection) -> Self {
        Self { conn, ref_ }
    }

    pub fn create<P: AsRef<Path>>(&mut self, dirname: P) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe {
            ffi::AFCDirectoryCreate(self.conn.as_mut_ptr(), as_c_char(dirname))
        })
    }

    pub fn read(&mut self) -> errors::AMDeviceResult<PathBuf> {
        Ok(PathBuf::new())
    }

    fn close(&mut self) -> errors::AMDeviceResult<()> {
        let ref_ = self.ref_;
        let r = errors::to_result(unsafe { ffi::AFCDirectoryClose(self.conn.as_mut_ptr(), ref_) });
        self.ref_ = ptr::null_mut();
        r
    }
}

impl<'c> Drop for AFCDirectory<'c> {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

pub struct AFCFile<'c> {
    ref_: ffi::afc_file_ref,
    conn: &'c mut AFCConnection,
}

impl<'c> AFCFile<'c> {
    pub fn as_ptr(&self) -> *const ffi::afc_file_ref {
        &self.ref_ as *const _
    }
    pub fn as_mut_ptr(&mut self) -> *mut ffi::afc_file_ref {
        &mut self.ref_ as *mut _
    }
    fn new(conn: &'c mut AFCConnection, ref_: ffi::afc_file_ref) -> Self {
        Self { conn, ref_ }
    }

    pub fn seek(&mut self, offset1: u64, offset2: u64) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe {
            ffi::AFCFileRefSeek(self.conn.as_mut_ptr(), self.ref_, offset1, offset2)
        })
    }

    pub fn set_file_size(&mut self, offset: u64) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe {
            ffi::AFCFileRefSetFileSize(self.conn.as_mut_ptr(), self.ref_, offset)
        })
    }

    fn close(&mut self) -> errors::AMDeviceResult<()> {
        errors::to_result(unsafe { ffi::AFCFileRefClose(self.conn.as_mut_ptr(), self.ref_) })
    }
}

impl<'c> Drop for AFCFile<'c> {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

impl<'c> Read for AFCFile<'c> {
    fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        let mut len = 0usize;
        errors::to_result(unsafe {
            ffi::AFCFileRefRead(
                self.conn.as_mut_ptr(),
                self.ref_,
                buf.as_mut_ptr() as *mut _,
                &mut len as *mut _,
            )
        })
        .and_then(|_| Ok(len))
        .map_err(|err| IOError::new(IOErrorKind::Other, err))
    }
}

impl<'c> Write for AFCFile<'c> {
    fn write(&mut self, buf: &[u8]) -> IOResult<usize> {
        let len = buf.len();
        errors::to_result(unsafe {
            ffi::AFCFileRefWrite(
                self.conn.as_mut_ptr(),
                self.ref_,
                buf.as_ptr() as *const _,
                len,
            )
        })
        .and_then(|_| Ok(len))
        .map_err(|err| IOError::new(IOErrorKind::Other, err))
    }

    fn flush(&mut self) -> IOResult<()> {
        Ok(())
    }
}

/*
pub struct AMSocket;
pub fn AMDPostNotification(
        socket: service_conn_t,
        notification: CFStringRef,
        userinfo: CFStringRef,
    ) -> mach_error_t;

    pub fn AMDObserveNotification(
        socket: *mut ::std::ffi::c_void,
        notification: CFStringRef,
    ) -> mach_error_t;

    pub fn AMDListenForNotifications(
        socket: *mut ::std::ffi::c_void,
        cb: notify_callback,
        data: *mut ::std::ffi::c_void,
    ) -> mach_error_t;

    pub fn AMDShutdownNotificationProxy(socket: *mut ::std::ffi::c_void) -> mach_error_t;
*/

unsafe extern "C" fn device_notification_callback(
    info: *mut ffi::am_device_notification_callback_info,
    _arg: *mut ::std::ffi::c_void,
) {
    println!("code: {}", (*info).msg);
    let mut device = AMDevice((*info).dev);
    println!(
        "interface type: {}, udid: {}",
        device.interface_type().to_string(),
        device.device_identifier(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::runloop::CFRunLoop;
    #[test]
    fn test_connection_notifications() {
        AMDevice::notification_subscribe(NotificationSubscriber);
        CFRunLoop::run_current();
    }

    #[test]
    fn test_list_all_devices() {
        let mut devs = AMDevice::devices();
        println!("found devices:");
        for dev in &mut devs {
            println!("{}", dev.debug());
        }
    }
}
