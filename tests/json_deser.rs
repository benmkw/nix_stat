//! Tests for JSON deserialization of sensors, tailscale status, and ip addr outputs.

use nix_stat::{CpuStat, DiskInfo, IpAddrInfo, TailscaleStatus};

#[test]
fn test_tailscale_status_with_peer_and_user() {
    let json = include_str!("./tailscale_status_peer_user.json");
    let status: TailscaleStatus = serde_json::from_str(json).expect("valid json");
    // Check Peer
    let peer_map = status.peer.as_ref().expect("peer map");
    assert!(peer_map.len() > 0);
    // let user_map = status.user.as_ref().expect("user map");
    // assert!(user_map.len() > 0);
    // Check a known peer
    let peer = peer_map
        .values()
        .find(|p| p.host_name == "redacted-peer-host3")
        .expect("redacted-peer-host3 peer");
    assert_eq!(peer.host_name, "redacted-peer-host3");
    // Accept both string and array for PeerAPIURL
    // if let Some(urls) = &peer.peer_api_url {
    //     let _urls: Vec<String> = urls.as_vec();
    // }
    // Check addrs (may be None)
    // if let Some(addrs) = &peer.addrs {
    //     assert!(addrs.iter().any(|a| a.contains(":")) || addrs.is_empty());
    // }
    // Check a known user
    // let user = user_map
    //     .values()
    //     .find(|u| u.login_name == "redacted@example.com")
    //     .expect("redacted@example.com user");
    // assert_eq!(user.display_name, "Redacted User");
}

#[test]
fn test_tailscale_status_json_deserialize() {
    let json = r#"{
    "Version": "1.84.3",
    "TUN": true,
    "BackendState": "Running",
    "HaveNodeKey": true,
    "AuthURL": "",
    "TailscaleIPs": ["100.73.252.58", "fd7a:115c:a1e0::c436:fc3a"],
    "Self": {
        "ID": "aaa-aaa",
        "PublicKey": "nodekey:aaa-aaa",
        "HostName": "test-host",
        "DNSName": "test-host.tailnet.",
        "OS": "linux",
        "UserID": 123456789,
        "TailscaleIPs": ["100.73.252.58", "fd7a:115c:a1e0::c436:fc3a"],
        "AllowedIPs": ["100.73.252.58/32", "fd7a:115c:a1e0::c436:fc3a/128", "0.0.0.0/0", "::/0"],
        "Addrs": ["1.2.3.4:5678"],
        "CurAddr": "",
        "Relay": "fra",
        "RxBytes": 0,
        "TxBytes": 0,
        "Created": "2025-07-27T15:27:30.652585519Z",
        "LastWrite": "0001-01-01T00:00:00Z",
        "LastSeen": "0001-01-01T00:00:00Z",
        "LastHandshake": "0001-01-01T00:00:00Z",
        "Online": true,
        "ExitNode": false,
        "ExitNodeOption": true,
        "Active": false,
        "PeerAPIURL": ["http://100.73.252.58:39006"],
        "TaildropTarget": 0,
        "NoFileSharingReason": "",
        "Capabilities": ["cap1", "cap2"],
        "CapMap": null,
        "InNetworkMap": true,
        "InMagicSock": false,
        "InEngine": false
    },
    "Health": [],
    "MagicDNSSuffix": "tailnet",
    "CurrentTailnet": {
        "Name": "test@test.com",
        "MagicDNSSuffix": "tailnet",
        "MagicDNSEnabled": false
    },
    "CertDomains": null
}
"#;
    let _status: TailscaleStatus = serde_json::from_str(json).expect("valid json");
    // assert_eq!(status.version, "1.84.3");
    // assert_eq!(status.tun, true);
    // assert_eq!(status.backend_state, "Running");
    // assert_eq!(status.self_field.host_name, "test-host");
    // assert_eq!(status.self_field.tailscale_ips[0], "100.73.252.58");
    // assert_eq!(status.self_field.online, true);
    // assert_eq!(status.current_tailnet.name, "test@test.com");
    // Check addrs (should be Some and non-empty)
    // assert!(status.self_field.addrs.as_ref().unwrap().len() > 0);
}

