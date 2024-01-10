//! This module contains initialization code, run once at program start.

use bleps::{
    ad_structure::{
        create_advertising_data, AdStructure, BR_EDR_NOT_SUPPORTED, LE_GENERAL_DISCOVERABLE,
    },
    att::Uuid,
    attribute_server::{AttributeServer, NotificationData, WorkResult},
    gatt, Ble, HciConnector,
};
use embedded_io::*;
use embedded_svc::{
    ipv4::Interface,
    wifi::{AccessPointConfiguration, AccessPointInfo, ClientConfiguration, Configuration, Wifi},
};
// todo: defmt leads to link errors here, but not elsewhere(?)
// use defmt::println;  // todo: After next HAL release.
use esp_println::println;
use esp_wifi::{
    ble::controller::BleConnector,
    current_millis, initialize,
    wifi::{utils::create_network_interface, WifiApDevice, WifiError, WifiStaDevice},
    wifi_interface::WifiStack,
    EspWifiInitFor, EspWifiInitialization,
};
use hal::{
    clock::ClockControl, peripherals::Peripherals, prelude::*, systimer::SystemTimer, Delay, Rng,
};
use heapless::Vec;
use smoltcp::iface::SocketStorage;

use crate::setup;

const SSID: &str = "temp";
const PASSWORD: &str = "temp";

const MAX_NUM_APS: usize = 20;

fn parse_ip(ip: &str) -> [u8; 4] {
    let mut result = [0u8; 4];
    for (idx, octet) in ip.split(".").into_iter().enumerate() {
        result[idx] = u8::from_str_radix(octet, 10).unwrap();
    }
    result
}

// fn wifi_ap_test(peripherals: &Peripherals, init: &EspWifiInitialization) {
// fn wifi_ap_test(peripherals: Peripherals, init: &EspWifiInitialization) {
fn wifi_ap_test(init: &EspWifiInitialization) {
    let peripherals = unsafe { Peripherals::steal() };

    let wifi = peripherals.WIFI;
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let (iface, device, mut controller, sockets) =
        create_network_interface(&init, wifi, WifiApDevice, &mut socket_set_entries).unwrap();
    let mut wifi_stack = WifiStack::new(iface, device, sockets, current_millis);

    let client_config = Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "esp-wifi".try_into().unwrap(),
        ..Default::default()
    });
    let res = controller.set_configuration(&client_config);
    println!("wifi_set_configuration returned {:?}", res);

    controller.start().unwrap();
    println!("is wifi started: {:?}", controller.is_started());

    println!("{:?}", controller.get_capabilities());

    wifi_stack
        .set_iface_configuration(&embedded_svc::ipv4::Configuration::Client(
            embedded_svc::ipv4::ClientConfiguration::Fixed(embedded_svc::ipv4::ClientSettings {
                ip: embedded_svc::ipv4::Ipv4Addr::from(parse_ip("192.168.2.1")),
                subnet: embedded_svc::ipv4::Subnet {
                    gateway: embedded_svc::ipv4::Ipv4Addr::from(parse_ip("192.168.2.1")),
                    mask: embedded_svc::ipv4::Mask(24),
                },
                dns: None,
                secondary_dns: None,
            }),
        ))
        .unwrap();

    println!("Start busy loop on main. Connect to the AP `esp-wifi` and point your browser to http://192.168.2.1:8080/");
    println!("Use a static IP in the range 192.168.2.2 .. 192.168.2.255, use gateway 192.168.2.1");
}

/// List nearby Wi-Fi access points.
pub fn list_aps(
    init: &EspWifiInitialization,
) -> Result<(Vec<AccessPointInfo, MAX_NUM_APS>, usize), WifiError> {
    let peripherals = unsafe { Peripherals::steal() };
    let wifi = peripherals.WIFI;

    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let (iface, device, mut controller, sockets) =
        create_network_interface(&init, wifi, WifiStaDevice, &mut socket_set_entries).unwrap();

    let wifi_stack = WifiStack::new(iface, device, sockets, current_millis);

    let client_config = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        password: PASSWORD.try_into().unwrap(),
        ..Default::default()
    });

    let res = controller.set_configuration(&client_config);
    println!("wifi_set_configuration returned {:?}", res);

    controller.start().unwrap();
    println!("Wifi started: {:?}", controller.is_started());

    println!("Starting Wifi Scan...");
    let res: Result<(Vec<AccessPointInfo, MAX_NUM_APS>, usize), WifiError> = controller.scan_n();

    println!("{:?}", controller.get_capabilities());
    // println!("wifi_connect {:?}", controller.connect());

    res
}

/// List nearby bluetooth devices
pub fn list_bluetooth(init: &EspWifiInitialization) {
    let peripherals = unsafe { Peripherals::steal() };
    let bluetooth = peripherals.BT;

    let connector = BleConnector::new(&init, bluetooth);
    let hci = HciConnector::new(connector, esp_wifi::current_millis);
    let mut ble = Ble::new(&hci);

    ble.cmd_set_le_scan_rsp_data(
        // todo temp
        create_advertising_data(&[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[Uuid::Uuid16(0x1809)]),
            // AdStructure::CompleteLocalName(examples_util::SOC_NAME),
            AdStructure::CompleteLocalName("TEST"),
        ])
        .unwrap(),
    )
    .unwrap();

    println!("BLE init: {:?}", ble.init());
}

pub fn run() {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    println!("Hello world!");
    let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;

    setup::setup(&clocks);

    let init = initialize(
        // EspWifiInitFor::Wifi,
        EspWifiInitFor::WifiBle,
        timer,
        Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    // wifi_ap_test(peripherals, &init);
    // wifi_ap_test(&init);

    loop {
        let aps = list_aps(&init);
        let btle_devices = list_bluetooth(&init);

        println!("\nAPs:");
        if let Ok((res, _count)) = aps {
            for ap in res {
                println!("{:?}", ap);
            }
        }

        delay.delay_ms(5_000u32);
    }
}
