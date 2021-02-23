#[derive(Debug, Clone)]
pub struct DeviceDesc {
    model: &'static str,
    name: &'static str,
    sdk: &'static str,
    arch: &'static str,
}

impl DeviceDesc {
    pub const UNKNOWN: DeviceDesc =
        DeviceDesc::new("UNKN", "Unknown Device", "uknownos", "unkarch");

    pub const fn new(
        model: &'static str,
        name: &'static str,
        sdk: &'static str,
        arch: &'static str,
    ) -> Self {
        Self {
            model,
            name,
            sdk,
            arch,
        }
    }

    pub fn get(model: &str) -> &'static Self {
        for dev in DEVICES.iter() {
            if dev.model.to_uppercase() == model.to_uppercase() {
                return dev;
            }
        }
        &Self::UNKNOWN
    }

    pub fn model(&self) -> &str {
        self.model
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn arch(&self) -> &str {
        self.arch
    }

    pub fn sdk(&self) -> &str {
        self.sdk
    }
}

impl Default for DeviceDesc {
    fn default() -> Self {
        Self::UNKNOWN.clone()
    }
}

pub static DEVICES: &[DeviceDesc] = &[
    // iPod Touch
    DeviceDesc::new("N45AP", "iPod Touch", "iphoneos", "armv7"),
    DeviceDesc::new("N72AP", "iPod Touch 2G", "iphoneos", "armv7"),
    DeviceDesc::new("N18AP", "iPod Touch 3G", "iphoneos", "armv7"),
    DeviceDesc::new("N81AP", "iPod Touch 4G", "iphoneos", "armv7"),
    DeviceDesc::new("N78AP", "iPod Touch 5G", "iphoneos", "armv7"),
    DeviceDesc::new("N78AAP", "iPod Touch 5G", "iphoneos", "armv7"),
    DeviceDesc::new("N102AP", "iPod Touch 6G", "iphoneos", "arm64"),
    DeviceDesc::new("N112AP", "iPod Touch 7G", "iphoneos", "arm64"),
    // iPad
    DeviceDesc::new("K48AP", "iPad", "iphoneos", "armv7"),
    DeviceDesc::new("K93AP", "iPad 2", "iphoneos", "armv7"),
    DeviceDesc::new("K94AP", "iPad 2 (GSM)", "iphoneos", "armv7"),
    DeviceDesc::new("K95AP", "iPad 2 (CDMA)", "iphoneos", "armv7"),
    DeviceDesc::new("K93AAP", "iPad 2 (Wi-Fi, revision A)", "iphoneos", "armv7"),
    DeviceDesc::new("J1AP", "iPad 3", "iphoneos", "armv7"),
    DeviceDesc::new("J2AP", "iPad 3 (GSM)", "iphoneos", "armv7"),
    DeviceDesc::new("J2AAP", "iPad 3 (CDMA)", "iphoneos", "armv7"),
    DeviceDesc::new("P101AP", "iPad 4", "iphoneos", "armv7s"),
    DeviceDesc::new("P102AP", "iPad 4 (GSM)", "iphoneos", "armv7s"),
    DeviceDesc::new("P103AP", "iPad 4 (CDMA)", "iphoneos", "armv7s"),
    DeviceDesc::new("J71bAP", "iPad 6", "iphoneos", "arm64"),
    DeviceDesc::new("J71AP", "iPad Air", "iphoneos", "arm64"),
    DeviceDesc::new("J72AP", "iPad Air (GSM)", "iphoneos", "arm64"),
    DeviceDesc::new("J73AP", "iPad Air (CDMA)", "iphoneos", "arm64"),
    DeviceDesc::new("J81AP", "iPad Air 2", "iphoneos", "arm64"),
    DeviceDesc::new("J82AP", "iPad Air 2 (GSM)", "iphoneos", "arm64"),
    DeviceDesc::new("J83AP", "iPad Air 2 (CDMA)", "iphoneos", "arm64"),
    DeviceDesc::new("J71sAP", "iPad (2017)", "iphoneos", "arm64"),
    DeviceDesc::new("J71tAP", "iPad (2017)", "iphoneos", "arm64"),
    DeviceDesc::new("J72sAP", "iPad (2017)", "iphoneos", "arm64"),
    DeviceDesc::new("J72tAP", "iPad (2017)", "iphoneos", "arm64"),
    DeviceDesc::new("J71bAP", "iPad (2018)", "iphoneos", "arm64"),
    DeviceDesc::new("J72bAP", "iPad (2018)", "iphoneos", "arm64"),
    DeviceDesc::new("J217AP", "iPad Air 3", "iphoneos", "arm64e"),
    DeviceDesc::new("J218AP", "iPad Air 3 (Cellular)", "iphoneos", "arm64e"),
    DeviceDesc::new("J171AP", "iPad 7", "iphoneos", "arm64"),
    DeviceDesc::new("J172AP", "iPad 7 (Cellular)", "iphoneos", "arm64"),
    DeviceDesc::new("J171aAP", "iPad 8", "iphoneos", "arm64e"),
    DeviceDesc::new("J172aAP", "iPad 8 (Cellular)", "iphoneos", "arm64e"),
    DeviceDesc::new("J307AP", "iPad Air 4", "iphoneos", "arm64e"),
    DeviceDesc::new("J308AP", "iPad Air 4 (Cellular)", "iphoneos", "arm64e"),
    // iPad Pro
    DeviceDesc::new("J98aAP", "iPad Pro (12.9\")", "iphoneos", "arm64"),
    DeviceDesc::new("J99aAP", "iPad Pro (12.9\")", "iphoneos", "arm64"),
    DeviceDesc::new("J120AP", "iPad Pro 2G (12.9\")", "iphoneos", "arm64"),
    DeviceDesc::new("J121AP", "iPad Pro 2G (12.9\")", "iphoneos", "arm64"),
    DeviceDesc::new("J127AP", "iPad Pro (9.7\")", "iphoneos", "arm64"),
    DeviceDesc::new("J128AP", "iPad Pro (9.7\")", "iphoneos", "arm64"),
    DeviceDesc::new("J207AP", "iPad Pro (10.5\")", "iphoneos", "arm64"),
    DeviceDesc::new("J208AP", "iPad Pro (10.5\")", "iphoneos", "arm64"),
    DeviceDesc::new("J317AP", "iPad Pro (11\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J317xAP", "iPad Pro (11\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J318AP", "iPad Pro (11\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J318xAP", "iPad Pro (11\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J417AP", "iPad Pro 2g (11\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J418AP", "iPad Pro 2g (11\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J320AP", "iPad Pro 3G (12.9\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J320xAP", "iPad Pro 3G (12.9\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J321AP", "iPad Pro 3G (12.9\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J321xAP", "iPad Pro 3G (12.9\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J420AP", "iPad Pro 4G (12.9\")", "iphoneos", "arm64e"),
    DeviceDesc::new("J421AP", "iPad Pro 4G (12.9\")", "iphoneos", "arm64e"),
    // iPad Mini
    DeviceDesc::new("P105AP", "iPad mini", "iphoneos", "armv7"),
    DeviceDesc::new("P106AP", "iPad mini (GSM)", "iphoneos", "armv7"),
    DeviceDesc::new("P107AP", "iPad mini (CDMA)", "iphoneos", "armv7"),
    DeviceDesc::new("J85AP", "iPad mini 2", "iphoneos", "arm64"),
    DeviceDesc::new("J86AP", "iPad mini 2 (GSM)", "iphoneos", "arm64"),
    DeviceDesc::new("J87AP", "iPad mini 2 (CDMA)", "iphoneos", "arm64"),
    DeviceDesc::new("J85MAP", "iPad mini 3", "iphoneos", "arm64"),
    DeviceDesc::new("J86MAP", "iPad mini 3 (GSM)", "iphoneos", "arm64"),
    DeviceDesc::new("J87MAP", "iPad mini 3 (CDMA)", "iphoneos", "arm64"),
    DeviceDesc::new("J96AP", "iPad mini 4", "iphoneos", "arm64"),
    DeviceDesc::new("J97AP", "iPad mini 4 (GSM)", "iphoneos", "arm64"),
    DeviceDesc::new("J210AP", "iPad mini 5", "iphoneos", "arm64e"),
    DeviceDesc::new("J211AP", "iPad mini 5 (Cellular)", "iphoneos", "arm64e"),
    // iPhone
    DeviceDesc::new("M68AP", "iPhone", "iphoneos", "armv7"),
    DeviceDesc::new("N82AP", "iPhone 3G", "iphoneos", "armv7"),
    DeviceDesc::new("N88AP", "iPhone 3GS", "iphoneos", "armv7"),
    DeviceDesc::new("N90AP", "iPhone 4 (GSM)", "iphoneos", "armv7"),
    DeviceDesc::new("N92AP", "iPhone 4 (CDMA)", "iphoneos", "armv7"),
    DeviceDesc::new("N90BAP", "iPhone 4 (GSM, revision A)", "iphoneos", "armv7"),
    DeviceDesc::new("N94AP", "iPhone 4S", "iphoneos", "armv7"),
    DeviceDesc::new("N41AP", "iPhone 5 (GSM)", "iphoneos", "armv7s"),
    DeviceDesc::new("N42AP", "iPhone 5 (Global/CDMA)", "iphoneos", "armv7s"),
    DeviceDesc::new("N48AP", "iPhone 5c (GSM)", "iphoneos", "armv7s"),
    DeviceDesc::new("N49AP", "iPhone 5c (Global/CDMA)", "iphoneos", "armv7s"),
    DeviceDesc::new("N51AP", "iPhone 5s (GSM)", "iphoneos", "arm64"),
    DeviceDesc::new("N53AP", "iPhone 5s (Global/CDMA)", "iphoneos", "arm64"),
    DeviceDesc::new("N61AP", "iPhone 6 (GSM)", "iphoneos", "arm64"),
    DeviceDesc::new("N56AP", "iPhone 6 Plus", "iphoneos", "arm64"),
    DeviceDesc::new("N71mAP", "iPhone 6s", "iphoneos", "arm64"),
    DeviceDesc::new("N71AP", "iPhone 6s", "iphoneos", "arm64"),
    DeviceDesc::new("N66AP", "iPhone 6s Plus", "iphoneos", "arm64"),
    DeviceDesc::new("N66mAP", "iPhone 6s Plus", "iphoneos", "arm64"),
    DeviceDesc::new("N69AP", "iPhone SE", "iphoneos", "arm64"),
    DeviceDesc::new("N69uAP", "iPhone SE", "iphoneos", "arm64"),
    DeviceDesc::new("D10AP", "iPhone 7", "iphoneos", "arm64"),
    DeviceDesc::new("D101AP", "iPhone 7", "iphoneos", "arm64"),
    DeviceDesc::new("D11AP", "iPhone 7 Plus", "iphoneos", "arm64"),
    DeviceDesc::new("D111AP", "iPhone 7 Plus", "iphoneos", "arm64"),
    DeviceDesc::new("D20AP", "iPhone 8", "iphoneos", "arm64"),
    DeviceDesc::new("D20AAP", "iPhone 8", "iphoneos", "arm64"),
    DeviceDesc::new("D201AP", "iPhone 8", "iphoneos", "arm64"),
    DeviceDesc::new("D201AAP", "iPhone 8", "iphoneos", "arm64"),
    DeviceDesc::new("D21AP", "iPhone 8 Plus", "iphoneos", "arm64"),
    DeviceDesc::new("D21AAP", "iPhone 8 Plus", "iphoneos", "arm64"),
    DeviceDesc::new("D211AP", "iPhone 8 Plus", "iphoneos", "arm64"),
    DeviceDesc::new("D211AAP", "iPhone 8 Plus", "iphoneos", "arm64"),
    DeviceDesc::new("D22AP", "iPhone X", "iphoneos", "arm64"),
    DeviceDesc::new("D221AP", "iPhone X", "iphoneos", "arm64"),
    DeviceDesc::new("N841AP", "iPhone XR", "iphoneos", "arm64e"),
    DeviceDesc::new("D321AP", "iPhone XS", "iphoneos", "arm64e"),
    DeviceDesc::new("D331pAP", "iPhone XS Max", "iphoneos", "arm64e"),
    DeviceDesc::new("N104AP", "iPhone 11", "iphoneos", "arm64e"),
    DeviceDesc::new("D421AP", "iPhone 11 Pro", "iphoneos", "arm64e"),
    DeviceDesc::new("D431AP", "iPhone 11 Pro Max", "iphoneos", "arm64e"),
    DeviceDesc::new("D79AP", "iPhone SE 2G", "iphoneos", "arm64e"),
    DeviceDesc::new("D52gAP", "iPhone 12 Mini", "iphoneos", "arm64e"),
    DeviceDesc::new("D53gAP", "iPhone 12", "iphoneos", "arm64e"),
    DeviceDesc::new("D53pAP", "iPhone 12 Pro", "iphoneos", "arm64e"),
    DeviceDesc::new("D54pAP", "iPhone 12 Pro Max", "iphoneos", "arm64e"),
    // Apple TV
    DeviceDesc::new("K66AP", "Apple TV 2G", "appletvos", "armv7"),
    DeviceDesc::new("J33AP", "Apple TV 3G", "appletvos", "armv7"),
    DeviceDesc::new("J33IAP", "Apple TV 3.1G", "appletvos", "armv7"),
    DeviceDesc::new("J42dAP", "Apple TV 4G", "appletvos", "arm64"),
    DeviceDesc::new("J105aAP", "Apple TV 4K", "appletvos", "arm64"),
];