#[test]
fn test_ip_addr_json_deserialize() {
    let json = r#"[
  {"ifindex":1,"ifname":"lo","flags":["LOOPBACK","UP","LOWER_UP"],"mtu":65536,"qdisc":"noqueue","operstate":"UNKNOWN","group":"default","txqlen":1000,"link_type":"loopback","address":"00:00:00:00:00:00","broadcast":"00:00:00:00:00:00","addr_info":[{"family":"inet","local":"127.0.0.1","prefixlen":8,"scope":"host","label":"lo","valid_life_time":4294967295,"preferred_life_time":4294967295},{"family":"inet6","local":"::1","prefixlen":128,"scope":"host","noprefixroute":true,"valid_life_time":4294967295,"preferred_life_time":4294967295}]},
  {"ifindex":2,"ifname":"end0","flags":["NO-CARRIER","BROADCAST","MULTICAST","UP"],"mtu":1500,"qdisc":"mq","operstate":"DOWN","group":"default","txqlen":1000,"link_type":"ether","address":"4c:20:b8:a8:35:98","broadcast":"ff:ff:ff:ff:ff:ff","altnames":["enp3s0","enx4c20b8a83598"],"addr_info":[]},
  {"ifindex":3,"ifname":"wlan0","flags":["BROADCAST","MULTICAST","UP","LOWER_UP"],"mtu":1500,"qdisc":"fq_codel","operstate":"UP","group":"default","txqlen":1000,"link_type":"ether","address":"4c:20:b8:a8:10:5f","broadcast":"ff:ff:ff:ff:ff:ff","addr_info":[{"family":"inet","local":"192.168.2.230","prefixlen":24,"broadcast":"192.168.2.255","scope":"global","dynamic":true,"noprefixroute":true,"label":"wlan0","valid_life_time":1791916,"preferred_life_time":1565116},{"family":"inet","local":"169.254.11.146","prefixlen":16,"broadcast":"169.254.255.255","scope":"global","noprefixroute":true,"label":"wlan0","valid_life_time":4294967295,"preferred_life_time":4294967295},{"family":"inet6","local":"2003:df:174b:85e7:9132:9421:5943:ae00","prefixlen":64,"scope":"global","temporary":true,"dynamic":true,"valid_life_time":172789,"preferred_life_time":70204},{"family":"inet6","local":"2003:df:174b:85e7:4e20:b8ff:fea8:105f","prefixlen":64,"scope":"global","dynamic":true,"mngtmpaddr":true,"noprefixroute":true,"valid_life_time":172789,"preferred_life_time":86389},{"family":"inet6","local":"2003:df:174b:8579:4e20:b8ff:fea8:105f","prefixlen":128,"scope":"global","deprecated":true,"dynamic":true,"noprefixroute":true,"valid_life_time":5793,"preferred_life_time":0},{"family":"inet6","local":"fe80::4e20:b8ff:fea8:105f","prefixlen":64,"scope":"link","valid_life_time":4294967295,"preferred_life_time":4294967295}]},
  {"ifindex":5,"ifname":"tailscale0","flags":["POINTOPOINT","MULTICAST","NOARP","UP","LOWER_UP"],"mtu":1280,"qdisc":"fq_codel","operstate":"UNKNOWN","group":"default","txqlen":500,"link_type":"none","addr_info":[{"family":"inet","local":"100.73.252.58","prefixlen":32,"scope":"global","label":"tailscale0","valid_life_time":4294967295,"preferred_life_time":4294967295},{"family":"inet6","local":"fd7a:115c:a1e0::c436:fc3a","prefixlen":128,"scope":"global","valid_life_time":4294967295,"preferred_life_time":4294967295},{"family":"inet6","local":"fe80::60a:41be:46de:248c","prefixlen":64,"scope":"link","stable-privacy":true,"protocol":"kernel_ll","valid_life_time":4294967295,"preferred_life_time":4294967295}]}]
"#;
    let addrs: Vec<IpAddrInfo> = serde_json::from_str(json).expect("valid json");
    assert_eq!(addrs[0].ifname, "lo");
    assert_eq!(addrs[2].ifname, "wlan0");
    assert_eq!(addrs[2].operstate, "UP");
    assert_eq!(addrs[2].addr_info[0].local, "192.168.2.230");
    assert_eq!(addrs[3].ifname, "tailscale0");
    assert_eq!(addrs[3].addr_info[0].local, "100.73.252.58");
}

