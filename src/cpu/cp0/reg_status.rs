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

        let diag_status = ((data >> 16) & 0xff) as u8;
        self.diag_status.write(diag_status);
        let interrupt_mask = ((data >> 8) & 0xff) as u8;
        self.interrupt_mask.write(interrupt_mask);

        self.kernel_mode_64bit = ((data >> 7) & 0b1) != 0;
        self.supervisor_mode_64bit = ((data >> 6) & 0b1) != 0;
        self.user_mode_64bit = ((data >> 5) & 0b1) != 0;

        let mode = ((data >> 3) & 0b11) as u8;
        self.mode = match mode {
            0b00 => Mode::Kernel,
            0b01 => Mode::Supervisor,
            0b10 => Mode::User,
            _ => panic!("Invalid mode {:#b}", mode),
        };

        self.error_level = if ((data >> 2) & 0b1) != 0 {
            ErrorLevel::Normal
        } else {
            ErrorLevel::Error
        };


        self.exception_level = if ((data >> 2) & 0b1) != 0 {
            ExceptionLevel::Normal
        } else {
            ExceptionLevel::Exception
        };

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
    fn write(&mut self, data: u8) {
        self.instruction_trace_support = 0b10000000 != 0;

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
    fn default() -> Mode {
        Mode::Kernel
    }
}

#[derive(Debug)]
enum ExceptionVectorLocation {
    Normal,
    Bootstrap,
}

impl Default for ExceptionVectorLocation {
    fn default() -> ExceptionVectorLocation {
        ExceptionVectorLocation::Normal
    }
}

#[derive(Debug)]
enum ErrorLevel {
    Normal,
    Error,
}

impl Default for ErrorLevel {
    fn default() -> ErrorLevel {
        ErrorLevel::Normal
    }
}

#[derive(Debug)]
enum ExceptionLevel {
    Normal,
    Exception,
}

impl Default for ExceptionLevel {
    fn default() -> ExceptionLevel {
        ExceptionLevel::Normal
    }
}
