use super::*;

cpp_class!(pub unsafe struct SBTarget as "SBTarget");

unsafe impl Send for SBTarget {}

impl SBTarget {
    pub fn is_valid(&self) -> bool {
        cpp!(unsafe [self as "SBTarget*"] -> bool as "bool" {
            return self->IsValid();
        })
    }
    pub fn debugger(&self) -> SBDebugger {
        cpp!(unsafe [self as "SBTarget*"] -> SBDebugger as "SBDebugger" {
            return self->GetDebugger();
        })
    }
    pub fn launch(&self, launch_info: &SBLaunchInfo) -> Result<SBProcess, SBError> {
        let mut error = SBError::new();
        let process = cpp!(unsafe [self as "SBTarget*", launch_info as "SBLaunchInfo*", mut error as "SBError"] -> SBProcess as "SBProcess" {
            return self->Launch(*launch_info, error);
        });
        if error.is_success() {
            Ok(process)
        } else {
            Err(error)
        }
    }
    pub fn attach(&self, attach_info: &SBAttachInfo) -> Result<SBProcess, SBError> {
        let mut error = SBError::new();
        let process = cpp!(unsafe [self as "SBTarget*", attach_info as "SBAttachInfo*", mut error as "SBError"] -> SBProcess as "SBProcess" {
            return self->Attach(*attach_info, error);
        });
        if error.is_success() {
            if process.is_valid() {
                Ok(process)
            } else {
                error.set_error_string("Attach failed.");
                Err(error)
            }
        } else {
            Err(error)
        }
    }
    pub fn attach_to_process_with_id(&self, pid: ProcessID, listener: &SBListener) -> Result<SBProcess, SBError> {
        let error = SBError::new();
        let process = {
            let ref_error = &error;
            cpp!(unsafe [self as "SBTarget*", pid as "lldb::pid_t", listener as "SBListener*",
                                ref_error as "SBError*"] -> SBProcess as "SBProcess" {
                return self->AttachToProcessWithID(*listener, pid, *ref_error);
            })
        };
        if error.is_success() {
            Ok(process)
        } else {
            Err(error)
        }
    }
    pub fn process(&self) -> SBProcess {
        cpp!(unsafe [self as "SBTarget*"] -> SBProcess as "SBProcess" {
            return self->GetProcess();
        })
    }
    pub fn find_breakpoint_by_id(&self, id: BreakpointID) -> Option<SBBreakpoint> {
        let bp = cpp!(unsafe [self as "SBTarget*", id as "break_id_t"] -> SBBreakpoint as "SBBreakpoint" {
            return self->FindBreakpointByID(id);
        });
        if bp.is_valid() {
            Some(bp)
        } else {
            None
        }
    }
    pub fn breakpoint_create_by_location(&self, file: &str, line: u32) -> SBBreakpoint {
        with_cstr(file, |file| {
            cpp!(unsafe [self as "SBTarget*", file as "const char*", line as "uint32_t"] -> SBBreakpoint as "SBBreakpoint" {
                return self->BreakpointCreateByLocation(file, line);
            })
        })
    }
    pub fn breakpoint_create_by_name(&self, name: &str) -> SBBreakpoint {
        with_cstr(name, |name| {
            cpp!(unsafe [self as "SBTarget*", name as "const char*"] -> SBBreakpoint as "SBBreakpoint" {
                return self->BreakpointCreateByName(name);
            })
        })
    }
    pub fn breakpoint_create_by_regex(&self, regex: &str) -> SBBreakpoint {
        with_cstr(regex, |regex| {
            cpp!(unsafe [self as "SBTarget*", regex as "const char*"] -> SBBreakpoint as "SBBreakpoint" {
                return self->BreakpointCreateByRegex(regex);
            })
        })
    }
    pub fn breakpoint_create_for_exception(
        &self, language: LanguageType, catch_bp: bool, throw_bp: bool,
    ) -> SBBreakpoint {
        cpp!(unsafe [self as "SBTarget*", language as "lldb::LanguageType", catch_bp as "bool", throw_bp as "bool"] -> SBBreakpoint as "SBBreakpoint" {
            return self->BreakpointCreateForException(language, catch_bp, throw_bp);
        })
    }
    pub fn breakpoint_create_by_address(&self, address: &SBAddress) -> SBBreakpoint {
        cpp!(unsafe [self as "SBTarget*", address as "SBAddress*"] -> SBBreakpoint as "SBBreakpoint" {
            return self->BreakpointCreateBySBAddress(*address);
        })
    }
    pub fn breakpoint_create_by_absolute_address(&self, address: Address) -> SBBreakpoint {
        cpp!(unsafe [self as "SBTarget*", address as "addr_t"] -> SBBreakpoint as "SBBreakpoint" {
            return self->BreakpointCreateByAddress(address);
        })
    }
    pub fn breakpoint_delete(&self, id: BreakpointID) -> bool {
        cpp!(unsafe [self as "SBTarget*", id as "break_id_t"] -> bool as "bool" {
            return self->BreakpointDelete(id);
        })
    }
    pub fn read_instructions(&self, base_addr: &SBAddress, count: u32) -> SBInstructionList {
        cpp!(unsafe [self as "SBTarget*", base_addr as "SBAddress*", count as "uint32_t"] -> SBInstructionList as "SBInstructionList" {
            return self->ReadInstructions(*base_addr, count);
        })
    }
    pub fn evaluate_expression(&self, expr: &str) -> SBValue {
        with_cstr(expr, |expr| {
            cpp!(unsafe [self as "SBTarget*", expr as "const char*"] -> SBValue as "SBValue" {
                return self->EvaluateExpression(expr);
            })
        })
    }
    pub fn find_functions(&self, name: &str, name_type: FunctionNameType) -> SBSymbolContextList {
        with_cstr(name, |name| {
            cpp!(unsafe [self as "SBTarget*", name as "const char*", name_type as "FunctionNameType"]
                        -> SBSymbolContextList as "SBSymbolContextList" {
                return self->FindFunctions(name, name_type);
            })
        })
    }
    pub fn find_symbols(&self, name: &str, sym_type: SymbolType) -> SBSymbolContextList {
        with_cstr(name, |name| {
            cpp!(unsafe [self as "SBTarget*", name as "const char*", sym_type as "SymbolType"]
                        -> SBSymbolContextList as "SBSymbolContextList" {
                return self->FindSymbols(name, sym_type);
            })
        })
    }
    pub fn resolve_symbol_context_for_address(&self, addr: &SBAddress, scope: SymbolContext) -> SBSymbolContext {
        let addr = addr as *const SBAddress;
        cpp!(unsafe [self as "SBTarget*", addr as "const SBAddress*", scope as "uint32_t"]
                    -> SBSymbolContext as "SBSymbolContext" {
            return self->ResolveSymbolContextForAddress(*addr, scope);
        })
    }
    pub fn broadcaster(&self) -> SBBroadcaster {
        cpp!(unsafe [self as "SBTarget*"] -> SBBroadcaster as "SBBroadcaster" {
            return self->GetBroadcaster();
        })
    }
    pub fn broadcaster_class_name() -> &'static str {
        let ptr = cpp!(unsafe [] -> *const c_char as "const char*" {
            return SBTarget::GetBroadcasterClassName();
        });
        unsafe { CStr::from_ptr(ptr).to_str().unwrap() }
    }
}