#[test]
fn test_uptime_parse() {
    let sample = " 20:43:17  up  22:52,  1 user,  load average: 0.06, 0.01, 0.00
";
    // Simple parse: extract the 'up' field
    let up = sample.split(" up ").nth(1).unwrap();
    let up = up.split(',').next().unwrap().trim();
    assert_eq!(up, "22:52");
}

#[test]
fn test_disk_info_parse() {
    let sample = r#"Filesystem      Size  Used Avail Use% Mounted on
devtmpfs        380M     0  380M   0% /dev
tmpfs           3.8G     0  3.8G   0% /dev/shm
tmpfs           1.9G   20M  1.9G   2% /run
/dev/nvme0n1p5   75G   16G   56G  22% /
/dev/nvme0n1p4  476M  189M  288M  40% /boot
tmpfs           3.8G  2.9M  3.8G   1% /run/wrappers
tmpfs           1.0M     0  1.0M   0% /run/credentials/getty@tty1.service
tmpfs           1.0M     0  1.0M   0% /run/credentials/systemd-journald.service
tmpfs           760M   16K  760M   1% /run/user/1000
"#;
    let lines: Vec<_> = sample.lines().skip(1).collect();
    let disks: Vec<DiskInfo> = lines
        .iter()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            DiskInfo {
                filesystem: parts[0].to_string(),
                size: parts[1].to_string(),
                used: parts[2].to_string(),
                avail: parts[3].to_string(),
                use_perc: parts[4].to_string(),
                mount: parts[5].to_string(),
                total: parts[1].to_string(),
                percentage: parts[4].trim_end_matches('%').parse().unwrap(),
            }
        })
        .collect();
    assert!(
        disks
            .iter()
            .any(|d| d.mount == "/" && d.size == "75G" && d.used == "16G")
    );
}

#[test]
fn test_ram_info_parse() {
    let sample = r#"MemTotal:        7779296 kB
MemFree:         2801056 kB
MemAvailable:    7314368 kB
Buffers:          937472 kB
Cached:          3239904 kB
SwapCached:            0 kB
Active:          2008768 kB
Inactive:        2379088 kB
Active(anon):       2368 kB
Inactive(anon):   233280 kB
Active(file):    2006400 kB
Inactive(file):  2145808 kB
Unevictable:        2144 kB
Mlocked:               0 kB
SwapTotal:             0 kB
SwapFree:              0 kB
Zswap:                 0 kB
Zswapped:              0 kB
Dirty:              1296 kB
Writeback:             0 kB
AnonPages:        212720 kB
Mapped:           122704 kB
Shmem:             25184 kB
KReclaimable:     426576 kB
Slab:             507936 kB
SReclaimable:     426576 kB
SUnreclaim:        81360 kB
KernelStack:        6848 kB
PageTables:         4128 kB
SecPageTables:       416 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:     3889648 kB
Committed_AS:    1127824 kB
VmallocTotal:   136898928640 kB
VmallocUsed:       24352 kB
VmallocChunk:          0 kB
Percpu:             3072 kB
CmaTotal:          65536 kB
CmaFree:           61408 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:      32768 kB
Hugetlb:               0 kB
"#;
    let ram = nix_stat::get_ram_info_from_str(sample).expect("valid ram info string");
    assert_eq!(ram.total, "7.42 GB");
    assert_eq!(ram.used, "0.44 GB");
    assert_eq!(ram.free, "2.67 GB");
    assert_eq!(ram.shared, "0.02 GB");
    assert_eq!(ram.buff_cache, "3.98 GB");
    assert_eq!(ram.available, "6.98 GB");
    assert_eq!(ram.percentage, 6.0f64);
}

