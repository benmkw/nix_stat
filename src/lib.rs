use anyhow::{Context, Result};
use jiff::Zoned;
use serde::{Deserialize, Serialize};
use xshell::{Shell, cmd};

const SECTOR_SIZE: f64 = 512.0;
const MB_IN_BYTES: f64 = 1024.0 * 1024.0;
const KIB_TO_GIB: f64 = 1024.0 * 1024.0;

// Custom serializer for Zoned timestamp to make it JavaScript Date compatible
fn serialize_timestamp<S>(timestamp: &Zoned, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Convert to RFC 3339 format which is JavaScript Date compatible
    serializer.serialize_str(&timestamp.timestamp().to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(untagged)]
pub enum StringOrVec {
    Str(String),
    Vec(Vec<String>),
}

impl StringOrVec {
    pub fn into_vec(self) -> Vec<String> {
        match self {
            StringOrVec::Str(s) => vec![s],
            StringOrVec::Vec(v) => v,
        }
    }
    pub fn as_vec(&self) -> Vec<String> {
        match self {
            StringOrVec::Str(s) => vec![s.clone()],
            StringOrVec::Vec(v) => v.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TailscaleStatus {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "TUN")]
    pub tun: bool,
    #[serde(rename = "BackendState")]
    pub backend_state: String,
    #[serde(rename = "HaveNodeKey")]
    pub have_node_key: bool,
    #[serde(rename = "AuthURL")]
    pub auth_url: String,
    #[serde(rename = "TailscaleIPs")]
    pub tailscale_ips: Vec<String>,
    #[serde(rename = "Self")]
    pub self_field: TailscaleSelf,
    #[serde(rename = "Health")]
    pub health: Vec<serde_json::Value>,
    #[serde(rename = "MagicDNSSuffix")]
    pub magic_dnssuffix: String,
    #[serde(rename = "CurrentTailnet")]
    pub current_tailnet: Tailnet,
    #[serde(rename = "CertDomains")]
    pub cert_domains: Option<serde_json::Value>,
    #[serde(rename = "Peer")]
    pub peer: Option<std::collections::HashMap<String, TailscalePeerJson>>,
    #[serde(rename = "User")]
    pub user: Option<std::collections::HashMap<String, TailscaleUser>>,
    #[serde(rename = "ClientVersion")]
    pub client_version: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct TailscaleSelf {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "PublicKey")]
    pub public_key: String,
    #[serde(rename = "HostName")]
    pub host_name: String,
    #[serde(rename = "DNSName")]
    pub dns_name: String,
    #[serde(rename = "OS")]
    pub os: String,
    #[serde(rename = "UserID")]
    pub user_id: u64,
    #[serde(rename = "TailscaleIPs")]
    pub tailscale_ips: Vec<String>,
    #[serde(rename = "AllowedIPs")]
    pub allowed_ips: Vec<String>,
    #[serde(rename = "Addrs")]
    pub addrs: Option<Vec<String>>,
    #[serde(rename = "Relay")]
    pub relay: String,
    #[serde(rename = "RxBytes")]
    pub rx_bytes: u64,
    #[serde(rename = "TxBytes")]
    pub tx_bytes: u64,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "LastWrite")]
    pub last_write: String,
    #[serde(rename = "LastSeen")]
    pub last_seen: String,
    #[serde(rename = "LastHandshake")]
    pub last_handshake: String,
    #[serde(rename = "Online")]
    pub online: bool,
    #[serde(rename = "ExitNode")]
    pub exit_node: bool,
    #[serde(rename = "ExitNodeOption")]
    pub exit_node_option: bool,
    #[serde(rename = "Active")]
    pub active: bool,
    #[serde(rename = "TaildropTarget")]
    pub taildrop_target: Option<u64>,
    #[serde(rename = "NoFileSharingReason")]
    pub no_file_sharing_reason: String,
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<Vec<String>>,
    #[serde(rename = "CapMap")]
    pub cap_map: Option<serde_json::Value>,
    #[serde(rename = "InNetworkMap")]
    pub in_network_map: bool,
    #[serde(rename = "InMagicSock")]
    pub in_magic_sock: bool,
    #[serde(rename = "InEngine")]
    pub in_engine: bool,
}
#[derive(Debug, Deserialize)]
pub struct Tailnet {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MagicDNSSuffix")]
    pub magic_dnssuffix: String,
    #[serde(rename = "MagicDNSEnabled")]
    pub magic_dns_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct TailscalePeerJson {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "PublicKey")]
    pub public_key: String,
    #[serde(rename = "HostName")]
    pub host_name: String,
    #[serde(rename = "DNSName")]
    pub dns_name: String,
    #[serde(rename = "PeerAPIURL")]
    pub peer_api_url: Option<StringOrVec>,
    #[serde(rename = "UserID")]
    pub user_id: u64,
    #[serde(rename = "TailscaleIPs")]
    pub tailscale_ips: Vec<String>,
    #[serde(rename = "AllowedIPs")]
    pub allowed_ips: Vec<String>,
    #[serde(rename = "Addrs")]
    pub addrs: Option<Vec<String>>,
    #[serde(rename = "Relay")]
    pub relay: String,
    #[serde(rename = "RxBytes")]
    pub rx_bytes: u64,
    #[serde(rename = "TxBytes")]
    pub tx_bytes: u64,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "LastWrite")]
    pub last_write: String,
    #[serde(rename = "LastSeen")]
    pub last_seen: String,
    #[serde(rename = "LastHandshake")]
    pub last_handshake: String,
    #[serde(rename = "Online")]
    pub online: bool,
    #[serde(rename = "ExitNode")]
    pub exit_node: bool,
    #[serde(rename = "ExitNodeOption")]
    pub exit_node_option: bool,
    #[serde(rename = "Active")]
    pub active: bool,
    #[serde(rename = "TaildropTarget")]
    pub taildrop_target: Option<u64>,
    #[serde(rename = "NoFileSharingReason")]
    pub no_file_sharing_reason: String,
    #[serde(rename = "InNetworkMap")]
    pub in_network_map: bool,
    #[serde(rename = "InMagicSock")]
    pub in_magic_sock: bool,
    #[serde(rename = "InEngine")]
    pub in_engine: bool,
    #[serde(rename = "KeyExpiry")]
    pub key_expiry: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TailscaleUser {
    #[serde(rename = "ID")]
    pub id: u64,
    #[serde(rename = "LoginName")]
    pub login_name: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "ProfilePicURL")]
    pub profile_pic_url: String,
}