impl fmt::Debug for SBTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let full = f.alternate();
        debug_descr(f, |descr| {
            cpp!(unsafe [self as "SBTarget*", descr as "SBStream*", full as "bool"] -> bool as "bool" {
                return self->GetDescription(*descr, full ? eDescriptionLevelFull : eDescriptionLevelBrief);
            })
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum LanguageType {
    Unknown = 0x0000,        // Unknown or invalid language value.
    C89 = 0x0001,            // ISO C:1989.
    C = 0x0002,              // Non-standardized C, such as K&R.
    Ada83 = 0x0003,          // ISO Ada:1983.
    C_plus_plus = 0x0004,    // ISO C++:1998.
    Cobol74 = 0x0005,        // ISO Cobol:1974.
    Cobol85 = 0x0006,        // ISO Cobol:1985.
    Fortran77 = 0x0007,      // ISO Fortran 77.
    Fortran90 = 0x0008,      // ISO Fortran 90.
    Pascal83 = 0x0009,       // ISO Pascal:1983.
    Modula2 = 0x000a,        // ISO Modula-2:1996.
    Java = 0x000b,           // Java.
    C99 = 0x000c,            // ISO C:1999.
    Ada95 = 0x000d,          // ISO Ada:1995.
    Fortran95 = 0x000e,      // ISO Fortran 95.
    PLI = 0x000f,            // ANSI PL/I:1976.
    ObjC = 0x0010,           // Objective-C.
    ObjC_plus_plus = 0x0011, // Objective-C++.
    UPC = 0x0012,            // Unified Parallel C.
    D = 0x0013,              // D.
    Python = 0x0014,         // Python.
    // NOTE: The below are DWARF5 constants, subject to change upon
    // completion of the DWARF5 specification
    OpenCL = 0x0015,         // OpenCL.
    Go = 0x0016,             // Go.
    Modula3 = 0x0017,        // Modula 3.
    Haskell = 0x0018,        // Haskell.
    C_plus_plus_03 = 0x0019, // ISO C++:2003.
    C_plus_plus_11 = 0x001a, // ISO C++:2011.
    OCaml = 0x001b,          // OCaml.
    Rust = 0x001c,           // Rust.
    C11 = 0x001d,            // ISO C:2011.
    Swift = 0x001e,          // Swift.
    Julia = 0x001f,          // Julia.
    Dylan = 0x0020,          // Dylan.
    C_plus_plus_14 = 0x0021, // ISO C++:2014.
    Fortran03 = 0x0022,      // ISO Fortran 2003.
    Fortran08 = 0x0023,      // ISO Fortran 2008.
    // Vendor Extensions
    // Note: Language::GetNameForLanguageType
    // assumes these can be used as indexes into array language_names, and
    // Language::SetLanguageFromCString and Language::AsCString
    // assume these can be used as indexes into array g_languages.
    MipsAssembler = 0x0024,   // Mips_Assembler.
    ExtRenderScript = 0x0025, // RenderScript.
}

bitflags! {
    pub struct FunctionNameType : u32 {
        const None = 0;
        // Automatically figure out which FunctionNameType
        // bits to set based on the function name.
        const Auto = (1 << 1);
        // The function name.
        // For C this is the same as just the name of the function
        // For C++ this is the mangled or demangled version of the mangled name.
        // For ObjC this is the full function signature with the + or
        // - and the square brackets and the class and selector
        const Full = (1<< 2);
        // The function name only, no namespaces
        // or arguments and no class
        // methods or selectors will be searched.
        const Base = (1 << 3);
        // Find function by method name (C++)
        // with no namespace or arguments
        const Method = (1 << 4);
        // Find function by selector name (ObjC) names
        const Selector = (1 << 5);
    }
}

/// These mask bits allow a common interface for queries that can
/// limit the amount of information that gets parsed to only the
/// information that is requested. These bits also can indicate what
/// actually did get resolved during query function calls.
///
/// Each definition corresponds to a one of the member variables
/// in this class, and requests that that item be resolved, or
/// indicates that the member did get resolved.
bitflags! {
    pub struct SymbolContext : u32 {
        ///< Set when \a target is requested from
        ///a query, or was located in query
        ///results
        const Target = (1 << 0);
        ///< Set when \a module is requested from
        ///a query, or was located in query
        ///results
        const Module = (1 << 1);
        ///< Set when \a comp_unit is requested
        ///from a query, or was located in query
        ///results
        const CompUnit = (1 << 2);
        ///< Set when \a function is requested
        ///from a query, or was located in query
        ///results
        const Function = (1 << 3);
        ///< Set when the deepest \a block is
        ///requested from a query, or was located
        ///in query results
        const Block = (1 << 4);
        ///< Set when \a line_entry is
        ///requested from a query, or was
        ///located in query results
        const LineEntry = (1 << 5);
        ///< Set when \a symbol is requested from
        ///a query, or was located in query
        ///results
        const Symbol = (1 << 6);
        ///< Indicates to try and lookup everything
        ///up during a routine symbol context
        ///query.
        const Everything = ((1 << 7) - 1);
        ///< Set when \a global or static
        ///variable is requested from a query, or
        ///was located in query results.
        ///< Variable is potentially expensive to lookup so it isn't
        ///included in
        ///< Everything which stops it from being used during frame PC
        ///lookups and
        ///< many other potential address to symbol context lookups.
        const Variable = (1 << 7);
    }
}