#[test]
fn test_cpu_stat_parse() {
    let sample = r#"cpu  625969 24 122187 65096957 6053 23493 16192 0 0 0
cpu0 72382 1 41749 8102273 1666 8531 11492 0 0 0
cpu1 84123 0 21752 8119593 1777 4994 1577 0 0 0
cpu2 77322 0 14883 8138002 1139 3108 949 0 0 0
cpu3 74525 0 11156 8146970 477 2279 715 0 0 0
cpu4 88192 10 10377 8135714 255 1564 444 0 0 0
cpu5 80679 4 8023 8145831 269 1327 474 0 0 0
cpu6 76275 5 7533 8151843 234 923 258 0 0 0
cpu7 72467 1 6710 8156728 232 765 279 0 0 0
"#;

    let stats: Vec<CpuStat> = sample
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            CpuStat {
                name: parts[0].to_string(),
                user: parts[1].parse().unwrap(),
                nice: parts[2].parse().unwrap(),
                system: parts[3].parse().unwrap(),
                idle: parts[4].parse().unwrap(),
                iowait: parts[5].parse().unwrap(),
                irq: parts[6].parse().unwrap(),
                softirq: parts[7].parse().unwrap(),
                steal: parts[8].parse().unwrap(),
                guest: parts[9].parse().unwrap(),
                guest_nice: parts[10].parse().unwrap(),
            }
        })
        .collect();
    assert!(stats.iter().any(|c| c.name == "cpu0"));
    assert_eq!(stats[0].name, "cpu");
    assert_eq!(stats[1].name, "cpu0");
}

#[test]
fn test_sensors_json_deserialize() {
    // Example output from `sensors -j` (simplified, real output may have more fields)
    let json = r#"{
      "macsmc_hwmon-isa-0000": {
        "Adapter": "ISA adapter",
        "AC Input Voltage": {"in0_input": 12.01},
        "Fan": {"fan1_input": 2000.0, "fan1_min": 1200.0, "fan1_max": 6000.0},
        "NAND Flash Temperature": {"temp1_input": 38.0},
        "WiFi/BT Module Temp": {"temp1_input": 40.0},
        "Total System Power": {"power1_input": 25.0},
        "AC Input Power": {"power1_input": 30.0},
        "3.8 V Rail Power": {"power1_input": 5.0},
        "AC Input Current": {"curr1_input": 2.5}
      }
    }
"#;
    use serde_json::Value;
    let sensors: Value = serde_json::from_str(json).expect("valid sensors json");

    // Test that we can parse the macsmc device
    if let Value::Object(devices) = &sensors {
        let macsmc_device = devices.get("macsmc_hwmon-isa-0000").expect("macsmc device");
        if let Ok(macsmc) = serde_json::from_value::<nix_stat::Macsmc>(macsmc_device.clone()) {
            assert_eq!(macsmc.adapter, "ISA adapter");
            assert!(macsmc.ac_input_voltage.is_some());
            assert!(macsmc.fan.is_some());
            assert!(macsmc.nand_flash_temperature.is_some());
            assert!(macsmc.wifi_bt_module_temp.is_some());
            assert!(macsmc.total_system_power.is_some());
            assert!(macsmc.ac_input_power.is_some());
            assert!(macsmc.rail_power.is_some());
            assert!(macsmc.ac_input_current.is_some());
            // Check a value
            assert_eq!(macsmc.fan.as_ref().unwrap().fan1_input, 2000.0);
        } else {
            panic!("Failed to parse macsmc device");
        }
    } else {
        panic!("Expected sensors to be an object");
    }
}

#[tokio::test]
async fn test_load_avg_parse() {
    let sample = "0.13 0.50 0.36 1/217 688199";
    let load_avg = nix_stat::get_load_avg_from_str(sample).expect("valid loadavg string");
    assert_eq!(load_avg.one_min, 0.13);
    assert_eq!(load_avg.five_min, 0.50);
    assert_eq!(load_avg.fifteen_min, 0.36);
    assert_eq!(load_avg.runnable_entities, 1);
    assert_eq!(load_avg.total_processes, 217);
}

