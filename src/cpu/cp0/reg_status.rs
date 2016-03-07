#[derive(Debug, Default)]
pub struct RegStatus {
	// CU
	coprocessor_usability: [bool; 4],
	// RP
	low_power: bool,
	// FR
	additional_fp_regs: bool,
	// RE
	reverse_endian: bool,
	// DS
	diagnostic_status: DiagnosticStatus,
	// IM
	interrupt_mask: InterruptMask,
	// KX
	kernel_mode_64bit_addressing: bool,
	// SX
	supervisor_mode_64bit_addressing: bool,
	// UX
	user_mode_64bit_addressing: bool,
	//KSU
	mode: Mode,
	// ERL
	error_level: bool,
	// EXL
	exception_level: bool,
	// IE
	interrupts_enabled: bool
}

impl From<u32> for RegStatus {
	fn from(data: u32) -> Self {
		RegStatus {
			coprocessor_usability: [
												(data & 0x1000_0000) != 0, // 0b0001 0000 0000 0000
												(data & 0x2000_0000) != 0, // 0b0010 0000 0000 0000
												(data & 0x4000_0000) != 0, // 0b0100 0000 0000 0000
												(data & 0x8000_0000) != 0, // 0b1000 0000 0000 0000
			],
			low_power: 							(data & 0x0800_0000) != 0,
			additional_fp_regs: 				(data & 0x0400_0000) != 0,
			reverse_endian: 					(data & 0x0200_0000) != 0,
			diagnostic_status: 					data.into(),
			interrupt_mask: 					data.into(),
			kernel_mode_64bit_addressing: 		(data & 0x0000_0040) != 0,
			supervisor_mode_64bit_addressing: 	(data & 0x0000_0020) != 0,
			user_mode_64bit_addressing: 		(data & 0x0000_0010) != 0,
			mode: 								data.into(),
			error_level: 						(data & 0x0000_0004) != 0,
			exception_level: 					(data & 0x0000_0002) != 0,
			interrupts_enabled: 				(data & 0x0000_0001) != 0
		}
	}
}

#[derive(Debug, Default)]
struct DiagnosticStatus {
	// TTS
	instruction_trace_support: bool,
	// BEV
	tlb_general_exception_vector_location: TLBGeneralExceptionVectorLocation,
	// TS
	tlb_shutdown: bool,
	// SR
	soft_reset_or_nmi_occured: bool,
	// CH
	condition_bit: bool
}

impl From<u32> for DiagnosticStatus {
	fn from(data: u32) -> Self {
		DiagnosticStatus {
			instruction_trace_support: (data & 0x0100_0000) != 0,
			tlb_general_exception_vector_location: 
				TLBGeneralExceptionVectorLocation::from(data),
			tlb_shutdown:				(data & 0x0040_0000) != 0,
			soft_reset_or_nmi_occured: 	(data & 0x0020_0000) != 0,
			condition_bit: (data & 0x0008_0000) != 0,
		}
	}
}

#[derive(Debug)]
enum TLBGeneralExceptionVectorLocation {
	Normal,
	Bootstrap
}

impl Default for TLBGeneralExceptionVectorLocation {
	fn default() -> TLBGeneralExceptionVectorLocation {
		TLBGeneralExceptionVectorLocation::Normal
	}
}

impl From<u32> for TLBGeneralExceptionVectorLocation {
	fn from(data: u32) -> TLBGeneralExceptionVectorLocation {
		match data & 0x0080_0000 {
			0b0 => TLBGeneralExceptionVectorLocation::Normal,
			0b1 => TLBGeneralExceptionVectorLocation::Bootstrap,
			_ => unreachable!()
		}
	}
}

#[derive(Debug, Default)]
struct InterruptMask {
	// IM(7)
	timer_interrupt: bool,
	// IM(6:2)
	external_interrupt_write_req: [bool; 5],
	// IM(1:0)
	software_interrupts_cause_reg: [bool; 2]
}

impl From<u32> for InterruptMask {
	fn from(data: u32) -> Self {
		InterruptMask {
			timer_interrupt: (data & 0x0000_8000) != 0,
			external_interrupt_write_req: [
				(data & 0x0000_0400) != 0,
				(data & 0x0000_0800) != 0,
				(data & 0x0000_1000) != 0,
				(data & 0x0000_2000) != 0,
				(data & 0x0000_4000) != 0,
			],
			software_interrupts_cause_reg: [
				(data & 0x0000_0100) != 0,
				(data & 0x0000_0200) != 0
			]
		}
	}
}

#[derive(Debug)]
enum Mode {
	Kernel,
	Supervisor,
	User
}

impl Default for Mode {
	fn default() -> Mode {
		Mode::Kernel
	}
}

impl From<u32> for Mode {
	fn from(data: u32) -> Mode {
		match data & 0x0000_0018 {
			0b00 => Mode::Kernel,
			0b01 => Mode::Supervisor,
			0b10 => Mode::User,
			_ => panic!("Invalid cp0 KSU bits: {:#b}", data)
		}
	}
}