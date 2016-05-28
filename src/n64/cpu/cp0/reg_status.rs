#[derive(Default, Debug)]
pub struct RegStatus {
    // CU
    coproc_usability: [bool; 4],

    // RP
    low_power: bool,

    // FR
    fpregs_extend: bool,

    // RE
    reverse_endian: bool,

    // DS
    diag_status: DiagnosticStatus,

    // IM
    interrupt_mask: InterruptMask,

    // KX
    kernel_mode_64bit: bool,

    // SX
    supervisor_mode_64bit: bool,

    // UX
    user_mode_64bit: bool,

    // KSU
    mode: Mode,

    // ERL
    error_level: ErrorLevel,

    // EXL
    exception_level: ExceptionLevel,

    // IE
    interrupt_enabled: bool,
}

impl From<u32> for RegStatus {
    fn from(data: u32) -> Self {
        let coproc = data >> 28;
        let diag_status = ((data >> 16) & 0b111111111) as u16;
        let interrupt_mask = ((data >> 8) & 0xff) as u8;
        RegStatus {
            coproc_usability: [(coproc & 0b0001) != 0,
                               (coproc & 0b0010) != 0,
                               (coproc & 0b0100) != 0,
                               (coproc & 0b1000) != 0],
            low_power: ((data >> 27) & 0b1) != 0,
            fpregs_extend: ((data >> 26) & 0b1) != 0,
            reverse_endian: ((data >> 25) & 0b1) != 0,
            diag_status: diag_status.into(),
            interrupt_mask: interrupt_mask.into(),
            kernel_mode_64bit: ((data >> 7) & 0b1) != 0,
            supervisor_mode_64bit: ((data >> 6) & 0b1) != 0,
            user_mode_64bit: ((data >> 5) & 0b1) != 0,
            mode: data.into(),
            error_level: data.into(),
            exception_level: data.into(),
            interrupt_enabled: (data & 0b1) != 0,
        }
    }
}

#[derive(Debug, Default)]
struct DiagnosticStatus {
    // ITS
    instruction_trace_support: bool,

    // BEV
    tlb_exception_vector_location: TLBExceptionVectorLocation,

    // TS
    tlb_shutdown: bool,

    // SR
    soft_reset_or_nmi_occurred: bool,

    // CH
    condition_bit: bool,
}

impl From<u16> for DiagnosticStatus {
    fn from(f: u16) -> Self {
        DiagnosticStatus {
            instruction_trace_support: 0b10000000 != 0,
            tlb_exception_vector_location: f.into(),
            tlb_shutdown: 0b100000 != 0,
            soft_reset_or_nmi_occurred: 0b10000 != 0,
            condition_bit: 0b100 != 0,
        }
    }
}

#[derive(Debug, Default)]
struct InterruptMask {
    // IM(7)
    timer_interrupt: bool,

    // IM(6:2)
    external_interrupt: [bool; 5],

    // IM(1:0)
    software_interrupt: [bool; 2],
}

impl From<u8> for InterruptMask {
    fn from(data: u8) -> Self {
        InterruptMask {
            timer_interrupt: (data & 0b10000000) != 0,
            external_interrupt: [(data & 0b00000100) != 0,
                                 (data & 0b00001000) != 0,
                                 (data & 0b00010000) != 0,
                                 (data & 0b00100000) != 0,
                                 (data & 0b01000000) != 0],
            software_interrupt: [(data & 0b00000001) != 0, (data & 0b00000010) != 0],
        }
    }
}

#[derive(Debug)]
enum Mode {
    // 10 User
    User,

    // 01 Supervisor
    Supervisor,

    // 00 Kernel
    Kernel,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Kernel
    }
}

impl From<u32> for Mode {
    fn from(f: u32) -> Self {
        match (f >> 3) & 0b11 {
            0b00 => Mode::Kernel,
            0b01 => Mode::Supervisor,
            0b10 => Mode::User,
            _ => panic!("Invalid mode {:#b}", f),
        }
    }
}

#[derive(Debug)]
enum TLBExceptionVectorLocation {
    Normal,
    Bootstrap,
}

impl Default for TLBExceptionVectorLocation {
    fn default() -> Self {
        TLBExceptionVectorLocation::Normal
    }
}

impl From<u16> for TLBExceptionVectorLocation {
    fn from(f: u16) -> Self {
        match f & 0b001000000 {
            0 => TLBExceptionVectorLocation::Normal,
            1 => TLBExceptionVectorLocation::Bootstrap,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum ErrorLevel {
    Normal,
    Error,
}

impl Default for ErrorLevel {
    fn default() -> Self {
        ErrorLevel::Normal
    }
}

impl From<u32> for ErrorLevel {
    fn from(f: u32) -> Self {
        if ((f >> 2) & 0b1) != 0 {
            ErrorLevel::Normal
        } else {
            ErrorLevel::Error
        }
    }
}

#[derive(Debug)]
enum ExceptionLevel {
    Normal,
    Exception,
}

impl Default for ExceptionLevel {
    fn default() -> Self {
        ExceptionLevel::Normal
    }
}

impl From<u32> for ExceptionLevel {
    fn from(f: u32) -> Self {
        if ((f >> 2) & 0b1) != 0 {
            ExceptionLevel::Normal
        } else {
            ExceptionLevel::Exception
        }
    }
}