#[test]
fn test_net_dev_info_parse() {
    let sample = r#"Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo:  622501    3473    0    0    0     0          0         0   622501    3473    0    0    0     0       0          0
  end0:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0
 wlan0: 5336190684 4671814    0    0    0     0          0     35515 336233620 2199943    0    4    0     0       0          0
tailscale0: 3332329   52567    0    0    0     0          0         0 42578956   47327    0    0    0     0       0          0
"#;
    let net_dev_info = nix_stat::get_net_dev_info_from_str(sample).expect("valid net_dev string");
    assert_eq!(net_dev_info.len(), 4);
    assert_eq!(net_dev_info[0].interface, "lo");
    assert_eq!(net_dev_info[0].rx_bytes, 622501);
    assert_eq!(net_dev_info[0].tx_bytes, 622501);
    assert_eq!(net_dev_info[2].interface, "wlan0");
    assert_eq!(net_dev_info[2].rx_bytes, 5336190684);
    assert_eq!(net_dev_info[2].tx_bytes, 336233620);
}

#[test]
fn test_tcp_connections_parse() {
    let sample = r#"State              Recv-Q              Send-Q                                                       Local Address:Port                                                           Peer Address:Port
ESTAB              0                   0                                                            192.168.2.230:42270                                                               8.8.4.4:443
ESTAB              0                   0                                                            192.168.2.230:52758                                                          162.159.61.8:443
ESTAB              0                   68                                                           192.168.2.230:22                                                            192.168.2.204:52970
ESTAB              0                   0                                                            100.73.252.58:8000                                                           100.85.63.23:54004
ESTAB              0                   0                                  [2003:df:1712:4eb4:5010:36b4:39f5:ade2]:53620                                                   [2606:b740:49::115]:80
"#;
    let connections =
        nix_stat::get_tcp_connections_from_str(sample).expect("valid tcp connections string");
    assert_eq!(connections.len(), 5);
    assert_eq!(connections[0].state, "ESTAB");
    assert_eq!(connections[0].local_address, "192.168.2.230:42270");
    assert_eq!(connections[0].peer_address, "8.8.4.4:443");
    assert_eq!(connections[3].local_address, "100.73.252.58:8000");
    assert_eq!(
        connections[4].local_address,
        "[2003:df:1712:4eb4:5010:36b4:39f5:ade2]:53620"
    );
}

#[test]
fn test_uptime_info_parse() {
    let sample = "135951.86 1076503.01";
    let uptime_info = nix_stat::get_uptime_info_from_str(sample).expect("valid uptime string");
    assert_eq!(uptime_info.total_uptime_seconds, 135951.86);
    assert_eq!(uptime_info.idle_time_seconds, 1076503.01);
    assert_eq!(uptime_info.formatted_uptime, "1d 13h 45m");
}

#[test]
fn test_services_parse() {
    let sample = r#"adguardhome.service       loaded active running AdGuard Home: Network-level blocker
dbus.service              loaded active running D-Bus System Message Bus
dhcpcd.service            loaded active running DHCP Client
getty@tty1.service        loaded active running Getty on tty1
iwd.service               loaded active running Wireless service
nix-daemon.service        loaded active running Nix Daemon
nix-stat.service          loaded active running Nix Stat Dashboard
nscd.service              loaded active running Name Service Cache Daemon (nsncd)
polkit.service            loaded active running Authorization Manager
speakersafetyd.service    loaded active running Speaker Protection Daemon
sshd.service              loaded active running SSH Daemon
systemd-journald.service  loaded active running Journal Service
systemd-logind.service    loaded active running User Login Management
systemd-oomd.service      loaded active running Userspace Out-Of-Memory (OOM) Killer
systemd-timesyncd.service loaded active running Network Time Synchronization
systemd-udevd.service     loaded active running Rule-based Manager for Device Events and Files
tailscaled.service        loaded active running Tailscale node agent
user@1000.service         loaded active running User Manager for UID 1000
"#;
    let services = nix_stat::get_services_from_str(sample).expect("valid services string");
    assert_eq!(services.len(), 18);
    assert_eq!(services[0].unit, "adguardhome.service");
    assert_eq!(
        services[0].description,
        "AdGuard Home: Network-level blocker"
    );
    assert_eq!(services[6].unit, "nix-stat.service");
    assert_eq!(services[6].description, "Nix Stat Dashboard");
    assert_eq!(services[17].unit, "user@1000.service");
    assert_eq!(services[17].description, "User Manager for UID 1000");
}

