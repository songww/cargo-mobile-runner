use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
#[repr(u32)]
pub enum AMDeviceError {
    #[error("An unknown error occurred.")]
    AMDUndefinedError = 0xe8000001,
    #[error("Could not transfer file.")]
    AMDBadHeaderError = 0xe8000002,
    #[error("Could not allocate a resource.")]
    AMDNoResourcesError = 0xe8000003,
    #[error("Could not read from the device.")]
    AMDReadError = 0xe8000004,
    #[error("Could not write to the device.")]
    AMDWriteError = 0xe8000005,
    #[error("The packet is unknown.")]
    AMDUnknownPacketError = 0xe8000006,
    #[error("The argument is invalid.")]
    AMDInvalidArgumentError = 0xe8000007,
    #[error("The file could not be found.")]
    AMDNotFoundError = 0xe8000008,
    #[error("The path is a directory.")]
    AMDIsDirectoryError = 0xe8000009,
    #[error("You do not have permission.")]
    AMDPermissionError = 0xe800000a,
    #[error("Not connected to the device.")]
    AMDNotConnectedError = 0xe800000b,
    #[error("The operation timed out.")]
    AMDTimeOutError = 0xe800000c,
    #[error("There was a buffer overrun.")]
    AMDOverrunError = 0xe800000d,
    #[error("End of file.")]
    AMDEOFError = 0xe800000e,
    #[error("This operation is unsupported.")]
    AMDUnsupportedError = 0xe800000f,
    #[error("The file already exists.")]
    AMDFileExistsError = 0xe8000010,
    #[error("The device is busy.")]
    AMDBusyError = 0xe8000011,
    #[error("Could not establish a secure connection to the device.")]
    AMDCryptoError = 0xe8000012,
    #[error("Received an unexpected response from the device.")]
    AMDInvalidResponseError = 0xe8000013,
    #[error("The key is missing.")]
    AMDMissingKeyError = 0xe8000014,
    #[error("The value is missing.")]
    AMDMissingValueError = 0xe8000015,
    #[error("Cannot retrieve value from the passcode locked device.")]
    AMDGetProhibitedError = 0xe8000016,
    #[error("Cannot set value on device.")]
    AMDSetProhibitedError = 0xe8000017,
    #[error("Cannot remove value on device.")]
    AMDRemoveProhibitedError = 0xe8000018,
    #[error("This value cannot be changed.")]
    AMDImmutableValueError = 0xe8000019,
    #[error("The device is passcode protected.")]
    AMDPasswordProtectedError = 0xe800001a,
    #[error("The device does not recognize this host.")]
    AMDMissingHostIDError = 0xe800001b,
    #[error("The device does not recognize this host.")]
    AMDInvalidHostIDError = 0xe800001c,
    #[error("The session is active.")]
    AMDSessionActiveError = 0xe800001d,
    #[error("The session is inactive.")]
    AMDSessionInactiveError = 0xe800001e,
    #[error("The session ID is missing.")]
    AMDMissingSessionIDError = 0xe800001f,
    #[error("The session ID is invalid.")]
    AMDInvalidSessionIDError = 0xe8000020,
    #[error("The service is missing.")]
    AMDMissingServiceError = 0xe8000021,
    #[error("The service is invalid.")]
    AMDInvalidServiceError = 0xe8000022,
    #[error("Could not start service on device")]
    AMDInvalidCheckinError = 0xe8000023,
    #[error("The service did not start properly on the device.")]
    AMDCheckinTimeoutError = 0xe8000024,
    #[error("The host is not paired with the device.")]
    AMDMissingPairRecordError = 0xe8000025,
    #[error("The activation record is not valid.")]
    AMDInvalidActivationRecordError = 0xe8000026,
    #[error("The activation record could not be found.")]
    AMDMissingActivationRecordError = 0xe8000027,
    #[error("The device is in recovery mode.")]
    AMDWrongDroidError = 0xe8000028,
    #[error("The software update package could not be verified.")]
    AMDSUVerificationError = 0xe8000029,
    #[error("Could not patch the file.")]
    AMDSUPatchError = 0xe800002a,
    #[error("Could not flash the firmware.")]
    AMDSUFirmwareError = 0xe800002b,
    #[error("The provisioning profile is not valid.")]
    AMDProvisioningProfileNotValid = 0xe800002c,
    #[error("Could not send a message to the device.")]
    AMDSendMessageError = 0xe800002d,
    #[error("Could not receive a message from the device.")]
    AMDReceiveMessageError = 0xe800002e,
    #[error("The options are missing.")]
    AMDMissingOptionsError = 0xe800002f,
    #[error("The image is missing.")]
    AMDMissingImageTypeError = 0xe8000030,
    #[error("Could not read disk image.")]
    AMDDigestFailedError = 0xe8000031,
    #[error("The service could not be started.")]
    AMDStartServiceError = 0xe8000032,
    #[error("The disk image is invalid.")]
    AMDInvalidDiskImageError = 0xe8000033,
    #[error("The digest is missing.")]
    AMDMissingDigestError = 0xe8000034,
    #[error("There was an error with the USB device multiplexor.")]
    AMDMuxError = 0xe8000035,
    #[error("A system application with the given bundle identifier is already installed on the device and cannot be replaced.")]
    AMDApplicationAlreadyInstalledError = 0xe8000036,
    #[error("The application could not be moved into place on the device.")]
    AMDApplicationMoveFailedError = 0xe8000037,
    #[error("kAMDApplicationSINFCaptureFailedError")]
    Code0xe8000038 = 0xe8000038,
    #[error("The application could not be sandboxed.")]
    AMDApplicationSandboxFailedError = 0xe8000039,
    #[error("The application could not be verified.")]
    AMDApplicationVerificationFailedError = 0xe800003a,
    #[error("Could not remove the application archive.")]
    AMDArchiveDestructionFailedError = 0xe800003b,
    #[error("The carrier bundle could not be verified.")]
    AMDBundleVerificationFailedError = 0xe800003c,
    #[error("Could not install the carrier bundle.")]
    AMDCarrierBundleCopyFailedError = 0xe800003d,
    #[error("Could not create the carrier bundle directory.")]
    AMDCarrierBundleDirectoryCreationFailedError = 0xe800003e,
    #[error("There are no supported SIMs for this carrier bundle.")]
    AMDCarrierBundleMissingSupportedSIMsError = 0xe800003f,
    #[error("Could not listen for notification from the baseband.")]
    AMDCommCenterNotificationFailedError = 0xe8000040,
    #[error("Could not create application container.")]
    AMDContainerCreationFailedError = 0xe8000041,
    #[error("Could not repair permissions on application container.")]
    AMDContainerP0wnFailedError = 0xe8000042,
    #[error("Could not remove the application container.")]
    AMDContainerRemovalFailedError = 0xe8000043,
    #[error("Could not install the embedded provisioning profile.")]
    AMDEmbeddedProfileInstallFailedError = 0xe8000044,
    #[error("An error occurred.")]
    AMDErrorError = 0xe8000045,
    #[error("Could not change executable permissions on the application.")]
    AMDExecutableTwiddleFailedError = 0xe8000046,
    #[error("Could not check to see if the application already exists.")]
    AMDExistenceCheckFailedError = 0xe8000047,
    #[error("Could not update the installed applications list.")]
    AMDInstallMapUpdateFailedError = 0xe8000048,
    #[error("Could not save the application manifest.")]
    AMDManifestCaptureFailedError = 0xe8000049,
    #[error("Could not generate the map.")]
    AMDMapGenerationFailedError = 0xe800004a,
    #[error("The application bundle does not contain an executable.")]
    AMDMissingBundleExecutableError = 0xe800004b,
    #[error("The application bundle does not contain a valid identifier.")]
    AMDMissingBundleIdentifierError = 0xe800004c,
    #[error("Could not determine the application bundle path.")]
    AMDMissingBundlePathError = 0xe800004d,
    #[error("Could not find the container for the installed application.")]
    AMDMissingContainerError = 0xe800004e,
    #[error("Could not post a notification.")]
    AMDNotificationFailedError = 0xe800004f,
    #[error("Could not open the application package.")]
    AMDPackageExtractionFailedError = 0xe8000050,
    #[error("Could not inspect the application package.")]
    AMDPackageInspectionFailedError = 0xe8000051,
    #[error("Could not move the application package into the staging location.")]
    AMDPackageMoveFailedError = 0xe8000052,
    #[error("Could not convert the path.")]
    AMDPathConversionFailedError = 0xe8000053,
    #[error("Could not restore the application container.")]
    AMDRestoreContainerFailedError = 0xe8000054,
    #[error("Could not remove the application seatbelt profile.")]
    AMDSeatbeltProfileRemovalFailedError = 0xe8000055,
    #[error("Could not create the staging directory.")]
    AMDStageCreationFailedError = 0xe8000056,
    #[error("Could not create the symlink.")]
    AMDSymlinkFailedError = 0xe8000057,
    #[error("Could not save the iTunes artwork.")]
    AMDiTunesArtworkCaptureFailedError = 0xe8000058,
    #[error("Could not save the iTunes metadata.")]
    AMDiTunesMetadataCaptureFailedError = 0xe8000059,
    #[error("The application is already archived.")]
    AMDAlreadyArchivedError = 0xe800005a,
    #[error("Too many instances of this service are already running.")]
    AMDServiceLimitError = 0xe800005b,
    #[error("The host is no longer paired with the device.")]
    AMDInvalidPairRecordError = 0xe800005c,
    #[error("The service could not be started on the device.")]
    AMDServiceProhibitedError = 0xe800005d,
    #[error("Could not start service on device")]
    AMDCheckinSetupFailedError = 0xe800005e,
    #[error("The service did not start properly on the device.")]
    AMDCheckinConnectionFailedError = 0xe800005f,
    #[error("The service did not start properly on the device.")]
    AMDCheckinReceiveFailedError = 0xe8000060,
    #[error("The service did not start properly on the device.")]
    AMDCheckinResponseFailedError = 0xe8000061,
    #[error("The service did not start properly on the device.")]
    AMDCheckinSendFailedError = 0xe8000062,
    #[error("Could not listen for USB devices.")]
    AMDMuxCreateListenerError = 0xe8000063,
    #[error("Could not get the USB listener.")]
    AMDMuxGetListenerError = 0xe8000064,
    #[error("Could not connect to the device.")]
    AMDMuxConnectError = 0xe8000065,
    #[error("The device does not recognize the command.")]
    AMDUnknownCommandError = 0xe8000066,
    #[error("There was an internal API error.")]
    AMDAPIInternalError = 0xe8000067,
    #[error("Could not save the pairing record.")]
    AMDSavePairRecordFailedError = 0xe8000068,
    #[error("The service did not start properly on the device.")]
    AMDCheckinOutOfMemoryError = 0xe8000069,
    #[error("This application needs to be updated.")]
    AMDDeviceTooNewError = 0xe800006a,
    #[error("This device is no longer connected.")]
    AMDDeviceRefNoGood = 0xe800006b,
    #[error("Could not translate messages from device")]
    AMDCannotTranslateError = 0xe800006c,
    #[error("Could not support development.")]
    AMDMobileImageMounterMissingImageSignature = 0xe800006d,
    #[error("Could not support development.")]
    AMDMobileImageMounterResponseCreationFailed = 0xe800006e,
    #[error("Could not support development.")]
    AMDMobileImageMounterMissingImageType = 0xe800006f,
    #[error("Could not support development.")]
    AMDMobileImageMounterMissingImagePath = 0xe8000070,
    #[error("Could not support development.")]
    AMDMobileImageMounterImageMapLoadFailed = 0xe8000071,
    #[error("Image is already mounted.")]
    AMDMobileImageMounterAlreadyMounted = 0xe8000072,
    #[error("Could not support development.")]
    AMDMobileImageMounterImageMoveFailed = 0xe8000073,
    #[error("Could not support development.")]
    AMDMobileImageMounterMountPathMissing = 0xe8000074,
    #[error("Could not support development.")]
    AMDMobileImageMounterMountPathNotEmpty = 0xe8000075,
    #[error("Could not support development.")]
    AMDMobileImageMounterImageMountFailed = 0xe8000076,
    #[error("Could not support development.")]
    AMDMobileImageMounterTrustCacheLoadFailed = 0xe8000077,
    #[error("Could not support development.")]
    AMDMobileImageMounterDigestFailed = 0xe8000078,
    #[error("Could not support development.")]
    AMDMobileImageMounterDigestCreationFailed = 0xe8000079,
    #[error("Could not support development.")]
    AMDMobileImageMounterImageVerificationFailed = 0xe800007a,
    #[error("Could not support development.")]
    AMDMobileImageMounterImageInfoCreationFailed = 0xe800007b,
    #[error("Could not support development.")]
    AMDMobileImageMounterImageMapStoreFailed = 0xe800007c,
    #[error("kAMDBonjourSetupError")]
    Code0xe800007d = 0xe800007d,
    #[error("The device OS version is too low.")]
    AMDDeviceOSVersionTooLow = 0xe800007e,
    #[error("Device doesn't support wireless sync.")]
    AMDNoWifiSyncSupportError = 0xe800007f,
    #[error("This application does not support this kind of device.")]
    AMDDeviceFamilyNotSupported = 0xe8000080,
    #[error("Device is not available until first unlock after boot.")]
    AMDEscrowLockedError = 0xe8000081,
    #[error("Pairing only allowed over USB.")]
    AMDPairingProhibitedError = 0xe8000082,
    #[error("Operation prohibited on supervised devices.")]
    AMDProhibitedBySupervision = 0xe8000083,
    #[error("This device is no longer connected.")]
    AMDDeviceDisconnectedError = 0xe8000084,
    #[error("The message is too big.")]
    AMDTooBigError = 0xe8000085,
    #[error("Could not apply patch update to application.")]
    AMDPackagePatchFailedError = 0xe8000086,
    #[error("This application does not support this device's CPU type.")]
    AMDIncorrectArchitectureError = 0xe8000087,
    #[error("Could not copy VPN Plugin into app container.")]
    AMDPluginCopyFailedError = 0xe8000088,
    #[error("Could not write installation breadcrumb.")]
    AMDBreadcrumbFailedError = 0xe8000089,
    #[error("Could not update installation breadcrumb.")]
    AMDBreadcrumbUnlockError = 0xe800008a,
    #[error("Could not save the GeoJSON data.")]
    AMDGeoJSONCaptureFailedError = 0xe800008b,
    #[error("Could not save the Newsstand artwork.")]
    AMDNewsstandArtworkCaptureFailedError = 0xe800008c,
    #[error("The request did not contain a command.")]
    AMDMissingCommandError = 0xe800008d,
    #[error("The requesting application is not allowed to make this request.")]
    AMDNotEntitledError = 0xe800008e,
    #[error("Request was missing the package path.")]
    AMDMissingPackagePathError = 0xe800008f,
    #[error("Request was missing the container path.")]
    AMDMissingContainerPathError = 0xe8000090,
    #[error("Request was missing the application identifier.")]
    AMDMissingApplicationIdentifierError = 0xe8000091,
    #[error("Request was missing a required value.")]
    AMDMissingAttributeValueError = 0xe8000092,
    #[error("Could not list installed applications.")]
    AMDLookupFailedError = 0xe8000093,
    #[error("Could not extract capabilities from the request.")]
    AMDDictCreationFailedError = 0xe8000094,
    #[error("The device rejected the pairing attempt.")]
    AMDUserDeniedPairingError = 0xe8000095,
    #[error("The user has not yet responded to the pairing request.")]
    AMDPairingDialogResponsePendingError = 0xe8000096,
    #[error("Installation of apps is prohibited by a policy on the device.")]
    AMDInstallProhibitedError = 0xe8000097,
    #[error("Uninstallation of apps is prohibited by a policy on the device.")]
    AMDUninstallProhibitedError = 0xe8000098,
    #[error("The device is in lost mode.")]
    AMDFMiPProtectedError = 0xe8000099,
    #[error("Pairing is prohibited by a policy on the device.")]
    AMDMCProtected = 0xe800009a,
    #[error("A policy on the device requires secure pairing.")]
    AMDMCChallengeRequired = 0xe800009b,
    #[error("The bundle's Info.plist does not contain a CFBundleVersion key or its value is not a string.")]
    AMDMissingBundleVersionError = 0xe800009c,
    #[error("This app is not allowed to be installed on this device.")]
    AMDAppBlacklistedError = 0xe800009d,
    #[error("This app contains an app extension with an illegal bundle identifier. App extension bundle identifiers must have a prefix consisting of their containing application's bundle identifier followed by a '.'.")]
    Code0xe800009e = 0xe800009e,
    #[error("If an app extension defines the XPCService key in its Info.plist, it must have a dictionary value.")]
    Code0xe800009f = 0xe800009f,
    #[error("App extensions must define the NSExtension key with a dictionary value in their Info.plist.")]
    Code0xe80000a0 = 0xe80000a0,
    #[error("If an app extension defines the CFBundlePackageType key in its Info.plist, it must have the value \"XPC!\".")]
    Code0xe80000a1 = 0xe80000a1,
    #[error("App extensions must define either NSExtensionMainStoryboard or NSExtensionPrincipalClass keys in the NSExtension dictionary in their Info.plist.")]
    Code0xe80000a2 = 0xe80000a2,
    #[error("If an app extension defines the NSExtensionContextClass key in the NSExtension dictionary in its Info.plist, it must have a string value containing one or more characters.")]
    Code0xe80000a3 = 0xe80000a3,
    #[error("If an app extension defines the NSExtensionContextHostClass key in the NSExtension dictionary in its Info.plist, it must have a string value containing one or more characters.")]
    Code0xe80000a4 = 0xe80000a4,
    #[error("If an app extension defines the NSExtensionViewControllerHostClass key in the NSExtension dictionary in its Info.plist, it must have a string value containing one or more characters.")]
    Code0xe80000a5 = 0xe80000a5,
    #[error("This app contains an app extension that does not define the NSExtensionPointIdentifier key in its Info.plist. This key must have a reverse-DNS format string value.")]
    Code0xe80000a6 = 0xe80000a6,
    #[error("This app contains an app extension that does not define the NSExtensionPointIdentifier key in its Info.plist with a valid reverse-DNS format string value.")]
    Code0xe80000a7 = 0xe80000a7,
    #[error("If an app extension defines the NSExtensionAttributes key in the NSExtension dictionary in its Info.plist, it must have a dictionary value.")]
    Code0xe80000a8 = 0xe80000a8,
    #[error("If an app extension defines the NSExtensionPointName key in the NSExtensionAttributes dictionary in the NSExtension dictionary in its Info.plist, it must have a string value containing one or more characters.")]
    Code0xe80000a9 = 0xe80000a9,
    #[error("If an app extension defines the NSExtensionPointVersion key in the NSExtensionAttributes dictionary in the NSExtension dictionary in its Info.plist, it must have a string value containing one or more characters.")]
    Code0xe80000aa = 0xe80000aa,
    #[error("This app or a bundle it contains does not define the CFBundleName key in its Info.plist with a string value containing one or more characters.")]
    Code0xe80000ab = 0xe80000ab,
    #[error("This app or a bundle it contains does not define the CFBundleDisplayName key in its Info.plist with a string value containing one or more characters.")]
    Code0xe80000ac = 0xe80000ac,
    #[error("This app or a bundle it contains defines the CFBundleShortVersionStringKey key in its Info.plist with a non-string value or a zero-length string value.")]
    Code0xe80000ad = 0xe80000ad,
    #[error("This app or a bundle it contains defines the RunLoopType key in the XPCService dictionary in its Info.plist with a non-string value or a zero-length string value.")]
    Code0xe80000ae = 0xe80000ae,
    #[error("This app or a bundle it contains defines the ServiceType key in the XPCService dictionary in its Info.plist with a non-string value or a zero-length string value.")]
    Code0xe80000af = 0xe80000af,
    #[error("This application or a bundle it contains has the same bundle identifier as this application or another bundle that it contains. Bundle identifiers must be unique.")]
    Code0xe80000b0 = 0xe80000b0,
    #[error("This app contains an app extension that specifies an extension point identifier that is not supported on this version of iOS for the value of the NSExtensionPointIdentifier key in its Info.plist.")]
    Code0xe80000b1 = 0xe80000b1,
    #[error("This app contains multiple app extensions that are file providers. Apps are only allowed to contain at most a single file provider app extension.")]
    Code0xe80000b2 = 0xe80000b2,
    #[error("The request was missing a command.")]
    MobileHouseArrestMissingCommand = 0xe80000b3,
    #[error("The request contained an invalid command.")]
    MobileHouseArrestUnknownCommand = 0xe80000b4,
    #[error("The request was missing an application identifier.")]
    MobileHouseArrestMissingIdentifier = 0xe80000b5,
    #[error("The request contained an invalid request dictionary.")]
    MobileHouseArrestDictionaryFailed = 0xe80000b6,
    #[error("Could not find the requested application.")]
    MobileHouseArrestInstallationLookupFailed = 0xe80000b7,
    #[error("The requested application is not a user application.")]
    MobileHouseArrestApplicationLookupFailed = 0xe80000b8,
    #[error("The requested application does not contain a valid data container.")]
    MobileHouseArrestMissingContainer = 0xe80000b9,
    #[error("Could not convert the requested application's data container path.")]
    MobileHouseArrestPathConversionFailed = 0xe80000bb,
    #[error("The requested application's data container path does not exist.")]
    MobileHouseArrestPathMissing = 0xe80000bc,
    #[error("The requested application contained an invalid data container path.")]
    MobileHouseArrestInvalidPath = 0xe80000bd,
    #[error("This application's application-identifier entitlement does not match that of the installed application. These values must match for an upgrade to be allowed.")]
    AMDMismatchedApplicationIdentifierEntitlementError = 0xe80000be,
    #[error("The bundle contained an invalid symlink.")]
    AMDInvalidSymlinkError = 0xe80000bf,
    #[error("No space is available on the device.")]
    AMDNoSpaceError = 0xe80000c0,
    #[error("The WatchKit app extension must have, in its Info.plist's NSExtension dictionary's NSExtensionAttributes dictionary, the key WKAppBundleIdentifier with a value equal to the associated WatchKit app's bundle identifier.")]
    Code0xe80000c1 = 0xe80000c1,
    #[error("This app is not a valid AppleTV Stub App")]
    Code0xe80000c2 = 0xe80000c2,
    #[error("This application's iTunesMetadata.plist specifies versions that do not match the versions listed for the app in its Info.plist")]
    AMDBundleiTunesMetadataVersionMismatchError = 0xe80000c3,
    #[error("This application's iTunesMetadata.plist is not valid.")]
    AMDInvalidiTunesMetadataPlistError = 0xe80000c4,
    #[error("This application's bundle identifier does not match its code signing identifier.")]
    AMDMismatchedBundleIDSigningIdentifierError = 0xe80000c5,
    #[error("This app contains multiple WatchKit app extensions. Only a single WatchKit extension is allowed.")]
    Code0xe80000c6 = 0xe80000c6,
    #[error("A WatchKit app within this app is not a valid bundle.")]
    Code0xe80000c7 = 0xe80000c7,
    #[error("This application is not built for this device.")]
    AMDDeviceNotSupportedByThinningError = 0xe80000c8,
    #[error("The UISupportedDevices key in this app's Info.plist does not specify a valid set of supported devices.")]
    Code0xe80000c9 = 0xe80000c9,
    #[error("This app contains an app extension with an illegal bundle identifier. App extension bundle identifiers must have a prefix consisting of their containing application's bundle identifier followed by a '.', with no further '.' characters after the prefix.")]
    Code0xe80000ca = 0xe80000ca,
    #[error("This application contains an app extension with a bundle identifier that conflicts with the bundle identifier of another app or app extension already installed.")]
    AMDAppexBundleIDConflictWithOtherIdentifierError = 0xe80000cb,
    #[error("This application's bundle identifier conflicts with the identifier of another app or app extension already installed.")]
    AMDBundleIDConflictWithOtherIdentifierError = 0xe80000cc,
    #[error(
        "This app contains multiple WatchKit 1.0 apps. Only a single WatchKit 1.0 app is allowed."
    )]
    Code0xe80000cd = 0xe80000cd,
    #[error(
        "This app contains multiple WatchKit 2.0 apps. Only a single WatchKit 2.0 app is allowed."
    )]
    Code0xe80000ce = 0xe80000ce,
    #[error("The WatchKit app has an invalid stub executable.")]
    Code0xe80000cf = 0xe80000cf,
    #[error("The WatchKit app has multiple app extensions. Only a single WatchKit extension is allowed in a WatchKit app, and only if this is a WatchKit 2.0 app.")]
    Code0xe80000d0 = 0xe80000d0,
    #[error("The WatchKit 2.0 app contains non-WatchKit app extensions. Only WatchKit app extensions are allowed in WatchKit apps.")]
    Code0xe80000d1 = 0xe80000d1,
    #[error("The WatchKit app has one or more embedded frameworks. Frameworks are only allowed in WatchKit app extensions in WatchKit 2.0 apps.")]
    Code0xe80000d2 = 0xe80000d2,
    #[error("This app contains a WatchKit 1.0 app with app extensions. This is not allowed.")]
    Code0xe80000d3 = 0xe80000d3,
    #[error("This app contains a WatchKit 2.0 app without an app extension. WatchKit 2.0 apps must contain a WatchKit app extension.")]
    Code0xe80000d4 = 0xe80000d4,
    #[error("The WatchKit app's Info.plist must have a WKCompanionAppBundleIdentifier key set to the bundle identifier of the companion app.")]
    Code0xe80000d5 = 0xe80000d5,
    #[error("The WatchKit app's Info.plist contains a non-string key.")]
    Code0xe80000d6 = 0xe80000d6,
    #[error("The WatchKit app's Info.plist contains a key that is not in the whitelist of allowed keys for a WatchKit app.")]
    Code0xe80000d7 = 0xe80000d7,
    #[error("The WatchKit 1.0 and a WatchKit 2.0 apps within this app must have have the same bundle identifier.")]
    Code0xe80000d8 = 0xe80000d8,
    #[error("This app contains a WatchKit app with an invalid bundle identifier. The bundle identifier of a WatchKit app must have a prefix consisting of the companion app's bundle identifier, followed by a '.'.")]
    Code0xe80000d9 = 0xe80000d9,
    #[error("This app contains a WatchKit app where the UIDeviceFamily key in its Info.plist does not specify the value 4 to indicate that it's compatible with the Apple Watch device type.")]
    Code0xe80000da = 0xe80000da,
    #[error("The device is out of storage for apps. Please remove some apps from the device and try again.")]
    Code0xe80000db = 0xe80000db,
    #[error("This app or an app that it contains has a Siri Intents app extension that is missing the IntentsSupported array in the NSExtensionAttributes dictionary in the NSExtension dictionary in its Info.plist.")]
    Code0xe80000dc = 0xe80000dc,
    #[error("This app or an app that it contains has a Siri Intents app extension that does not correctly define the IntentsRestrictedWhileLocked key in the NSExtensionAttributes dictionary in the NSExtension dictionary in its Info.plist. The key's value must be an array of strings.")]
    Code0xe80000dd = 0xe80000dd,
    #[error("This app or an app that it contains has a Siri Intents app extension that declares values in its IntentsRestrictedWhileLocked key's array value that are not in its IntentsSupported key's array value (in the NSExtensionAttributes dictionary in the NSExtension dictionary in its Info.plist).")]
    Code0xe80000de = 0xe80000de,
    #[error("This app or an app that it contains declares multiple Siri Intents app extensions that declare one or more of the same values in the IntentsSupported array in the NSExtensionAttributes dictionary in the NSExtension dictionary in their Info.plist. IntentsSupported must be distinct among a given Siri Intents extension type within an app.")]
    Code0xe80000df = 0xe80000df,
    #[error("The WatchKit 2.0 app, which expects to be compatible with watchOS versions earlier than 3.0, contains a non-WatchKit extension in a location that's not compatible with watchOS versions earlier than 3.0.")]
    Code0xe80000e0 = 0xe80000e0,
    #[error("The WatchKit 2.0 app, which expects to be compatible with watchOS versions earlier than 3.0, contains a framework in a location that's not compatible with watchOS versions earlier than 3.0.")]
    Code0xe80000e1 = 0xe80000e1,
    #[error("The device is locked.")]
    AMDMobileImageMounterDeviceLocked = 0xe80000e2,
    #[error("The encryption information included with this application is not valid so this application cannot be installed on this device.")]
    AMDInvalidSINFError = 0xe80000e3,
    #[error("Multiple iMessage app extensions were found in this app. Only one is allowed.")]
    Code0xe80000e4 = 0xe80000e4,
    #[error("This iMessage application is missing its required iMessage app extension.")]
    Code0xe80000e5 = 0xe80000e5,
    #[error("This iMessage application contains an app extension type other than an iMessage app extension. iMessage applications may only contain one iMessage app extension and may not contain other types of app extensions.")]
    Code0xe80000e6 = 0xe80000e6,
    #[error("This app contains a WatchKit app with one or more Siri Intents app extensions that declare IntentsSupported that are not declared in any of the companion app's Siri Intents app extensions. WatchKit Siri Intents extensions' IntentsSupported values must be a subset of the companion app's Siri Intents extensions' IntentsSupported values.")]
    Code0xe80000e7 = 0xe80000e7,
    #[error("Invalid PIN code entered.")]
    AMDRequireCUPairingCodeError = 0xe80000e8,
    #[error("Retry later.")]
    AMDRequireCUPairingBackoffError = 0xe80000e9,
    #[error("General failure while pairing over the network.")]
    AMDCUPairingError = 0xe80000ea,
    #[error("Continue pairing process over the network.")]
    AMDCUPairingContinueError = 0xe80000eb,
    #[error("Pairing was reset due to earlier issues, try again.")]
    AMDCUPairingResetError = 0xe80000ec,
    #[error("Cannot pair over network yet")]
    AMDRequireCUPairingError = 0xe80000ed,
    #[error("A passcode is required to be set on the device.")]
    AMDPasswordRequiredError = 0xe80000ee,
    #[error("An unknown error has occurred.")]
    Code0xe8008001 = 0xe8008001,
    #[error("Attempted to modify an immutable provisioning profile.")]
    Code0xe8008002 = 0xe8008002,
    #[error("This provisioning profile is malformed.")]
    Code0xe8008003 = 0xe8008003,
    #[error("This provisioning profile does not have a valid signature (or it has a valid, but untrusted signature).")]
    Code0xe8008004 = 0xe8008004,
    #[error("This provisioning profile is malformed.")]
    Code0xe8008005 = 0xe8008005,
    #[error("This provisioning profile is malformed.")]
    Code0xe8008006 = 0xe8008006,
    #[error("This provisioning profile is malformed.")]
    Code0xe8008007 = 0xe8008007,
    #[error("This provisioning profile is malformed.")]
    Code0xe8008008 = 0xe8008008,
    #[error("The signature was not valid.")]
    Code0xe8008009 = 0xe8008009,
    #[error("Unable to allocate memory.")]
    Code0xe800800a = 0xe800800a,
    #[error("A file operation failed.")]
    Code0xe800800b = 0xe800800b,
    #[error("There was an error communicating with your device.")]
    Code0xe800800c = 0xe800800c,
    #[error("There was an error communicating with your device.")]
    Code0xe800800d = 0xe800800d,
    #[error("This provisioning profile does not have a valid signature (or it has a valid, but untrusted signature).")]
    Code0xe800800e = 0xe800800e,
    #[error("The application's signature is valid but it does not match the expected hash.")]
    Code0xe800800f = 0xe800800f,
    #[error("This provisioning profile is unsupported.")]
    Code0xe8008010 = 0xe8008010,
    #[error("This provisioning profile has expired.")]
    Code0xe8008011 = 0xe8008011,
    #[error("This provisioning profile cannot be installed on this device.")]
    Code0xe8008012 = 0xe8008012,
    #[error("This provisioning profile does not have a valid signature (or it has a valid, but untrusted signature).")]
    Code0xe8008013 = 0xe8008013,
    #[error("The executable contains an invalid signature.")]
    Code0xe8008014 = 0xe8008014,
    #[error("A valid provisioning profile for this executable was not found.")]
    Code0xe8008015 = 0xe8008015,
    #[error("The executable was signed with invalid entitlements.")]
    Code0xe8008016 = 0xe8008016,
    #[error("A signed resource has been added, modified, or deleted.")]
    Code0xe8008017 = 0xe8008017,
    #[error("The identity used to sign the executable is no longer valid.")]
    Code0xe8008018 = 0xe8008018,
    #[error("The application does not have a valid signature.")]
    Code0xe8008019 = 0xe8008019,
    #[error("This provisioning profile does not have a valid signature (or it has a valid, but untrusted signature).")]
    Code0xe800801a = 0xe800801a,
    #[error("There was an error communicating with your device.")]
    Code0xe800801b = 0xe800801b,
    #[error("No code signature found.")]
    Code0xe800801c = 0xe800801c,
    #[error("Rejected by policy.")]
    Code0xe800801d = 0xe800801d,
    #[error("The requested profile does not exist (it may have been removed).")]
    Code0xe800801e = 0xe800801e,
    #[error("Attempted to install a Beta profile without the proper entitlement.")]
    Code0xe800801f = 0xe800801f,
    #[error("Attempted to install a Beta profile over lockdown connection.")]
    Code0xe8008020 = 0xe8008020,
    #[error("The maximum number of apps for free development profiles has been reached.")]
    Code0xe8008021 = 0xe8008021,
    #[error("An error occured while accessing the profile database.")]
    Code0xe8008022 = 0xe8008022,
    #[error("An error occured while communicating with the agent.")]
    Code0xe8008023 = 0xe8008023,
    #[error("The provisioning profile is banned.")]
    Code0xe8008024 = 0xe8008024,
    #[error("The user did not explicitly trust the provisioning profile.")]
    Code0xe8008025 = 0xe8008025,
    #[error("The provisioning profile requires online authorization.")]
    Code0xe8008026 = 0xe8008026,
    #[error("The cdhash is not in the trust cache.")]
    Code0xe8008027 = 0xe8008027,
    #[error("Invalid arguments or option combination.")]
    Code0xe8008028 = 0xe8008028,
}

