// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>
#![cfg(windows)]
extern crate setupapi;
use setupapi::*;
#[inline(never)] fn bb<T>(_: T) {}
#[test] #[cfg(target_env = "msvc")]
fn functions_msvc() {
    //bb(SetupGetFileCompressionInfoExW);
    //bb(SetupWriteTextLog);
    //bb(SetupWriteTextLogError);
}
#[test]
fn functions() {
    bb(CMP_WaitNoPendingInstallEvents);
    bb(CM_Add_Empty_Log_Conf);
    bb(CM_Add_Empty_Log_Conf_Ex);
    bb(CM_Add_IDA);
    bb(CM_Add_IDW);
    bb(CM_Add_ID_ExA);
    bb(CM_Add_ID_ExW);
    bb(CM_Add_Range);
    bb(CM_Add_Res_Des);
    bb(CM_Add_Res_Des_Ex);
    bb(CM_Connect_MachineA);
    bb(CM_Connect_MachineW);
    bb(CM_Create_DevNodeA);
    bb(CM_Create_DevNodeW);
    bb(CM_Create_DevNode_ExA);
    bb(CM_Create_DevNode_ExW);
    bb(CM_Create_Range_List);
    bb(CM_Delete_Class_Key);
    bb(CM_Delete_Class_Key_Ex);
    bb(CM_Delete_DevNode_Key);
    bb(CM_Delete_DevNode_Key_Ex);
    bb(CM_Delete_Device_Interface_KeyA);
    bb(CM_Delete_Device_Interface_KeyW);
    bb(CM_Delete_Device_Interface_Key_ExA);
    bb(CM_Delete_Device_Interface_Key_ExW);
    bb(CM_Delete_Range);
    bb(CM_Detect_Resource_Conflict);
    bb(CM_Detect_Resource_Conflict_Ex);
    bb(CM_Disable_DevNode);
    bb(CM_Disable_DevNode_Ex);
    bb(CM_Disconnect_Machine);
    bb(CM_Dup_Range_List);
    bb(CM_Enable_DevNode);
    bb(CM_Enable_DevNode_Ex);
    bb(CM_Enumerate_Classes);
    bb(CM_Enumerate_Classes_Ex);
    bb(CM_Enumerate_EnumeratorsA);
    bb(CM_Enumerate_EnumeratorsW);
    bb(CM_Enumerate_Enumerators_ExA);
    bb(CM_Enumerate_Enumerators_ExW);
    bb(CM_Find_Range);
    bb(CM_First_Range);
    bb(CM_Free_Log_Conf);
    bb(CM_Free_Log_Conf_Ex);
    bb(CM_Free_Log_Conf_Handle);
    bb(CM_Free_Range_List);
    bb(CM_Free_Res_Des);
    bb(CM_Free_Res_Des_Ex);
    bb(CM_Free_Res_Des_Handle);
    bb(CM_Free_Resource_Conflict_Handle);
    bb(CM_Get_Child);
    bb(CM_Get_Child_Ex);
    bb(CM_Get_Class_Key_NameA);
    bb(CM_Get_Class_Key_NameW);
    bb(CM_Get_Class_Key_Name_ExA);
    bb(CM_Get_Class_Key_Name_ExW);
    bb(CM_Get_Class_NameA);
    bb(CM_Get_Class_NameW);
    bb(CM_Get_Class_Name_ExA);
    bb(CM_Get_Class_Name_ExW);
    bb(CM_Get_Class_Registry_PropertyA);
    bb(CM_Get_Class_Registry_PropertyW);
    bb(CM_Get_Depth);
    bb(CM_Get_Depth_Ex);
    bb(CM_Get_DevNode_Custom_PropertyA);
    bb(CM_Get_DevNode_Custom_PropertyW);
    bb(CM_Get_DevNode_Custom_Property_ExA);
    bb(CM_Get_DevNode_Custom_Property_ExW);
    bb(CM_Get_DevNode_Registry_PropertyA);
    bb(CM_Get_DevNode_Registry_PropertyW);
    bb(CM_Get_DevNode_Registry_Property_ExA);
    bb(CM_Get_DevNode_Registry_Property_ExW);
    bb(CM_Get_DevNode_Status);
    bb(CM_Get_DevNode_Status_Ex);
    bb(CM_Get_Device_IDA);
    bb(CM_Get_Device_IDW);
    bb(CM_Get_Device_ID_ExA);
    bb(CM_Get_Device_ID_ExW);
    bb(CM_Get_Device_ID_ListA);
    bb(CM_Get_Device_ID_ListW);
    bb(CM_Get_Device_ID_List_ExA);
    bb(CM_Get_Device_ID_List_ExW);
    bb(CM_Get_Device_ID_List_SizeA);
    bb(CM_Get_Device_ID_List_SizeW);
    bb(CM_Get_Device_ID_List_Size_ExA);
    bb(CM_Get_Device_ID_List_Size_ExW);
    bb(CM_Get_Device_ID_Size);
    bb(CM_Get_Device_ID_Size_Ex);
    bb(CM_Get_Device_Interface_AliasA);
    bb(CM_Get_Device_Interface_AliasW);
    bb(CM_Get_Device_Interface_Alias_ExA);
    bb(CM_Get_Device_Interface_Alias_ExW);
    bb(CM_Get_Device_Interface_ListA);
    bb(CM_Get_Device_Interface_ListW);
    bb(CM_Get_Device_Interface_List_ExA);
    bb(CM_Get_Device_Interface_List_ExW);
    bb(CM_Get_Device_Interface_List_SizeA);
    bb(CM_Get_Device_Interface_List_SizeW);
    bb(CM_Get_Device_Interface_List_Size_ExA);
    bb(CM_Get_Device_Interface_List_Size_ExW);
    bb(CM_Get_First_Log_Conf);
    bb(CM_Get_First_Log_Conf_Ex);
    bb(CM_Get_Global_State);
    bb(CM_Get_Global_State_Ex);
    bb(CM_Get_HW_Prof_FlagsA);
    bb(CM_Get_HW_Prof_FlagsW);
    bb(CM_Get_HW_Prof_Flags_ExA);
    bb(CM_Get_HW_Prof_Flags_ExW);
    bb(CM_Get_Hardware_Profile_InfoA);
    bb(CM_Get_Hardware_Profile_InfoW);
    bb(CM_Get_Hardware_Profile_Info_ExA);
    bb(CM_Get_Hardware_Profile_Info_ExW);
    bb(CM_Get_Log_Conf_Priority);
    bb(CM_Get_Log_Conf_Priority_Ex);
    bb(CM_Get_Next_Log_Conf);
    bb(CM_Get_Next_Log_Conf_Ex);
    bb(CM_Get_Next_Res_Des);
    bb(CM_Get_Next_Res_Des_Ex);
    bb(CM_Get_Parent);
    bb(CM_Get_Parent_Ex);
    bb(CM_Get_Res_Des_Data);
    bb(CM_Get_Res_Des_Data_Ex);
    bb(CM_Get_Res_Des_Data_Size);
    bb(CM_Get_Res_Des_Data_Size_Ex);
    bb(CM_Get_Resource_Conflict_Count);
    bb(CM_Get_Resource_Conflict_DetailsA);
    bb(CM_Get_Resource_Conflict_DetailsW);
    bb(CM_Get_Sibling);
    bb(CM_Get_Sibling_Ex);
    bb(CM_Get_Version);
    bb(CM_Get_Version_Ex);
    bb(CM_Intersect_Range_List);
    bb(CM_Invert_Range_List);
    bb(CM_Is_Dock_Station_Present);
    bb(CM_Is_Dock_Station_Present_Ex);
    bb(CM_Is_Version_Available);
    bb(CM_Is_Version_Available_Ex);
    bb(CM_Locate_DevNodeA);
    bb(CM_Locate_DevNodeW);
    bb(CM_Locate_DevNode_ExA);
    bb(CM_Locate_DevNode_ExW);
    bb(CM_Merge_Range_List);
    bb(CM_Modify_Res_Des);
    bb(CM_Modify_Res_Des_Ex);
    bb(CM_Move_DevNode);
    bb(CM_Move_DevNode_Ex);
    bb(CM_Next_Range);
    bb(CM_Open_Class_KeyA);
    bb(CM_Open_Class_KeyW);
    bb(CM_Open_Class_Key_ExA);
    bb(CM_Open_Class_Key_ExW);
    bb(CM_Open_DevNode_Key);
    bb(CM_Open_DevNode_Key_Ex);
    bb(CM_Open_Device_Interface_KeyA);
    bb(CM_Open_Device_Interface_KeyW);
    bb(CM_Open_Device_Interface_Key_ExA);
    bb(CM_Open_Device_Interface_Key_ExW);
    bb(CM_Query_And_Remove_SubTreeA);
    bb(CM_Query_And_Remove_SubTreeW);
    bb(CM_Query_And_Remove_SubTree_ExA);
    bb(CM_Query_And_Remove_SubTree_ExW);
    bb(CM_Query_Arbitrator_Free_Data);
    bb(CM_Query_Arbitrator_Free_Data_Ex);
    bb(CM_Query_Arbitrator_Free_Size);
    bb(CM_Query_Arbitrator_Free_Size_Ex);
    bb(CM_Query_Remove_SubTree);
    bb(CM_Query_Remove_SubTree_Ex);
    bb(CM_Query_Resource_Conflict_List);
    bb(CM_Reenumerate_DevNode);
    bb(CM_Reenumerate_DevNode_Ex);
    bb(CM_Register_Device_Driver);
    bb(CM_Register_Device_Driver_Ex);
    bb(CM_Register_Device_InterfaceA);
    bb(CM_Register_Device_InterfaceW);
    bb(CM_Register_Device_Interface_ExA);
    bb(CM_Register_Device_Interface_ExW);
    bb(CM_Remove_SubTree);
    bb(CM_Remove_SubTree_Ex);
    bb(CM_Request_Device_EjectA);
    bb(CM_Request_Device_EjectW);
    bb(CM_Request_Device_Eject_ExA);
    bb(CM_Request_Device_Eject_ExW);
    bb(CM_Request_Eject_PC);
    bb(CM_Request_Eject_PC_Ex);
    bb(CM_Run_Detection);
    bb(CM_Run_Detection_Ex);
    bb(CM_Set_Class_Registry_PropertyA);
    bb(CM_Set_Class_Registry_PropertyW);
    bb(CM_Set_DevNode_Problem);
    bb(CM_Set_DevNode_Problem_Ex);
    bb(CM_Set_DevNode_Registry_PropertyA);
    bb(CM_Set_DevNode_Registry_PropertyW);
    bb(CM_Set_DevNode_Registry_Property_ExA);
    bb(CM_Set_DevNode_Registry_Property_ExW);
    bb(CM_Set_HW_Prof);
    bb(CM_Set_HW_Prof_Ex);
    bb(CM_Set_HW_Prof_FlagsA);
    bb(CM_Set_HW_Prof_FlagsW);
    bb(CM_Set_HW_Prof_Flags_ExA);
    bb(CM_Set_HW_Prof_Flags_ExW);
    bb(CM_Setup_DevNode);
    bb(CM_Setup_DevNode_Ex);
    bb(CM_Test_Range_Available);
    bb(CM_Uninstall_DevNode);
    bb(CM_Uninstall_DevNode_Ex);
    bb(CM_Unregister_Device_InterfaceA);
    bb(CM_Unregister_Device_InterfaceW);
    bb(CM_Unregister_Device_Interface_ExA);
    bb(CM_Unregister_Device_Interface_ExW);
    bb(InstallHinfSection);
    bb(InstallHinfSectionA);
    bb(InstallHinfSectionW);
    bb(SetupAddInstallSectionToDiskSpaceListA);
    bb(SetupAddInstallSectionToDiskSpaceListW);
    bb(SetupAddSectionToDiskSpaceListA);
    bb(SetupAddSectionToDiskSpaceListW);
    bb(SetupAddToDiskSpaceListA);
    bb(SetupAddToDiskSpaceListW);
    bb(SetupAddToSourceListA);
    bb(SetupAddToSourceListW);
    bb(SetupAdjustDiskSpaceListA);
    bb(SetupAdjustDiskSpaceListW);
    bb(SetupBackupErrorA);
    bb(SetupBackupErrorW);
    bb(SetupCancelTemporarySourceList);
    bb(SetupCloseFileQueue);
    bb(SetupCloseInfFile);
    bb(SetupCloseLog);
    bb(SetupCommitFileQueue);
    bb(SetupCommitFileQueueA);
    bb(SetupCommitFileQueueW);
    bb(SetupConfigureWmiFromInfSectionA);
    bb(SetupConfigureWmiFromInfSectionW);
    bb(SetupCopyErrorA);
    bb(SetupCopyErrorW);
    bb(SetupCopyOEMInfA);
    bb(SetupCopyOEMInfW);
    bb(SetupCreateDiskSpaceListA);
    bb(SetupCreateDiskSpaceListW);
    bb(SetupDecompressOrCopyFileA);
    bb(SetupDecompressOrCopyFileW);
    bb(SetupDefaultQueueCallback);
    bb(SetupDefaultQueueCallbackA);
    bb(SetupDefaultQueueCallbackW);
    bb(SetupDeleteErrorA);
    bb(SetupDeleteErrorW);
    bb(SetupDestroyDiskSpaceList);
    bb(SetupDiAskForOEMDisk);
    bb(SetupDiBuildClassInfoList);
    bb(SetupDiBuildClassInfoListExA);
    bb(SetupDiBuildClassInfoListExW);
    bb(SetupDiBuildDriverInfoList);
    bb(SetupDiCallClassInstaller);
    bb(SetupDiCancelDriverInfoSearch);
    bb(SetupDiChangeState);
    bb(SetupDiClassGuidsFromNameA);
    bb(SetupDiClassGuidsFromNameExA);
    bb(SetupDiClassGuidsFromNameExW);
    bb(SetupDiClassGuidsFromNameW);
    bb(SetupDiClassNameFromGuidA);
    bb(SetupDiClassNameFromGuidExA);
    bb(SetupDiClassNameFromGuidExW);
    bb(SetupDiClassNameFromGuidW);
    bb(SetupDiCreateDevRegKeyA);
    bb(SetupDiCreateDevRegKeyW);
    bb(SetupDiCreateDeviceInfoA);
    bb(SetupDiCreateDeviceInfoList);
    bb(SetupDiCreateDeviceInfoListExA);
    bb(SetupDiCreateDeviceInfoListExW);
    bb(SetupDiCreateDeviceInfoW);
    bb(SetupDiCreateDeviceInterfaceA);
    bb(SetupDiCreateDeviceInterfaceRegKeyA);
    bb(SetupDiCreateDeviceInterfaceRegKeyW);
    bb(SetupDiCreateDeviceInterfaceW);
    bb(SetupDiDeleteDevRegKey);
    bb(SetupDiDeleteDeviceInfo);
    bb(SetupDiDeleteDeviceInterfaceData);
    bb(SetupDiDeleteDeviceInterfaceRegKey);
    bb(SetupDiDestroyClassImageList);
    bb(SetupDiDestroyDeviceInfoList);
    bb(SetupDiDestroyDriverInfoList);
    bb(SetupDiDrawMiniIcon);
    bb(SetupDiEnumDeviceInfo);
    bb(SetupDiEnumDeviceInterfaces);
    bb(SetupDiEnumDriverInfoA);
    bb(SetupDiEnumDriverInfoW);
    bb(SetupDiGetActualModelsSectionA);
    bb(SetupDiGetActualModelsSectionW);
    bb(SetupDiGetActualSectionToInstallA);
    bb(SetupDiGetActualSectionToInstallExA);
    bb(SetupDiGetActualSectionToInstallExW);
    bb(SetupDiGetActualSectionToInstallW);
    bb(SetupDiGetClassBitmapIndex);
    bb(SetupDiGetClassDescriptionA);
    bb(SetupDiGetClassDescriptionExA);
    bb(SetupDiGetClassDescriptionExW);
    bb(SetupDiGetClassDescriptionW);
    bb(SetupDiGetClassDevPropertySheetsA);
    bb(SetupDiGetClassDevPropertySheetsW);
    bb(SetupDiGetClassDevsA);
    bb(SetupDiGetClassDevsExA);
    bb(SetupDiGetClassDevsExW);
    bb(SetupDiGetClassDevsW);
    bb(SetupDiGetClassImageIndex);
    bb(SetupDiGetClassImageList);
    bb(SetupDiGetClassImageListExA);
    bb(SetupDiGetClassImageListExW);
    bb(SetupDiGetClassInstallParamsA);
    bb(SetupDiGetClassInstallParamsW);
    bb(SetupDiGetClassPropertyExW);
    bb(SetupDiGetClassPropertyKeys);
    bb(SetupDiGetClassPropertyKeysExW);
    bb(SetupDiGetClassPropertyW);
    bb(SetupDiGetClassRegistryPropertyA);
    bb(SetupDiGetClassRegistryPropertyW);
    bb(SetupDiGetCustomDevicePropertyA);
    bb(SetupDiGetCustomDevicePropertyW);
    bb(SetupDiGetDeviceInfoListClass);
    bb(SetupDiGetDeviceInfoListDetailA);
    bb(SetupDiGetDeviceInfoListDetailW);
    bb(SetupDiGetDeviceInstallParamsA);
    bb(SetupDiGetDeviceInstallParamsW);
    bb(SetupDiGetDeviceInstanceIdA);
    bb(SetupDiGetDeviceInstanceIdW);
    bb(SetupDiGetDeviceInterfaceAlias);
    bb(SetupDiGetDeviceInterfaceDetailA);
    bb(SetupDiGetDeviceInterfaceDetailW);
    bb(SetupDiGetDeviceInterfacePropertyKeys);
    bb(SetupDiGetDeviceInterfacePropertyW);
    bb(SetupDiGetDevicePropertyKeys);
    bb(SetupDiGetDevicePropertyW);
    bb(SetupDiGetDeviceRegistryPropertyA);
    bb(SetupDiGetDeviceRegistryPropertyW);
    bb(SetupDiGetDriverInfoDetailA);
    bb(SetupDiGetDriverInfoDetailW);
    bb(SetupDiGetDriverInstallParamsA);
    bb(SetupDiGetDriverInstallParamsW);
    bb(SetupDiGetHwProfileFriendlyNameA);
    bb(SetupDiGetHwProfileFriendlyNameExA);
    bb(SetupDiGetHwProfileFriendlyNameExW);
    bb(SetupDiGetHwProfileFriendlyNameW);
    bb(SetupDiGetHwProfileList);
    bb(SetupDiGetHwProfileListExA);
    bb(SetupDiGetHwProfileListExW);
    bb(SetupDiGetINFClassA);
    bb(SetupDiGetINFClassW);
    bb(SetupDiGetSelectedDevice);
    bb(SetupDiGetSelectedDriverA);
    bb(SetupDiGetSelectedDriverW);
    bb(SetupDiGetWizardPage);
    bb(SetupDiInstallClassA);
    bb(SetupDiInstallClassExA);
    bb(SetupDiInstallClassExW);
    bb(SetupDiInstallClassW);
    bb(SetupDiInstallDevice);
    bb(SetupDiInstallDeviceInterfaces);
    bb(SetupDiInstallDriverFiles);
    bb(SetupDiLoadClassIcon);
    bb(SetupDiLoadDeviceIcon);
    bb(SetupDiOpenClassRegKey);
    bb(SetupDiOpenClassRegKeyExA);
    bb(SetupDiOpenClassRegKeyExW);
    bb(SetupDiOpenDevRegKey);
    bb(SetupDiOpenDeviceInfoA);
    bb(SetupDiOpenDeviceInfoW);
    bb(SetupDiOpenDeviceInterfaceA);
    bb(SetupDiOpenDeviceInterfaceRegKey);
    bb(SetupDiOpenDeviceInterfaceW);
    bb(SetupDiRegisterCoDeviceInstallers);
    bb(SetupDiRegisterDeviceInfo);
    bb(SetupDiRemoveDevice);
    bb(SetupDiRemoveDeviceInterface);
    bb(SetupDiRestartDevices);
    bb(SetupDiSelectBestCompatDrv);
    bb(SetupDiSelectDevice);
    bb(SetupDiSelectOEMDrv);
    bb(SetupDiSetClassInstallParamsA);
    bb(SetupDiSetClassInstallParamsW);
    bb(SetupDiSetClassPropertyExW);
    bb(SetupDiSetClassPropertyW);
    bb(SetupDiSetClassRegistryPropertyA);
    bb(SetupDiSetClassRegistryPropertyW);
    bb(SetupDiSetDeviceInstallParamsA);
    bb(SetupDiSetDeviceInstallParamsW);
    bb(SetupDiSetDeviceInterfaceDefault);
    bb(SetupDiSetDeviceInterfacePropertyW);
    bb(SetupDiSetDevicePropertyW);
    bb(SetupDiSetDeviceRegistryPropertyA);
    bb(SetupDiSetDeviceRegistryPropertyW);
    bb(SetupDiSetDriverInstallParamsA);
    bb(SetupDiSetDriverInstallParamsW);
    bb(SetupDiSetSelectedDevice);
    bb(SetupDiSetSelectedDriverA);
    bb(SetupDiSetSelectedDriverW);
    bb(SetupDiUnremoveDevice);
    bb(SetupDuplicateDiskSpaceListA);
    bb(SetupDuplicateDiskSpaceListW);
    bb(SetupEnumInfSectionsA);
    bb(SetupEnumInfSectionsW);
    bb(SetupFindFirstLineA);
    bb(SetupFindFirstLineW);
    bb(SetupFindNextLine);
    bb(SetupFindNextMatchLineA);
    bb(SetupFindNextMatchLineW);
    bb(SetupFreeSourceListA);
    bb(SetupFreeSourceListW);
    bb(SetupGetBackupInformationA);
    bb(SetupGetBackupInformationW);
    bb(SetupGetBinaryField);
    bb(SetupGetFieldCount);
    bb(SetupGetFileCompressionInfoA);
    bb(SetupGetFileCompressionInfoExA);
    bb(SetupGetFileCompressionInfoW);
    bb(SetupGetFileQueueCount);
    bb(SetupGetFileQueueFlags);
    bb(SetupGetInfDriverStoreLocationA);
    bb(SetupGetInfDriverStoreLocationW);
    bb(SetupGetInfFileListA);
    bb(SetupGetInfFileListW);
    bb(SetupGetInfInformationA);
    bb(SetupGetInfInformationW);
    bb(SetupGetInfPublishedNameA);
    bb(SetupGetInfPublishedNameW);
    bb(SetupGetIntField);
    bb(SetupGetLineByIndexA);
    bb(SetupGetLineByIndexW);
    bb(SetupGetLineCountA);
    bb(SetupGetLineCountW);
    bb(SetupGetLineTextA);
    bb(SetupGetLineTextW);
    bb(SetupGetMultiSzFieldA);
    bb(SetupGetMultiSzFieldW);
    bb(SetupGetNonInteractiveMode);
    bb(SetupGetSourceFileLocationA);
    bb(SetupGetSourceFileLocationW);
    bb(SetupGetSourceFileSizeA);
    bb(SetupGetSourceFileSizeW);
    bb(SetupGetSourceInfoA);
    bb(SetupGetSourceInfoW);
    bb(SetupGetStringFieldA);
    bb(SetupGetStringFieldW);
    bb(SetupGetTargetPathA);
    bb(SetupGetTargetPathW);
    bb(SetupGetThreadLogToken);
    bb(SetupInitDefaultQueueCallback);
    bb(SetupInitDefaultQueueCallbackEx);
    bb(SetupInitializeFileLogA);
    bb(SetupInitializeFileLogW);
    bb(SetupInstallFileA);
    bb(SetupInstallFileExA);
    bb(SetupInstallFileExW);
    bb(SetupInstallFileW);
    bb(SetupInstallFilesFromInfSectionA);
    bb(SetupInstallFilesFromInfSectionW);
    bb(SetupInstallFromInfSectionA);
    bb(SetupInstallFromInfSectionW);
    bb(SetupInstallServicesFromInfSectionA);
    bb(SetupInstallServicesFromInfSectionExA);
    bb(SetupInstallServicesFromInfSectionExW);
    bb(SetupInstallServicesFromInfSectionW);
    bb(SetupIterateCabinetA);
    bb(SetupIterateCabinetW);
    bb(SetupLogErrorA);
    bb(SetupLogErrorW);
    bb(SetupLogFileA);
    bb(SetupLogFileW);
    bb(SetupOpenAppendInfFileA);
    bb(SetupOpenAppendInfFileW);
    bb(SetupOpenFileQueue);
    bb(SetupOpenInfFileA);
    bb(SetupOpenInfFileW);
    bb(SetupOpenLog);
    bb(SetupOpenMasterInf);
    bb(SetupPrepareQueueForRestoreA);
    bb(SetupPrepareQueueForRestoreW);
    bb(SetupPromptForDiskA);
    bb(SetupPromptForDiskW);
    bb(SetupPromptReboot);
    bb(SetupQueryDrivesInDiskSpaceListA);
    bb(SetupQueryDrivesInDiskSpaceListW);
    bb(SetupQueryFileLogA);
    bb(SetupQueryFileLogW);
    bb(SetupQueryInfFileInformationA);
    bb(SetupQueryInfFileInformationW);
    bb(SetupQueryInfOriginalFileInformationA);
    bb(SetupQueryInfOriginalFileInformationW);
    bb(SetupQueryInfVersionInformationA);
    bb(SetupQueryInfVersionInformationW);
    bb(SetupQuerySourceListA);
    bb(SetupQuerySourceListW);
    bb(SetupQuerySpaceRequiredOnDriveA);
    bb(SetupQuerySpaceRequiredOnDriveW);
    bb(SetupQueueCopyA);
    bb(SetupQueueCopyIndirectA);
    bb(SetupQueueCopyIndirectW);
    bb(SetupQueueCopySectionA);
    bb(SetupQueueCopySectionW);
    bb(SetupQueueCopyW);
    bb(SetupQueueDefaultCopyA);
    bb(SetupQueueDefaultCopyW);
    bb(SetupQueueDeleteA);
    bb(SetupQueueDeleteSectionA);
    bb(SetupQueueDeleteSectionW);
    bb(SetupQueueDeleteW);
    bb(SetupQueueRenameA);
    bb(SetupQueueRenameSectionA);
    bb(SetupQueueRenameSectionW);
    bb(SetupQueueRenameW);
    bb(SetupRemoveFileLogEntryA);
    bb(SetupRemoveFileLogEntryW);
    bb(SetupRemoveFromDiskSpaceListA);
    bb(SetupRemoveFromDiskSpaceListW);
    bb(SetupRemoveFromSourceListA);
    bb(SetupRemoveFromSourceListW);
    bb(SetupRemoveInstallSectionFromDiskSpaceListA);
    bb(SetupRemoveInstallSectionFromDiskSpaceListW);
    bb(SetupRemoveSectionFromDiskSpaceListA);
    bb(SetupRemoveSectionFromDiskSpaceListW);
    bb(SetupRenameErrorA);
    bb(SetupRenameErrorW);
    bb(SetupScanFileQueue);
    bb(SetupScanFileQueueA);
    bb(SetupScanFileQueueW);
    bb(SetupSetDirectoryIdA);
    bb(SetupSetDirectoryIdExA);
    bb(SetupSetDirectoryIdExW);
    bb(SetupSetDirectoryIdW);
    bb(SetupSetFileQueueAlternatePlatformA);
    bb(SetupSetFileQueueAlternatePlatformW);
    bb(SetupSetFileQueueFlags);
    bb(SetupSetNonInteractiveMode);
    bb(SetupSetPlatformPathOverrideA);
    bb(SetupSetPlatformPathOverrideW);
    bb(SetupSetSourceListA);
    bb(SetupSetSourceListW);
    bb(SetupSetThreadLogToken);
    bb(SetupTermDefaultQueueCallback);
    bb(SetupTerminateFileLog);
    bb(SetupUninstallNewlyCopiedInfs);
    bb(SetupUninstallOEMInfA);
    bb(SetupUninstallOEMInfW);
    bb(SetupVerifyInfFileA);
    bb(SetupVerifyInfFileW);
    bb(SetupWriteTextLogInfLine);
}