#[test]
fn test_cgroup_data_parse() {
    let sample = r#"CGroup                                                                                                                                                                    Tasks   %CPU   Memory  Input/s Output/s
/                                                                                                                                                                           214      -     472M        -        -
dev-hugepages.mount                                                                                                                                                           -      -      16K        -        -
dev-mqueue.mount                                                                                                                                                              -      -      16K        -        -
init.scope                                                                                                                                                                    1      -    19.1M        -        -
sys-fs-fuse-connections.mount                                                                                                                                                 -      -      16K        -        -
sys-kernel-config.mount                                                                                                                                                       -      -      16K        -        -
sys-kernel-debug.mount                                                                                                                                                        -      -      16K        -        -
system.slice                                                                                                                                                                 67      -     1.2G        -        -
system.slice/adguardhome.service                                                                                                                                             14      -   156.1M        -        -
system.slice/boot.mount                                                                                                                                                       -      -      16K        -        -
system.slice/dbus.service                                                                                                                                                     1      -     5.4M        -        -
system.slice/dhcpcd.service                                                                                                                                                   1      -     4.4M        -        -
system.slice/iwd.service                                                                                                                                                      1      -     3.4M        -        -
system.slice/nix-daemon.service                                                                                                                                               2      -   726.1M        -        -
system.slice/nix-stat.service                                                                                                                                                 9      -     4.9M        -        -
system.slice/nscd.service                                                                                                                                                    11      -     6.2M        -        -
system.slice/polkit.service                                                                                                                                                   4      -     2.4M        -        -
system.slice/run-wrappers.mount                                                                                                                                               -      -      16K        -        -
system.slice/speakersafetyd.service                                                                                                                                           1      -     672K        -        -
system.slice/sshd.service                                                                                                                                                     1      -     9.3M        -        -
system.slice/system-getty.slice                                                                                                                                               1      -     576K        -        -
system.slice/system-getty.slice/getty@tty1.service                                                                                                                            1      -     544K        -        -
system.slice/system-modprobe.slice                                                                                                                                            -      -     480K        -        -
system.slice/systemd-journald.service                                                                                                                                         1      -    57.7M        -        -
system.slice/systemd-logind.service                                                                                                                                           1      -     8.3M        -        -
system.slice/systemd-oomd.service                                                                                                                                             1      -     2.2M        -        -
system.slice/systemd-timesyncd.service                                                                                                                                        2      -     2.4M        -        -
system.slice/systemd-udevd.service                                                                                                                                            1      -       6M        -        -
system.slice/tailscaled.service                                                                                                                                              15      -    48.7M        -        -
user.slice                                                                                                                                                                    7      -     2.5G        -        -
user.slice/user-0.slice                                                                                                                                                       -      -     4.2M        -        -
user.slice/user-1000.slice                                                                                                                                                    7      -     2.5G        -        -
user.slice/user-1000.slice/session-69.scope                                                                                                                                   4      -   593.1M        -        -
user.slice/user-1000.slice/user@1000.service                                                                                                                                  3      -     6.7M        -        -
"#;
    let cgroup_data = nix_stat::get_cgroup_data_from_str(sample).expect("valid cgroup data string");
    assert_eq!(cgroup_data.len(), 34);

    let cgroup_root = cgroup_data
        .iter()
        .find(|c| c.path == "/")
        .expect("root cgroup not found");
    assert_eq!(cgroup_root.tasks, "214");
    assert_eq!(cgroup_root.cpu, "-");
    assert_eq!(cgroup_root.memory, "472M");

    let cgroup_adguardhome = cgroup_data
        .iter()
        .find(|c| c.path == "system.slice/adguardhome.service")
        .expect("adguardhome cgroup not found");
    assert_eq!(cgroup_adguardhome.tasks, "14");
    assert_eq!(cgroup_adguardhome.cpu, "-");
    assert_eq!(cgroup_adguardhome.memory, "156.1M");

    let cgroup_user = cgroup_data
        .iter()
        .find(|c| c.path == "user.slice/user-1000.slice/user@1000.service")
        .expect("user cgroup not found");
    assert_eq!(cgroup_user.tasks, "3");
    assert_eq!(cgroup_user.cpu, "-");
    assert_eq!(cgroup_user.memory, "6.7M");
}