impl AMDeviceError {}

impl From<u32> for AMDeviceError {
    fn from(code: u32) -> Self {
        match code {
            0xe8000001 => AMDeviceError::AMDUndefinedError,
            0xe8000002 => AMDeviceError::AMDBadHeaderError,
            0xe8000003 => AMDeviceError::AMDNoResourcesError,
            0xe8000004 => AMDeviceError::AMDReadError,
            0xe8000005 => AMDeviceError::AMDWriteError,
            0xe8000006 => AMDeviceError::AMDUnknownPacketError,
            0xe8000007 => AMDeviceError::AMDInvalidArgumentError,
            0xe8000008 => AMDeviceError::AMDNotFoundError,
            0xe8000009 => AMDeviceError::AMDIsDirectoryError,
            0xe800000a => AMDeviceError::AMDPermissionError,
            0xe800000b => AMDeviceError::AMDNotConnectedError,
            0xe800000c => AMDeviceError::AMDTimeOutError,
            0xe800000d => AMDeviceError::AMDOverrunError,
            0xe800000e => AMDeviceError::AMDEOFError,
            0xe800000f => AMDeviceError::AMDUnsupportedError,
            0xe8000010 => AMDeviceError::AMDFileExistsError,
            0xe8000011 => AMDeviceError::AMDBusyError,
            0xe8000012 => AMDeviceError::AMDCryptoError,
            0xe8000013 => AMDeviceError::AMDInvalidResponseError,
            0xe8000014 => AMDeviceError::AMDMissingKeyError,
            0xe8000015 => AMDeviceError::AMDMissingValueError,
            0xe8000016 => AMDeviceError::AMDGetProhibitedError,
            0xe8000017 => AMDeviceError::AMDSetProhibitedError,
            0xe8000018 => AMDeviceError::AMDRemoveProhibitedError,
            0xe8000019 => AMDeviceError::AMDImmutableValueError,
            0xe800001a => AMDeviceError::AMDPasswordProtectedError,
            0xe800001b => AMDeviceError::AMDMissingHostIDError,
            0xe800001c => AMDeviceError::AMDInvalidHostIDError,
            0xe800001d => AMDeviceError::AMDSessionActiveError,
            0xe800001e => AMDeviceError::AMDSessionInactiveError,
            0xe800001f => AMDeviceError::AMDMissingSessionIDError,
            0xe8000020 => AMDeviceError::AMDInvalidSessionIDError,
            0xe8000021 => AMDeviceError::AMDMissingServiceError,
            0xe8000022 => AMDeviceError::AMDInvalidServiceError,
            0xe8000023 => AMDeviceError::AMDInvalidCheckinError,
            0xe8000024 => AMDeviceError::AMDCheckinTimeoutError,
            0xe8000025 => AMDeviceError::AMDMissingPairRecordError,
            0xe8000026 => AMDeviceError::AMDInvalidActivationRecordError,
            0xe8000027 => AMDeviceError::AMDMissingActivationRecordError,
            0xe8000028 => AMDeviceError::AMDWrongDroidError,
            0xe8000029 => AMDeviceError::AMDSUVerificationError,
            0xe800002a => AMDeviceError::AMDSUPatchError,
            0xe800002b => AMDeviceError::AMDSUFirmwareError,
            0xe800002c => AMDeviceError::AMDProvisioningProfileNotValid,
            0xe800002d => AMDeviceError::AMDSendMessageError,
            0xe800002e => AMDeviceError::AMDReceiveMessageError,
            0xe800002f => AMDeviceError::AMDMissingOptionsError,
            0xe8000030 => AMDeviceError::AMDMissingImageTypeError,
            0xe8000031 => AMDeviceError::AMDDigestFailedError,
            0xe8000032 => AMDeviceError::AMDStartServiceError,
            0xe8000033 => AMDeviceError::AMDInvalidDiskImageError,
            0xe8000034 => AMDeviceError::AMDMissingDigestError,
            0xe8000035 => AMDeviceError::AMDMuxError,
            0xe8000036 => AMDeviceError::AMDApplicationAlreadyInstalledError,
            0xe8000037 => AMDeviceError::AMDApplicationMoveFailedError,
            0xe8000038 => AMDeviceError::Code0xe8000038,
            0xe8000039 => AMDeviceError::AMDApplicationSandboxFailedError,
            0xe800003a => AMDeviceError::AMDApplicationVerificationFailedError,
            0xe800003b => AMDeviceError::AMDArchiveDestructionFailedError,
            0xe800003c => AMDeviceError::AMDBundleVerificationFailedError,
            0xe800003d => AMDeviceError::AMDCarrierBundleCopyFailedError,
            0xe800003e => AMDeviceError::AMDCarrierBundleDirectoryCreationFailedError,
            0xe800003f => AMDeviceError::AMDCarrierBundleMissingSupportedSIMsError,
            0xe8000040 => AMDeviceError::AMDCommCenterNotificationFailedError,
            0xe8000041 => AMDeviceError::AMDContainerCreationFailedError,
            0xe8000042 => AMDeviceError::AMDContainerP0wnFailedError,
            0xe8000043 => AMDeviceError::AMDContainerRemovalFailedError,
            0xe8000044 => AMDeviceError::AMDEmbeddedProfileInstallFailedError,
            0xe8000045 => AMDeviceError::AMDErrorError,
            0xe8000046 => AMDeviceError::AMDExecutableTwiddleFailedError,
            0xe8000047 => AMDeviceError::AMDExistenceCheckFailedError,
            0xe8000048 => AMDeviceError::AMDInstallMapUpdateFailedError,
            0xe8000049 => AMDeviceError::AMDManifestCaptureFailedError,
            0xe800004a => AMDeviceError::AMDMapGenerationFailedError,
            0xe800004b => AMDeviceError::AMDMissingBundleExecutableError,
            0xe800004c => AMDeviceError::AMDMissingBundleIdentifierError,
            0xe800004d => AMDeviceError::AMDMissingBundlePathError,
            0xe800004e => AMDeviceError::AMDMissingContainerError,
            0xe800004f => AMDeviceError::AMDNotificationFailedError,
            0xe8000050 => AMDeviceError::AMDPackageExtractionFailedError,
            0xe8000051 => AMDeviceError::AMDPackageInspectionFailedError,
            0xe8000052 => AMDeviceError::AMDPackageMoveFailedError,
            0xe8000053 => AMDeviceError::AMDPathConversionFailedError,
            0xe8000054 => AMDeviceError::AMDRestoreContainerFailedError,
            0xe8000055 => AMDeviceError::AMDSeatbeltProfileRemovalFailedError,
            0xe8000056 => AMDeviceError::AMDStageCreationFailedError,
            0xe8000057 => AMDeviceError::AMDSymlinkFailedError,
            0xe8000058 => AMDeviceError::AMDiTunesArtworkCaptureFailedError,
            0xe8000059 => AMDeviceError::AMDiTunesMetadataCaptureFailedError,
            0xe800005a => AMDeviceError::AMDAlreadyArchivedError,
            0xe800005b => AMDeviceError::AMDServiceLimitError,
            0xe800005c => AMDeviceError::AMDInvalidPairRecordError,
            0xe800005d => AMDeviceError::AMDServiceProhibitedError,
            0xe800005e => AMDeviceError::AMDCheckinSetupFailedError,
            0xe800005f => AMDeviceError::AMDCheckinConnectionFailedError,
            0xe8000060 => AMDeviceError::AMDCheckinReceiveFailedError,
            0xe8000061 => AMDeviceError::AMDCheckinResponseFailedError,
            0xe8000062 => AMDeviceError::AMDCheckinSendFailedError,
            0xe8000063 => AMDeviceError::AMDMuxCreateListenerError,
            0xe8000064 => AMDeviceError::AMDMuxGetListenerError,
            0xe8000065 => AMDeviceError::AMDMuxConnectError,
            0xe8000066 => AMDeviceError::AMDUnknownCommandError,
            0xe8000067 => AMDeviceError::AMDAPIInternalError,
            0xe8000068 => AMDeviceError::AMDSavePairRecordFailedError,
            0xe8000069 => AMDeviceError::AMDCheckinOutOfMemoryError,
            0xe800006a => AMDeviceError::AMDDeviceTooNewError,
            0xe800006b => AMDeviceError::AMDDeviceRefNoGood,
            0xe800006c => AMDeviceError::AMDCannotTranslateError,
            0xe800006d => AMDeviceError::AMDMobileImageMounterMissingImageSignature,
            0xe800006e => AMDeviceError::AMDMobileImageMounterResponseCreationFailed,
            0xe800006f => AMDeviceError::AMDMobileImageMounterMissingImageType,
            0xe8000070 => AMDeviceError::AMDMobileImageMounterMissingImagePath,
            0xe8000071 => AMDeviceError::AMDMobileImageMounterImageMapLoadFailed,
            0xe8000072 => AMDeviceError::AMDMobileImageMounterAlreadyMounted,
            0xe8000073 => AMDeviceError::AMDMobileImageMounterImageMoveFailed,
            0xe8000074 => AMDeviceError::AMDMobileImageMounterMountPathMissing,
            0xe8000075 => AMDeviceError::AMDMobileImageMounterMountPathNotEmpty,
            0xe8000076 => AMDeviceError::AMDMobileImageMounterImageMountFailed,
            0xe8000077 => AMDeviceError::AMDMobileImageMounterTrustCacheLoadFailed,
            0xe8000078 => AMDeviceError::AMDMobileImageMounterDigestFailed,
            0xe8000079 => AMDeviceError::AMDMobileImageMounterDigestCreationFailed,
            0xe800007a => AMDeviceError::AMDMobileImageMounterImageVerificationFailed,
            0xe800007b => AMDeviceError::AMDMobileImageMounterImageInfoCreationFailed,
            0xe800007c => AMDeviceError::AMDMobileImageMounterImageMapStoreFailed,
            0xe800007d => AMDeviceError::Code0xe800007d,
            0xe800007e => AMDeviceError::AMDDeviceOSVersionTooLow,
            0xe800007f => AMDeviceError::AMDNoWifiSyncSupportError,
            0xe8000080 => AMDeviceError::AMDDeviceFamilyNotSupported,
            0xe8000081 => AMDeviceError::AMDEscrowLockedError,
            0xe8000082 => AMDeviceError::AMDPairingProhibitedError,
            0xe8000083 => AMDeviceError::AMDProhibitedBySupervision,
            0xe8000084 => AMDeviceError::AMDDeviceDisconnectedError,
            0xe8000085 => AMDeviceError::AMDTooBigError,
            0xe8000086 => AMDeviceError::AMDPackagePatchFailedError,
            0xe8000087 => AMDeviceError::AMDIncorrectArchitectureError,
            0xe8000088 => AMDeviceError::AMDPluginCopyFailedError,
            0xe8000089 => AMDeviceError::AMDBreadcrumbFailedError,
            0xe800008a => AMDeviceError::AMDBreadcrumbUnlockError,
            0xe800008b => AMDeviceError::AMDGeoJSONCaptureFailedError,
            0xe800008c => AMDeviceError::AMDNewsstandArtworkCaptureFailedError,
            0xe800008d => AMDeviceError::AMDMissingCommandError,
            0xe800008e => AMDeviceError::AMDNotEntitledError,
            0xe800008f => AMDeviceError::AMDMissingPackagePathError,
            0xe8000090 => AMDeviceError::AMDMissingContainerPathError,
            0xe8000091 => AMDeviceError::AMDMissingApplicationIdentifierError,
            0xe8000092 => AMDeviceError::AMDMissingAttributeValueError,
            0xe8000093 => AMDeviceError::AMDLookupFailedError,
            0xe8000094 => AMDeviceError::AMDDictCreationFailedError,
            0xe8000095 => AMDeviceError::AMDUserDeniedPairingError,
            0xe8000096 => AMDeviceError::AMDPairingDialogResponsePendingError,
            0xe8000097 => AMDeviceError::AMDInstallProhibitedError,
            0xe8000098 => AMDeviceError::AMDUninstallProhibitedError,
            0xe8000099 => AMDeviceError::AMDFMiPProtectedError,
            0xe800009a => AMDeviceError::AMDMCProtected,
            0xe800009b => AMDeviceError::AMDMCChallengeRequired,
            0xe800009c => AMDeviceError::AMDMissingBundleVersionError,
            0xe800009d => AMDeviceError::AMDAppBlacklistedError,
            0xe800009e => AMDeviceError::Code0xe800009e,
            0xe800009f => AMDeviceError::Code0xe800009f,
            0xe80000a0 => AMDeviceError::Code0xe80000a0,
            0xe80000a1 => AMDeviceError::Code0xe80000a1,
            0xe80000a2 => AMDeviceError::Code0xe80000a2,
            0xe80000a3 => AMDeviceError::Code0xe80000a3,
            0xe80000a4 => AMDeviceError::Code0xe80000a4,
            0xe80000a5 => AMDeviceError::Code0xe80000a5,
            0xe80000a6 => AMDeviceError::Code0xe80000a6,
            0xe80000a7 => AMDeviceError::Code0xe80000a7,
            0xe80000a8 => AMDeviceError::Code0xe80000a8,
            0xe80000a9 => AMDeviceError::Code0xe80000a9,
            0xe80000aa => AMDeviceError::Code0xe80000aa,
            0xe80000ab => AMDeviceError::Code0xe80000ab,
            0xe80000ac => AMDeviceError::Code0xe80000ac,
            0xe80000ad => AMDeviceError::Code0xe80000ad,
            0xe80000ae => AMDeviceError::Code0xe80000ae,
            0xe80000af => AMDeviceError::Code0xe80000af,
            0xe80000b0 => AMDeviceError::Code0xe80000b0,
            0xe80000b1 => AMDeviceError::Code0xe80000b1,
            0xe80000b2 => AMDeviceError::Code0xe80000b2,
            0xe80000b3 => AMDeviceError::MobileHouseArrestMissingCommand,
            0xe80000b4 => AMDeviceError::MobileHouseArrestUnknownCommand,
            0xe80000b5 => AMDeviceError::MobileHouseArrestMissingIdentifier,
            0xe80000b6 => AMDeviceError::MobileHouseArrestDictionaryFailed,
            0xe80000b7 => AMDeviceError::MobileHouseArrestInstallationLookupFailed,
            0xe80000b8 => AMDeviceError::MobileHouseArrestApplicationLookupFailed,
            0xe80000b9 => AMDeviceError::MobileHouseArrestMissingContainer,
            0xe80000bb => AMDeviceError::MobileHouseArrestPathConversionFailed,
            0xe80000bc => AMDeviceError::MobileHouseArrestPathMissing,
            0xe80000bd => AMDeviceError::MobileHouseArrestInvalidPath,
            0xe80000be => AMDeviceError::AMDMismatchedApplicationIdentifierEntitlementError,
            0xe80000bf => AMDeviceError::AMDInvalidSymlinkError,
            0xe80000c0 => AMDeviceError::AMDNoSpaceError,
            0xe80000c1 => AMDeviceError::Code0xe80000c1,
            0xe80000c2 => AMDeviceError::Code0xe80000c2,
            0xe80000c3 => AMDeviceError::AMDBundleiTunesMetadataVersionMismatchError,
            0xe80000c4 => AMDeviceError::AMDInvalidiTunesMetadataPlistError,
            0xe80000c5 => AMDeviceError::AMDMismatchedBundleIDSigningIdentifierError,
            0xe80000c6 => AMDeviceError::Code0xe80000c6,
            0xe80000c7 => AMDeviceError::Code0xe80000c7,
            0xe80000c8 => AMDeviceError::AMDDeviceNotSupportedByThinningError,
            0xe80000c9 => AMDeviceError::Code0xe80000c9,
            0xe80000ca => AMDeviceError::Code0xe80000ca,
            0xe80000cb => AMDeviceError::AMDAppexBundleIDConflictWithOtherIdentifierError,
            0xe80000cc => AMDeviceError::AMDBundleIDConflictWithOtherIdentifierError,
            0xe80000cd => AMDeviceError::Code0xe80000cd,
            0xe80000ce => AMDeviceError::Code0xe80000ce,
            0xe80000cf => AMDeviceError::Code0xe80000cf,
            0xe80000d0 => AMDeviceError::Code0xe80000d0,
            0xe80000d1 => AMDeviceError::Code0xe80000d1,
            0xe80000d2 => AMDeviceError::Code0xe80000d2,
            0xe80000d3 => AMDeviceError::Code0xe80000d3,
            0xe80000d4 => AMDeviceError::Code0xe80000d4,
            0xe80000d5 => AMDeviceError::Code0xe80000d5,
            0xe80000d6 => AMDeviceError::Code0xe80000d6,
            0xe80000d7 => AMDeviceError::Code0xe80000d7,
            0xe80000d8 => AMDeviceError::Code0xe80000d8,
            0xe80000d9 => AMDeviceError::Code0xe80000d9,
            0xe80000da => AMDeviceError::Code0xe80000da,
            0xe80000db => AMDeviceError::Code0xe80000db,
            0xe80000dc => AMDeviceError::Code0xe80000dc,
            0xe80000dd => AMDeviceError::Code0xe80000dd,
            0xe80000de => AMDeviceError::Code0xe80000de,
            0xe80000df => AMDeviceError::Code0xe80000df,
            0xe80000e0 => AMDeviceError::Code0xe80000e0,
            0xe80000e1 => AMDeviceError::Code0xe80000e1,
            0xe80000e2 => AMDeviceError::AMDMobileImageMounterDeviceLocked,
            0xe80000e3 => AMDeviceError::AMDInvalidSINFError,
            0xe80000e4 => AMDeviceError::Code0xe80000e4,
            0xe80000e5 => AMDeviceError::Code0xe80000e5,
            0xe80000e6 => AMDeviceError::Code0xe80000e6,
            0xe80000e7 => AMDeviceError::Code0xe80000e7,
            0xe80000e8 => AMDeviceError::AMDRequireCUPairingCodeError,
            0xe80000e9 => AMDeviceError::AMDRequireCUPairingBackoffError,
            0xe80000ea => AMDeviceError::AMDCUPairingError,
            0xe80000eb => AMDeviceError::AMDCUPairingContinueError,
            0xe80000ec => AMDeviceError::AMDCUPairingResetError,
            0xe80000ed => AMDeviceError::AMDRequireCUPairingError,
            0xe80000ee => AMDeviceError::AMDPasswordRequiredError,
            0xe8008001 => AMDeviceError::Code0xe8008001,
            0xe8008002 => AMDeviceError::Code0xe8008002,
            0xe8008003 => AMDeviceError::Code0xe8008003,
            0xe8008004 => AMDeviceError::Code0xe8008004,
            0xe8008005 => AMDeviceError::Code0xe8008005,
            0xe8008006 => AMDeviceError::Code0xe8008006,
            0xe8008007 => AMDeviceError::Code0xe8008007,
            0xe8008008 => AMDeviceError::Code0xe8008008,
            0xe8008009 => AMDeviceError::Code0xe8008009,
            0xe800800a => AMDeviceError::Code0xe800800a,
            0xe800800b => AMDeviceError::Code0xe800800b,
            0xe800800c => AMDeviceError::Code0xe800800c,
            0xe800800d => AMDeviceError::Code0xe800800d,
            0xe800800e => AMDeviceError::Code0xe800800e,
            0xe800800f => AMDeviceError::Code0xe800800f,
            0xe8008010 => AMDeviceError::Code0xe8008010,
            0xe8008011 => AMDeviceError::Code0xe8008011,
            0xe8008012 => AMDeviceError::Code0xe8008012,
            0xe8008013 => AMDeviceError::Code0xe8008013,
            0xe8008014 => AMDeviceError::Code0xe8008014,
            0xe8008015 => AMDeviceError::Code0xe8008015,
            0xe8008016 => AMDeviceError::Code0xe8008016,
            0xe8008017 => AMDeviceError::Code0xe8008017,
            0xe8008018 => AMDeviceError::Code0xe8008018,
            0xe8008019 => AMDeviceError::Code0xe8008019,
            0xe800801a => AMDeviceError::Code0xe800801a,
            0xe800801b => AMDeviceError::Code0xe800801b,
            0xe800801c => AMDeviceError::Code0xe800801c,
            0xe800801d => AMDeviceError::Code0xe800801d,
            0xe800801e => AMDeviceError::Code0xe800801e,
            0xe800801f => AMDeviceError::Code0xe800801f,
            0xe8008020 => AMDeviceError::Code0xe8008020,
            0xe8008021 => AMDeviceError::Code0xe8008021,
            0xe8008022 => AMDeviceError::Code0xe8008022,
            0xe8008023 => AMDeviceError::Code0xe8008023,
            0xe8008024 => AMDeviceError::Code0xe8008024,
            0xe8008025 => AMDeviceError::Code0xe8008025,
            0xe8008026 => AMDeviceError::Code0xe8008026,
            0xe8008027 => AMDeviceError::Code0xe8008027,
            0xe8008028 => AMDeviceError::Code0xe8008028,
            _ => panic!("Unknown error code."),
        }
    }
}

pub type AMDeviceResult<T> = Result<T, AMDeviceError>;

pub fn to_result(code: u32) -> AMDeviceResult<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(AMDeviceError::from(code))
    }
}
