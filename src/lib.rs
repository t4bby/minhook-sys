use std::ffi::{c_char, c_void};

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum MH_STATUS {
    /// Unknown error. Should not be returned.
    MH_UNKNOWN = -1,

    /// Successful.
    MH_OK = 0,

    /// MinHook is already initialized.
    MH_ERROR_ALREADY_INITIALIZED = 1,

    /// MinHook is not initialized yet, or already uninitialized.
    MH_ERROR_NOT_INITIALIZED = 2,

    /// The hook for the specified target function is already created.
    MH_ERROR_ALREADY_CREATED = 3,

    /// The hook for the specified target function is not created yet.
    MH_ERROR_NOT_CREATED = 4,

    /// The hook for the specified target function is already enabled.
    MH_ERROR_ENABLED = 5,

    /// The hook for the specified target function is not enabled yet, or already
    /// disabled.
    MH_ERROR_DISABLED = 6,

    /// The specified pointer is invalid. It points the address of non-allocated
    /// and/or non-executable region.
    MH_ERROR_NOT_EXECUTABLE = 7,

    /// The specified target function cannot be hooked.
    MH_ERROR_UNSUPPORTED_FUNCTION = 8,

    /// Failed to allocate memory.
    MH_ERROR_MEMORY_ALLOC = 9,

    /// Failed to change the memory protection.
    MH_ERROR_MEMORY_PROTECT = 10,

    /// The specified module is not loaded.
    MH_ERROR_MODULE_NOT_FOUND = 11,

    /// The specified function is not found.
    MH_ERROR_FUNCTION_NOT_FOUND = 12
}



#[link(name = "MinHook")]
extern "system" {

    /// Initialize the MinHook library. You must call this function EXACTLY ONCE
    /// at the beginning of your program.
    pub fn MH_Initialize() -> MH_STATUS;

    /// Uninitialize the MinHook library. You must call this function EXACTLY
    /// ONCE at the end of your program.
    pub fn MH_Uninitialize() -> MH_STATUS;

    /// Creates a hook for the specified target function, in disabled state.
    ///
    /// Parameters:
    ///
    ///   pTarget     [in]  A pointer to the target function, which will be
    ///                     overridden by the detour function.
    ///
    ///   pDetour     [in]  A pointer to the detour function, which will override
    ///                     the target function.
    ///
    ///   ppOriginal  [out] A pointer to the trampoline function, which will be
    ///                     used to call the original target function.
    ///                     This parameter can be NULL.
    pub fn MH_CreateHook(
        pTarget: *mut c_void,
        pDetour: *mut c_void,
        ppOriginal: *mut *mut c_void,
    ) -> MH_STATUS;

    /// Creates a hook for the specified API function, in disabled state.
    ///
    /// Parameters:
    ///
    ///   pszModule   [in]  A pointer to the loaded module name which contains the
    ///                     target function.
    ///
    ///   pszProcName [in]  A pointer to the target function name, which will be
    ///                     overridden by the detour function.
    ///
    ///   pDetour     [in]  A pointer to the detour function, which will override
    ///                     the target function.
    ///
    ///   ppOriginal  [out] A pointer to the trampoline function, which will be
    ///                     used to call the original target function.
    ///                     This parameter can be NULL.
    pub fn MH_CreateHookApi(
        pszModule: *const u16,
        pszProcName: *const c_char,
        pDetour: *mut c_void,
        ppOriginal: *mut *mut c_void,
    ) -> MH_STATUS;

    /// Creates a hook for the specified API function, in disabled state.
    ///
    /// Parameters:
    ///
    ///   pszModule   [in]  A pointer to the loaded module name which contains the
    ///                     target function.
    ///
    ///   pszProcName [in]  A pointer to the target function name, which will be
    ///                     overridden by the detour function.
    ///
    ///   pDetour     [in]  A pointer to the detour function, which will override
    ///                     the target function.
    ///
    ///   ppOriginal  [out] A pointer to the trampoline function, which will be
    ///                     used to call the original target function.
    ///                     This parameter can be NULL.
    ///
    ///   ppTarget    [out] A pointer to the target function, which will be used
    ///                     with other functions.
    ///                     This parameter can be NULL.
    pub fn MH_CreateHookApiEx(
        pszModule: *const u16,
        pszProcName: *const c_char,
        pDetour: *mut c_void,
        ppOriginal: *mut *mut c_void,
        ppTarget: *mut *mut c_void,
    ) -> MH_STATUS;

    /// Removes an already created hook.
    ///
    /// Parameters:
    ///
    ///   pTarget [in] A pointer to the target function.
    pub fn MH_RemoveHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Enables an already created hook.
    ///
    /// Parameters:
    ///
    ///   pTarget [in] A pointer to the target function.
    ///                If this parameter is MH_ALL_HOOKS, all created hooks are
    ///                enabled in one go.
    pub fn MH_EnableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Disables an already created hook.
    ///
    /// Parameters:
    ///
    ///   pTarget [in] A pointer to the target function.
    ///                If this parameter is MH_ALL_HOOKS, all created hooks are
    ///                disabled in one go.
    pub fn MH_DisableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Queues to enable an already created hook.
    ///
    /// Parameters:
    ///
    ///   pTarget [in] A pointer to the target function.
    ///                If this parameter is MH_ALL_HOOKS, all created hooks are
    ///                queued to be enabled.
    pub fn MH_QueueEnableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Queues to disable an already created hook.
    ///
    /// Parameters:
    ///
    ///   pTarget [in] A pointer to the target function.
    ///                If this parameter is MH_ALL_HOOKS, all created hooks are
    ///                queued to be disabled.
    pub fn MH_QueueDisableHook(pTarget: *mut c_void) -> MH_STATUS;

    /// Applies all queued changes in one go.
    pub fn MH_ApplyQueued() -> MH_STATUS;

    /// Translates the MH_STATUS to its name as a string.
    pub fn MH_StatusToString(status: MH_STATUS) -> *const c_char;
}