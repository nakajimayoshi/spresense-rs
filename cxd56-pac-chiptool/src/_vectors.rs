unsafe extern "C" {
    fn CRG();
    fn UART1();
    fn SPI3();
    fn I2C0();
    fn I2C1();
    fn EXDEVICE_0();
    fn EXDEVICE_1();
    fn EXDEVICE_2();
    fn EXDEVICE_3();
    fn EXDEVICE_4();
    fn EXDEVICE_5();
    fn EXDEVICE_6();
    fn EXDEVICE_7();
    fn EXDEVICE_8();
    fn EXDEVICE_9();
    fn EXDEVICE_10();
    fn EXDEVICE_11();
    fn SPI0();
    fn FIFO_TO();
    fn FIFO_FROM();
    fn SPH0();
    fn SPH1();
    fn SPH2();
    fn SPH3();
    fn SPH4();
    fn SPH5();
    fn SPH6();
    fn SPH7();
    fn SPH8();
    fn SPH9();
    fn SPH10();
    fn SPH11();
    fn SPH12();
    fn SPH13();
    fn SPH14();
    fn SPH15();
    fn GE2D();
    fn ROT();
    fn CISIF();
    fn SPI5();
    fn DMAC3();
    fn UART2();
    fn SPI4();
    fn DMAC1();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 119] = [
    Vector { _reserved: 0 },
    Vector { _handler: CRG },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: UART1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI3 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _reserved: 0 },
    Vector {
        _handler: EXDEVICE_0,
    },
    Vector {
        _handler: EXDEVICE_1,
    },
    Vector {
        _handler: EXDEVICE_2,
    },
    Vector {
        _handler: EXDEVICE_3,
    },
    Vector {
        _handler: EXDEVICE_4,
    },
    Vector {
        _handler: EXDEVICE_5,
    },
    Vector {
        _handler: EXDEVICE_6,
    },
    Vector {
        _handler: EXDEVICE_7,
    },
    Vector {
        _handler: EXDEVICE_8,
    },
    Vector {
        _handler: EXDEVICE_9,
    },
    Vector {
        _handler: EXDEVICE_10,
    },
    Vector {
        _handler: EXDEVICE_11,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FIFO_TO },
    Vector {
        _handler: FIFO_FROM,
    },
    Vector { _handler: SPH0 },
    Vector { _handler: SPH1 },
    Vector { _handler: SPH2 },
    Vector { _handler: SPH3 },
    Vector { _handler: SPH4 },
    Vector { _handler: SPH5 },
    Vector { _handler: SPH6 },
    Vector { _handler: SPH7 },
    Vector { _handler: SPH8 },
    Vector { _handler: SPH9 },
    Vector { _handler: SPH10 },
    Vector { _handler: SPH11 },
    Vector { _handler: SPH12 },
    Vector { _handler: SPH13 },
    Vector { _handler: SPH14 },
    Vector { _handler: SPH15 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GE2D },
    Vector { _handler: ROT },
    Vector { _handler: CISIF },
    Vector { _handler: SPI5 },
    Vector { _handler: DMAC3 },
    Vector { _handler: UART2 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI4 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DMAC1 },
];
