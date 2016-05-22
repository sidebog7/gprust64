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

impl RegStatus {
    pub fn write(&mut self, data: u32) {
        let coproc = data >> 28;
        self.coproc_usability[3] = (coproc & 0b1000) != 0;
        self.coproc_usability[2] = (coproc & 0b0100) != 0;
        self.coproc_usability[1] = (coproc & 0b0010) != 0;
        self.coproc_usability[0] = (coproc & 0b0001) != 0;

        self.low_power = ((data >> 27) & 0b1) != 0;
        self.fpregs_extend = ((data >> 26) & 0b1) != 0;
        self.reverse_endian = ((data >> 25) & 0b1) != 0;

        let diag_status = ((data >> 16) & 0b111111111) as u16;
        self.diag_status.write(diag_status);
        let interrupt_mask = ((data >> 8) & 0xff) as u8;
        self.interrupt_mask.write(interrupt_mask);

        self.kernel_mode_64bit = ((data >> 7) & 0b1) != 0;
        self.supervisor_mode_64bit = ((data >> 6) & 0b1) != 0;
        self.user_mode_64bit = ((data >> 5) & 0b1) != 0;

        self.mode = data.into();

        self.error_level = data.into();
        self.exception_level = data.into();

        self.interrupt_enabled = (data & 0b1) != 0;
    }
}

#[derive(Debug, Default)]
struct DiagnosticStatus {
    // ITS
    instruction_trace_support: bool,

    // BEV
    tlb_exception_vector_location: ExceptionVectorLocation,

    // TS
    tlb_shutdown: bool,

    // SR
    soft_reset_or_nmi_occurred: bool,

    // CH
    condition_bit: bool,
}

impl DiagnosticStatus {
    fn write(&mut self, data: u16) {
        self.instruction_trace_support = 0b10000000 != 0;
        self.tlb_exception_vector_location = data.into();
        self.tlb_shutdown = 0b100000 != 0;
        self.soft_reset_or_nmi_occurred = 0b10000 != 0;
        self.condition_bit = 0b100 != 0;
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

impl InterruptMask {
    fn write(&mut self, data: u8) {
        self.timer_interrupt = (data & 0b10000000) != 0;
        self.external_interrupt[4] = (data & 0b01000000) != 0;
        self.external_interrupt[3] = (data & 0b00100000) != 0;
        self.external_interrupt[2] = (data & 0b00010000) != 0;
        self.external_interrupt[1] = (data & 0b00001000) != 0;
        self.external_interrupt[0] = (data & 0b00000100) != 0;
        self.software_interrupt[1] = (data & 0b00000010) != 0;
        self.software_interrupt[0] = (data & 0b00000001) != 0;
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
enum ExceptionVectorLocation {
    Normal,
    Bootstrap,
}

impl Default for ExceptionVectorLocation {
    fn default() -> Self {
        ExceptionVectorLocation::Normal
    }
}

impl From<u16> for ExceptionVectorLocation {
    fn from(f: u16) -> Self {
        if f & 0b001000000 == 0 {
            ExceptionVectorLocation::Normal
        } else {
            ExceptionVectorLocation::Bootstrap
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