#[derive(Debug, Deserialize)]
pub struct IpAddrInfo {
    pub ifindex: u32,
    pub ifname: String,
    pub flags: Vec<String>,
    pub mtu: u32,
    pub qdisc: String,
    pub operstate: String,
    pub group: String,
    pub txqlen: u32,
    pub link_type: String,
    pub address: Option<String>,
    pub broadcast: Option<String>,
    pub altnames: Option<Vec<String>>,
    pub addr_info: Vec<IpAddrDetail>,
}
#[derive(Debug, Deserialize)]
pub struct IpAddrDetail {
    pub family: String,
    pub local: String,
    pub prefixlen: u32,
    pub broadcast: Option<String>,
    pub scope: String,
    pub label: Option<String>,
    pub valid_life_time: Option<u64>,
    pub preferred_life_time: Option<u64>,
    pub dynamic: Option<bool>,
    pub noprefixroute: Option<bool>,
    pub temporary: Option<bool>,
    pub mngtmpaddr: Option<bool>,
    pub deprecated: Option<bool>,
    pub stable_privacy: Option<bool>,
    pub protocol: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SensorsRoot {
    #[serde(rename = "macsmc_hwmon-isa-0000")]
    pub macsmc: Macsmc,
}

#[derive(Debug, Deserialize)]
pub struct Macsmc {
    #[serde(rename = "Adapter")]
    pub adapter: String,
    #[serde(rename = "AC Input Voltage")]
    pub ac_input_voltage: Option<InInput>,
    #[serde(rename = "Fan")]
    pub fan: Option<Fan>,
    #[serde(rename = "NAND Flash Temperature")]
    pub nand_flash_temperature: Option<TempInput>,
    #[serde(rename = "WiFi/BT Module Temp")]
    pub wifi_bt_module_temp: Option<TempInput>,
    #[serde(rename = "Total System Power")]
    pub total_system_power: Option<PowerInput>,
    #[serde(rename = "AC Input Power")]
    pub ac_input_power: Option<PowerInput>,
    #[serde(rename = "3.8 V Rail Power")]
    pub rail_power: Option<PowerInput>,
    #[serde(rename = "AC Input Current")]
    pub ac_input_current: Option<CurrInput>,
}

#[derive(Debug, Deserialize)]
pub struct InInput {
    pub in0_input: f64,
}
#[derive(Debug, Deserialize)]
pub struct Fan {
    pub fan1_input: f64,
    pub fan1_min: f64,
    pub fan1_max: f64,
}
#[derive(Debug, Deserialize)]
pub struct TempInput {
    #[serde(flatten)]
    pub temps: std::collections::HashMap<String, f64>,
}
#[derive(Debug, Deserialize)]
pub struct PowerInput {
    #[serde(flatten)]
    pub powers: std::collections::HashMap<String, f64>,
}
#[derive(Debug, Deserialize)]
pub struct CurrInput {
    #[serde(flatten)]
    pub currents: std::collections::HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiskStats {
    reads_completed: u64,
    sectors_read: u64,
    writes_completed: u64,
    sectors_written: u64,
    time_spent_io: u64,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct DiskIoInfo {
    pub device: String,
    pub read_iops: f64,
    pub write_iops: f64,
    pub read_mb_s: f64,
    pub write_mb_s: f64,
    pub utilization: f64,
    pub total_reads_completed: u64,
    pub total_sectors_read: u64,
    pub total_writes_completed: u64,
    pub total_sectors_written: u64,
    pub total_time_spent_io: u64,
}

pub fn parse_diskstats_from_str(
    input: &str,
) -> Result<std::collections::HashMap<String, DiskStats>> {
    let mut stats = std::collections::HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // Ensure we have at least the required fields (0-12 are the minimum we need)
        if parts.len() < 13 {
            continue;
        }

        let device = parts[2];

        // Filter out devices we don't want to track
        if device.starts_with("loop") || device.starts_with("sr") || device.starts_with("ram") {
            continue;
        }

        // Parse required fields with proper error context
        let reads_completed = parts[3]
            .parse()
            .with_context(|| format!("failed to parse reads_completed for device {}", device))?;
        let sectors_read = parts[5]
            .parse()
            .with_context(|| format!("failed to parse sectors_read for device {}", device))?;
        let writes_completed = parts[7]
            .parse()
            .with_context(|| format!("failed to parse writes_completed for device {}", device))?;
        let sectors_written = parts[9]
            .parse()
            .with_context(|| format!("failed to parse sectors_written for device {}", device))?;
        let time_spent_io = parts[12]
            .parse()
            .with_context(|| format!("failed to parse time_spent_io for device {}", device))?;

        stats.insert(
            device.to_string(),
            DiskStats {
                reads_completed,
                sectors_read,
                writes_completed,
                sectors_written,
                time_spent_io,
            },
        );
    }
    Ok(stats)
}

pub fn get_disk_io_info_from_str(
    initial_stats: &std::collections::HashMap<String, DiskStats>,
    final_stats: &std::collections::HashMap<String, DiskStats>,
    duration: std::time::Duration,
) -> Result<Vec<DiskIoInfo>> {
    if duration.is_zero() {
        return Ok(vec![]);
    }

    let mut result = vec![];
    let duration_secs = duration.as_secs_f64();

    for (device, final_stat) in final_stats {
        if let Some(initial_stat) = initial_stats.get(device) {
            // Use saturating_sub to prevent underflow
            let reads_completed_diff = final_stat
                .reads_completed
                .saturating_sub(initial_stat.reads_completed);
            let writes_completed_diff = final_stat
                .writes_completed
                .saturating_sub(initial_stat.writes_completed);
            let sectors_read_diff = final_stat
                .sectors_read
                .saturating_sub(initial_stat.sectors_read);
            let sectors_written_diff = final_stat
                .sectors_written
                .saturating_sub(initial_stat.sectors_written);
            let time_spent_io_diff = final_stat
                .time_spent_io
                .saturating_sub(initial_stat.time_spent_io);

            let read_iops = reads_completed_diff as f64 / duration_secs;
            let write_iops = writes_completed_diff as f64 / duration_secs;

            // Sectors are 512 bytes. Convert to MB/s
            let read_mb_s = sectors_read_diff as f64 * SECTOR_SIZE / MB_IN_BYTES / duration_secs;
            let write_mb_s =
                sectors_written_diff as f64 * SECTOR_SIZE / MB_IN_BYTES / duration_secs;

            // Utilization calculation with safety checks
            let utilization = if time_spent_io_diff > 0 && duration_secs > 0.0 {
                let util = (time_spent_io_diff as f64 / 1000.0 / duration_secs) * 100.0;
                util.clamp(0.0, 100.0) // Ensure utilization doesn't exceed 100%
            } else {
                0.0
            };

            // Round values to 2 decimal places
            let round_2dp = |val: f64| (val * 100.0).round() / 100.0;

            result.push(DiskIoInfo {
                device: device.clone(),
                read_iops: round_2dp(read_iops),
                write_iops: round_2dp(write_iops),
                read_mb_s: round_2dp(read_mb_s),
                write_mb_s: round_2dp(write_mb_s),
                utilization: round_2dp(utilization),
                total_reads_completed: final_stat.reads_completed,
                total_sectors_read: final_stat.sectors_read,
                total_writes_completed: final_stat.writes_completed,
                total_sectors_written: final_stat.sectors_written,
                total_time_spent_io: final_stat.time_spent_io,
            });
        }
    }

    // Sort by device name for consistent output
    result.sort_by(|a, b| a.device.cmp(&b.device));
    Ok(result)
}

// --- Application-specific structs (originally in lib.rs) ---
#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct SensorReading {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub current: Option<f64>,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct TailscalePeer {
    pub ip: String,
    pub name: String,
    pub os: String,
    pub status: String,
    pub online: bool,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct DiskInfo {
    pub filesystem: String,
    pub size: String,
    pub used: String,
    pub avail: String,
    pub use_perc: String,
    pub mount: String,
    pub total: String,
    pub percentage: u32,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct RamInfo {
    pub total: String,
    pub used: String,
    pub free: String,
    pub shared: String,
    pub buff_cache: String,
    pub available: String,
    pub percentage: f64,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct CpuUsage {
    pub core: String,
    pub usage: String,
}

#[derive(Debug, PartialEq)]
pub struct CpuStat {
    pub name: String,
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    pub guest: u64,
    pub guest_nice: u64,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct NetworkInterface {
    pub name: String,
    pub ip: String,
    pub state: String,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct NetDevInfo {
    pub interface: String,
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct TcpConnection {
    pub state: String,
    pub recv_q: u32,
    pub send_q: u32,
    pub local_address: String,
    pub peer_address: String,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct UptimeInfo {
    pub total_uptime_seconds: f64,
    pub idle_time_seconds: f64,
    pub formatted_uptime: String,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct SystemdService {
    pub unit: String,
    pub description: String,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct CgroupInfo {
    pub path: String,
    pub tasks: String,
    pub cpu: String,
    pub memory: String,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct TailscaleMetrics {
    pub total_tx: f64,
    pub total_rx: f64,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct LoadAvg {
    pub one_min: f32,
    pub five_min: f32,
    pub fifteen_min: f32,
    pub runnable_entities: u32,
    pub total_processes: u32,
}

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct SystemHealth {
    pub disk_info: DiskInfo,
    pub ram_info: RamInfo,
    pub cpu_usages: Vec<CpuUsage>,
    pub errors: Vec<String>,
    pub sensor_data: Vec<SensorReading>,
    pub tailscale_peers: Vec<TailscalePeer>,
    pub network_info: Vec<NetworkInterface>,
    pub load_avg: LoadAvg,
    pub tailscale_metrics: TailscaleMetrics,
    pub net_dev_info: Vec<NetDevInfo>,
    pub tcp_connections: Vec<TcpConnection>,
    pub uptime_info: UptimeInfo,
    pub services: Vec<SystemdService>,
    pub cgroup_data: Vec<CgroupInfo>,
    pub disk_io_info: Vec<DiskIoInfo>,
    #[serde(serialize_with = "serialize_timestamp")]
    pub timestamp: Zoned,
}

// --- Functions ---
fn get_network_info() -> anyhow::Result<Vec<NetworkInterface>> {
    let sh = Shell::new()?;
    let out = cmd!(sh, "ip -j addr").read()?;
    let addrs: Vec<IpAddrInfo> = serde_json::from_str(&out)?;
    let mut result = vec![];
    for iface in addrs {
        if iface.operstate == "DOWN" || iface.ifname == "lo" {
            continue;
        }
        let ip = iface
            .addr_info
            .iter()
            .find_map(|a| {
                if a.family == "inet" {
                    Some(a.local.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "No IPv4".to_string());
        result.push(NetworkInterface {
            name: iface.ifname,
            ip,
            state: iface.operstate,
        });
    }
    Ok(result)
}

#[cfg(target_os = "linux")]
pub fn get_load_avg() -> Result<LoadAvg> {
    let out = std::fs::read_to_string("/proc/loadavg")?;
    get_load_avg_from_str(&out)
}

#[cfg(not(target_os = "linux"))]
pub fn get_load_avg() -> Result<LoadAvg> {
    Ok(LoadAvg::default())
}

pub fn get_load_avg_from_str(input: &str) -> Result<LoadAvg> {
    let parts: Vec<_> = input.split_whitespace().collect();
    if parts.len() < 5 {
        anyhow::bail!("malformed /proc/loadavg output");
    }

    let one_min = parts[0].parse().context("parse one_min loadavg")?;
    let five_min = parts[1].parse().context("parse five_min loadavg")?;
    let fifteen_min = parts[2].parse().context("parse fifteen_min loadavg")?;
    let (runnable_entities, total_processes) = {
        let p = parts[3].split_once('/').context("parse runnable/total")?;
        (
            p.0.parse().context("parse runnable entities")?,
            p.1.parse().context("parse total processes")?,
        )
    };

    Ok(LoadAvg {
        one_min,
        five_min,
        fifteen_min,
        runnable_entities,
        total_processes,
    })
}

fn get_tailscale_metrics() -> anyhow::Result<TailscaleMetrics> {
    let sh = Shell::new()?;
    let output = cmd!(sh, "tailscale metrics").read()?;
    let mut total_tx = 0.0;
    let mut total_rx = 0.0;

    for line in output.lines() {
        if let Some(value_str) = line.split_whitespace().last()
            && let Ok(value) = value_str.parse::<f64>()
        {
            if line.starts_with("tailscaled_outbound_bytes_total") {
                total_tx += value;
            } else if line.starts_with("tailscaled_inbound_bytes_total") {
                total_rx += value;
            }
        }
    }

    // Convert bytes to MB and round to 2 decimal places
    let to_mb = |bytes: f64| (bytes / MB_IN_BYTES * 100.0).round() / 100.0;

    Ok(TailscaleMetrics {
        total_tx: to_mb(total_tx),
        total_rx: to_mb(total_rx),
    })
}
fn get_sensor_data() -> anyhow::Result<Vec<SensorReading>> {
    let sh = Shell::new()?;
    let out = cmd!(sh, "sensors -j").read()?;
    let root: SensorsRoot = serde_json::from_str(&out)?;
    let mut readings = vec![];

    // Process macsmc_hwmon-isa-0000
    let macsmc = root.macsmc;

    if let Some(input) = macsmc.ac_input_voltage {
        readings.push(SensorReading {
            name: "AC Input Voltage".to_string(),
            value: format!("{:.2}", input.in0_input),
            unit: "V".to_string(),
            current: Some(input.in0_input),
            ..Default::default()
        });
    }
    if let Some(fan) = macsmc.fan {
        readings.push(SensorReading {
            name: "Fan Speed".to_string(),
            value: format!("{:.0}", fan.fan1_input),
            unit: "RPM".to_string(),
            min: Some(fan.fan1_min),
            max: Some(fan.fan1_max),
            current: Some(fan.fan1_input),
        });
    }
    if let Some(temp) = macsmc.nand_flash_temperature {
        for (key, value) in temp.temps {
            readings.push(SensorReading {
                name: format!("NAND Flash Temperature ({})", key),
                value: format!("{:.1}", value),
                unit: "°C".to_string(),
                current: Some(value),
                ..Default::default()
            });
        }
    }
    if let Some(temp) = macsmc.wifi_bt_module_temp {
        for (key, value) in temp.temps {
            readings.push(SensorReading {
                name: format!("WiFi/BT Module Temp ({})", key),
                value: format!("{:.1}", value),
                unit: "°C".to_string(),
                current: Some(value),
                ..Default::default()
            });
        }
    }
    if let Some(power) = macsmc.total_system_power {
        for (key, value) in power.powers {
            readings.push(SensorReading {
                name: format!("Total System Power ({})", key),
                value: format!("{:.2}", value),
                unit: "W".to_string(),
                current: Some(value),
                ..Default::default()
            });
        }
    }
    if let Some(power) = macsmc.ac_input_power {
        for (key, value) in power.powers {
            readings.push(SensorReading {
                name: format!("AC Input Power ({})", key),
                value: format!("{:.2}", value),
                unit: "W".to_string(),
                current: Some(value),
                ..Default::default()
            });
        }
    }
    if let Some(power) = macsmc.rail_power {
        for (key, value) in power.powers {
            readings.push(SensorReading {
                name: format!("3.8 V Rail Power ({})", key),
                value: format!("{:.2}", value),
                unit: "W".to_string(),
                current: Some(value),
                ..Default::default()
            });
        }
    }
    if let Some(curr) = macsmc.ac_input_current {
        for (key, value) in curr.currents {
            readings.push(SensorReading {
                name: format!("AC Input Current ({})", key),
                value: format!("{:.0}", value * 1000.0),
                unit: "mA".to_string(),
                current: Some(value),
                ..Default::default()
            });
        }
    }

    readings.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(readings)
}

fn get_tailscale_peers() -> anyhow::Result<Vec<TailscalePeer>> {
    let sh = Shell::new()?;
    let out = cmd!(sh, "tailscale status --json").read()?;
    let status: TailscaleStatus = serde_json::from_str(&out)?;
    let mut peers = vec![];
    if let Some(peer_map) = status.peer {
        for (_, peer) in peer_map {
            peers.push(TailscalePeer {
                ip: peer.tailscale_ips.first().cloned().unwrap_or_default(),
                name: peer.host_name,
                os: "".to_string(), // OS is not available in the new struct
                status: if peer.online { "Online" } else { "Offline" }.to_string(),
                online: peer.online,
            });
        }
    }
    Ok(peers)
}

pub fn get_journal_logs(unit_name: &str) -> Result<String> {
    let sh = Shell::new()?;
    cmd!(
        sh,
        "journalctl -u {unit_name} --no-pager --output=short-iso --reverse --lines=200"
    )
    .read()
    .map_err(anyhow::Error::from)
}

#[cfg(target_os = "linux")]
pub fn get_disk_io_info() -> Result<Vec<DiskIoInfo>> {
    use std::sync::OnceLock;

    static LAST_DISK_STATS: OnceLock<
        std::sync::Mutex<
            Option<(
                std::collections::HashMap<String, DiskStats>,
                std::time::Instant,
            )>,
        >,
    > = OnceLock::new();

    let mutex = LAST_DISK_STATS.get_or_init(|| std::sync::Mutex::new(None));

    let current_content =
        std::fs::read_to_string("/proc/diskstats").context("failed to read /proc/diskstats")?;
    let current_stats = parse_diskstats_from_str(&current_content)?;
    let current_time = std::time::Instant::now();

    let mut last_stats_guard = mutex.lock().unwrap();

    if let Some((ref last_stats, last_time)) = *last_stats_guard {
        let duration = current_time.duration_since(last_time);

        if duration >= std::time::Duration::from_secs(2) {
            let result = get_disk_io_info_from_str(last_stats, &current_stats, duration)?;
            *last_stats_guard = Some((current_stats, current_time));
            return Ok(result);
        }
        // If not enough time has passed, return empty results but don't update stored stats
        return Ok(vec![]);
    }

    // First time, just store the stats
    *last_stats_guard = Some((current_stats, current_time));
    Ok(vec![])
}

#[cfg(not(target_os = "linux"))]
pub fn get_disk_io_info() -> Result<Vec<DiskIoInfo>> {
    Ok(vec![])
}

#[cfg(target_os = "linux")]
fn get_disk_info() -> Result<DiskInfo> {
    let stat = rustix::fs::statvfs("/").context("failed to get filesystem statistics")?;

    let total_bytes = stat.f_blocks * stat.f_frsize;
    let avail_bytes = stat.f_bavail * stat.f_frsize;
    let free_bytes = stat.f_bfree * stat.f_frsize;
    let used_bytes = total_bytes - free_bytes;

    let total_gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_gb = used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let avail_gb = avail_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

    let percentage = if total_bytes > 0 {
        ((used_bytes as f64 / total_bytes as f64) * 100.0).round() as u32
    } else {
        0
    };

    Ok(DiskInfo {
        filesystem: "rootfs".to_string(),
        size: format!("{:.1}G", total_gb),
        used: format!("{:.1}G", used_gb),
        avail: format!("{:.1}G", avail_gb),
        use_perc: format!("{}%", percentage),
        mount: "/".to_string(),
        total: format!("{:.1}G", total_gb),
        percentage,
    })
}

#[cfg(not(target_os = "linux"))]
fn get_disk_info() -> Result<DiskInfo> {
    Ok(DiskInfo::default())
}

pub fn get_ram_info_from_str(input: &str) -> Result<RamInfo> {
    let mut mem_total = None;
    let mut mem_available = None;
    let mut mem_free = None;
    let mut shmem = None;
    let mut buffers = 0.0;
    let mut cached = 0.0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let value: f64 = parts[1]
            .parse()
            .with_context(|| format!("failed to parse value for {}", parts[0]))?;

        match parts[0] {
            "MemTotal:" => mem_total = Some(value),
            "MemAvailable:" => mem_available = Some(value),
            "MemFree:" => mem_free = Some(value),
            "Shmem:" => shmem = Some(value),
            "Buffers:" => buffers = value,
            "Cached:" => cached = value,
            _ => {}
        }
    }

    let total_kib = mem_total.context("MemTotal not found in /proc/meminfo")?;
    let available_kib = mem_available.context("MemAvailable not found in /proc/meminfo")?;
    let free_kib = mem_free.context("MemFree not found in /proc/meminfo")?;
    let shared_kib = shmem.context("Shmem not found in /proc/meminfo")?;
    let buff_cache_kib = buffers + cached;

    let percentage = if total_kib > 0.0 {
        ((total_kib - available_kib) / total_kib) * 100.0
    } else {
        0.0
    };

    let to_gb = |kib: f64| format!("{:.2} GB", kib / KIB_TO_GIB);

    Ok(RamInfo {
        total: to_gb(total_kib),
        used: to_gb(total_kib - available_kib),
        free: to_gb(free_kib),
        shared: to_gb(shared_kib),
        buff_cache: to_gb(buff_cache_kib),
        available: to_gb(available_kib),
        percentage: (percentage * 10.0).round() / 10.0,
    })
}

#[cfg(target_os = "linux")]
fn get_ram_info() -> Result<RamInfo> {
    let out = std::fs::read_to_string("/proc/meminfo")?;
    get_ram_info_from_str(&out)
}

#[cfg(not(target_os = "linux"))]
fn get_ram_info() -> Result<RamInfo> {
    Ok(RamInfo::default())
}

#[cfg(target_os = "linux")]
fn get_cpu_usage() -> Result<Vec<CpuUsage>> {
    fn parse_cpu_stats() -> Result<Vec<CpuStat>> {
        let content = std::fs::read_to_string("/proc/stat").context("failed to read /proc/stat")?;
        let mut stats = vec![];

        for line in content.lines() {
            if line.starts_with("cpu") {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() < 11 {
                    continue; // Skip malformed lines
                }

                let name = fields[0].to_string();
                let values: Result<Vec<u64>, _> = fields[1..11]
                    .iter()
                    .map(|v| v.parse::<u64>().context("parse cpu stat value"))
                    .collect();

                let values = values?;
                if values.len() >= 10 {
                    stats.push(CpuStat {
                        name,
                        user: values[0],
                        nice: values[1],
                        system: values[2],
                        idle: values[3],
                        iowait: values[4],
                        irq: values[5],
                        softirq: values[6],
                        steal: values[7],
                        guest: values[8],
                        guest_nice: values[9],
                    });
                }
            }
        }
        Ok(stats)
    }

    let stats1 = parse_cpu_stats()?;
    std::thread::sleep(std::time::Duration::from_millis(500));
    let stats2 = parse_cpu_stats()?;

    let mut usages = Vec::with_capacity(stats1.len());

    for (stat1, stat2) in stats1.iter().zip(stats2.iter()) {
        if stat1.name != stat2.name {
            continue; // Safety check
        }

        let total1 = stat1.user
            + stat1.nice
            + stat1.system
            + stat1.idle
            + stat1.iowait
            + stat1.irq
            + stat1.softirq
            + stat1.steal
            + stat1.guest
            + stat1.guest_nice;
        let total2 = stat2.user
            + stat2.nice
            + stat2.system
            + stat2.idle
            + stat2.iowait
            + stat2.irq
            + stat2.softirq
            + stat2.steal
            + stat2.guest
            + stat2.guest_nice;

        let total_diff = total2.saturating_sub(total1);
        let idle_diff = stat2.idle.saturating_sub(stat1.idle);

        let usage_percent = if total_diff > 0 {
            let active_diff = total_diff.saturating_sub(idle_diff);
            (active_diff as f64 / total_diff as f64) * 100.0
        } else {
            0.0
        };

        let display_name = if stat1.name == "cpu" {
            "Total".to_string()
        } else {
            format!("Core {}", stat1.name.trim_start_matches("cpu"))
        };

        usages.push(CpuUsage {
            core: display_name,
            usage: format!("{:.1}", usage_percent.clamp(0.0, 100.0)),
        });
    }

    Ok(usages)
}

#[cfg(not(target_os = "linux"))]
fn get_cpu_usage() -> Result<Vec<CpuUsage>> {
    Ok(vec![])
}

#[cfg(target_os = "linux")]
fn get_net_dev_info() -> anyhow::Result<Vec<NetDevInfo>> {
    let out = std::fs::read_to_string("/proc/net/dev")?;
    get_net_dev_info_from_str(&out)
}

#[cfg(not(target_os = "linux"))]
fn get_net_dev_info() -> anyhow::Result<Vec<NetDevInfo>> {
    Ok(vec![])
}

pub fn get_net_dev_info_from_str(input: &str) -> anyhow::Result<Vec<NetDevInfo>> {
    let mut result = vec![];
    for line in input.lines().skip(2) {
        // Skip header lines
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            let interface = parts[0].trim_end_matches(':').to_string();
            let rx_bytes = parts[1].parse().context("parse rx_bytes")?;
            let rx_packets = parts[2].parse().context("parse rx_packets")?;
            let tx_bytes = parts[9].parse().context("parse tx_bytes")?;
            let tx_packets = parts[10].parse().context("parse tx_packets")?;

            result.push(NetDevInfo {
                interface,
                rx_bytes,
                rx_packets,
                tx_bytes,
                tx_packets,
            });
        }
    }
    Ok(result)
}

fn get_tcp_connections() -> anyhow::Result<Vec<TcpConnection>> {
    let sh = Shell::new()?;
    let out = cmd!(sh, "ss -t -n").read()?;
    get_tcp_connections_from_str(&out)
}

#[cfg(target_os = "linux")]
fn get_uptime_info() -> anyhow::Result<UptimeInfo> {
    let out = std::fs::read_to_string("/proc/uptime")?;
    get_uptime_info_from_str(&out)
}

#[cfg(not(target_os = "linux"))]
fn get_uptime_info() -> anyhow::Result<UptimeInfo> {
    Ok(UptimeInfo::default())
}

pub fn get_uptime_info_from_str(input: &str) -> anyhow::Result<UptimeInfo> {
    let parts: Vec<_> = input.split_whitespace().collect();
    if parts.len() < 2 {
        anyhow::bail!("malformed /proc/uptime output");
    }

    let total_uptime_seconds: f64 = parts[0].parse().context("parse total_uptime_seconds")?;
    let idle_time_seconds: f64 = parts[1].parse().context("parse idle_time_seconds")?;

    let days = (total_uptime_seconds / 86400.0).floor();
    let hours = ((total_uptime_seconds % 86400.0) / 3600.0).floor();
    let minutes = ((total_uptime_seconds % 3600.0) / 60.0).floor();

    let formatted_uptime = format!("{:.0}d {:.0}h {:.0}m", days, hours, minutes);

    Ok(UptimeInfo {
        total_uptime_seconds,
        idle_time_seconds,
        formatted_uptime,
    })
}

pub fn get_tcp_connections_from_str(input: &str) -> anyhow::Result<Vec<TcpConnection>> {
    let mut connections = vec![];
    for line in input.lines().skip(1) {
        // Skip header line
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            connections.push(TcpConnection {
                state: parts[0].to_string(),
                recv_q: parts[1].parse().context("parse recv_q")?,
                send_q: parts[2].parse().context("parse send_q")?,
                local_address: parts[3].to_string(),
                peer_address: parts[4].to_string(),
            });
        }
    }
    Ok(connections)
}

fn get_services() -> anyhow::Result<Vec<SystemdService>> {
    let sh = Shell::new()?;
    let out = cmd!(
        sh,
        "systemctl list-units --type=service --state=running --no-pager --plain --no-legend"
    )
    .read()?;
    get_services_from_str(&out)
}

pub fn get_services_from_str(input: &str) -> anyhow::Result<Vec<SystemdService>> {
    let mut services = vec![];
    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() >= 4 && parts[0].ends_with(".service") {
            services.push(SystemdService {
                unit: parts[0].to_string(),
                description: parts[4..].join(" "),
            });
        }
    }
    Ok(services)
}

fn get_cgroup_data() -> anyhow::Result<Vec<CgroupInfo>> {
    let sh = Shell::new()?;
    let out = cmd!(sh, "systemd-cgtop -b -n 1 --order=memory").read()?;
    get_cgroup_data_from_str(&out)
}

pub fn get_cgroup_data_from_str(input: &str) -> anyhow::Result<Vec<CgroupInfo>> {
    let mut cgroups = vec![];
    for line in input.lines().skip(1) {
        // Skip header
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() >= 4 && (parts[2] != "-" || parts[3] != "-") {
            cgroups.push(CgroupInfo {
                path: parts[0].to_string(),
                tasks: parts[1].to_string(),
                cpu: parts[2].to_string(),
                memory: parts[3].to_string(),
            });
        }
    }
    cgroups.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(cgroups)
}

pub fn get_system_health() -> SystemHealth {
    let mut errors = vec![];
    let disk_info = match get_disk_info() {
        Ok(d) => d,
        Err(e) => {
            errors.push(format!("disk: {e}"));
            DiskInfo::default()
        }
    };
    let ram_info = match get_ram_info() {
        Ok(r) => r,
        Err(e) => {
            errors.push(format!("ram: {e}"));
            RamInfo::default()
        }
    };
    let cpu_usages = match get_cpu_usage() {
        Ok(c) => c,
        Err(e) => {
            errors.push(format!("cpu: {e}"));
            vec![]
        }
    };
    let sensor_data = match get_sensor_data() {
        Ok(s) => s,
        Err(e) => {
            errors.push(format!("sensors: {e}"));
            vec![]
        }
    };
    let tailscale_peers = match get_tailscale_peers() {
        Ok(t) => t,
        Err(e) => {
            errors.push(format!("tailscale: {e}"));
            vec![]
        }
    };
    let network_info = match get_network_info() {
        Ok(n) => n,
        Err(e) => {
            errors.push(format!("network: {e}"));
            vec![]
        }
    };
    let load_avg = match get_load_avg() {
        Ok(l) => l,
        Err(e) => {
            errors.push(format!("load_avg: {e}"));
            LoadAvg::default()
        }
    };
    let tailscale_metrics = match get_tailscale_metrics() {
        Ok(m) => m,
        Err(e) => {
            errors.push(format!("tailscale_metrics: {e}"));
            TailscaleMetrics::default()
        }
    };
    let net_dev_info = match get_net_dev_info() {
        Ok(n) => n,
        Err(e) => {
            errors.push(format!("net_dev_info: {e}"));
            vec![]
        }
    };
    let tcp_connections = match get_tcp_connections() {
        Ok(t) => t,
        Err(e) => {
            errors.push(format!("tcp_connections: {e}"));
            vec![]
        }
    };
    let uptime_info = match get_uptime_info() {
        Ok(u) => u,
        Err(e) => {
            errors.push(format!("uptime_info: {e}"));
            UptimeInfo::default()
        }
    };
    let services = match get_services() {
        Ok(s) => s,
        Err(e) => {
            errors.push(format!("services: {e}"));
            vec![]
        }
    };
    let cgroup_data = match get_cgroup_data() {
        Ok(c) => c,
        Err(e) => {
            errors.push(format!("cgroup_data: {e}"));
            vec![]
        }
    };
    let disk_io_info = match get_disk_io_info() {
        Ok(d) => d,
        Err(e) => {
            errors.push(format!("disk_io: {e}"));
            vec![]
        }
    };
    SystemHealth {
        disk_info,
        ram_info,
        cpu_usages,
        sensor_data,
        tailscale_peers,
        network_info,
        load_avg,
        tailscale_metrics,
        net_dev_info,
        tcp_connections,
        uptime_info,
        services,
        cgroup_data,
        disk_io_info,
        errors,
        timestamp: Zoned::now(),
    }
}
