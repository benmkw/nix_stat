use nix_stat::{DiskIoInfo, get_disk_io_info_from_str, parse_diskstats_from_str};
use std::time::Duration;

#[test]
fn test_disk_io_parsing_and_calculation() -> Result<(), Box<dyn std::error::Error>> {
    let initial_diskstats = r#"
1       0 nvme0n1 27806 0 2130680 3086 6266 0 1263800 8723 0 2592 0 0
1       1 nvme0n1p1 100 0 800 10 50 0 400 5 0 10 0 0
1       2 nvme0n1p2 200 0 1600 20 100 0 800 10 0 20 0 0
7       0 loop0 0 0 0 0 0 0 0 0 0 0 0
"#;

    let final_diskstats = r#"
1       0 nvme0n1 27816 0 2130780 3096 6276 0 1263900 8733 0 2602 0 0
1       1 nvme0n1p1 105 0 840 15 55 0 440 10 0 15 0 0
1       2 nvme0n1p2 205 0 1640 25 105 0 840 15 0 25 0 0
7       0 loop0 0 0 0 0 0 0 0 0 0 0 0
"#;

    let result = get_disk_io_info_from_str(
        &parse_diskstats_from_str(&initial_diskstats)?,
        &parse_diskstats_from_str(&final_diskstats)?,
        Duration::from_secs(1),
    )
    .unwrap();

    assert_eq!(
        &result,
        &[
            DiskIoInfo {
                device: "nvme0n1".to_string(),
                read_iops: 10.0,
                write_iops: 10.0,
                read_mb_s: 0.05,
                write_mb_s: 0.05,
                utilization: 1.0,
                total_reads_completed: 27816,
                total_sectors_read: 2130780,
                total_writes_completed: 6276,
                total_sectors_written: 1263900,
                total_time_spent_io: 2602,
            },
            DiskIoInfo {
                device: "nvme0n1p1".to_string(),
                read_iops: 5.0,
                write_iops: 5.0,
                read_mb_s: 0.02,
                write_mb_s: 0.02,
                utilization: 0.5,
                total_reads_completed: 105,
                total_sectors_read: 840,
                total_writes_completed: 55,
                total_sectors_written: 440,
                total_time_spent_io: 15,
            },
            DiskIoInfo {
                device: "nvme0n1p2".to_string(),
                read_iops: 5.0,
                write_iops: 5.0,
                read_mb_s: 0.02,
                write_mb_s: 0.02,
                utilization: 0.5,
                total_reads_completed: 205,
                total_sectors_read: 1640,
                total_writes_completed: 105,
                total_sectors_written: 840,
                total_time_spent_io: 25,
            },
        ]
    );
    Ok(())
}

#[test]
fn test_disk_io_parsing_and_calculation_with_new_data() -> Result<(), Box<dyn std::error::Error>> {
    let initial_diskstats = r#"
 259       0 nvme0n1 27806 12340 2130680 3086 6266 38034 1263800 8723 0 2592 12211 0 0 0 0 115 400
 259       1 nvme0n1p1 41 0 3136 20 0 0 0 0 0 20 20 0 0 0 0 0 0
 259       2 nvme0n1p2 41 0 3136 19 0 0 0 0 0 19 19 0 0 0 0 0 0
 259       3 nvme0n1p3 41 0 3136 20 0 0 0 0 0 17 20 0 0 0 0 0 0
 259       4 nvme0n1p4 547 115 117584 107 123 65 81136 138 0 120 246 0 0 0 0 0 0
 259       5 nvme0n1p5 27032 12225 1997784 2876 6141 37969 1182664 8577 0 2843 11454 0 0 0 0 0 0
 259       6 nvme0n1p6 43 0 3216 20 0 0 0 0 0 20 20 0 0 0 0 0 0
 259       7 nvme0n2 47 0 1728 40 0 0 0 0 0 43 40 0 0 0 0 0 0
 259       8 nvme0n3 63 0 3520 40 0 0 0 0 0 38 40 0 0 0 0 0 0
   7       0 loop0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       1 loop1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       2 loop2 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       3 loop3 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       4 loop4 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       5 loop5 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       6 loop6 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       7 loop7 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
"#;

    let final_diskstats = r#"
 259       0 nvme0n1 27816 12340 2130780 3096 6271 38034 1263850 8723 0 2692 12211 0 0 0 0 115 400
 259       1 nvme0n1p1 41 0 3136 20 0 0 0 0 0 20 20 0 0 0 0 0 0
 259       2 nvme0n1p2 41 0 3136 19 0 0 0 0 0 19 19 0 0 0 0 0 0
 259       3 nvme0n1p3 41 0 3136 20 0 0 0 0 0 17 20 0 0 0 0 0 0
 259       4 nvme0n1p4 547 115 117584 107 123 65 81136 138 0 120 246 0 0 0 0 0 0
 259       5 nvme0n1p5 27037 12225 1997834 2876 6143 37969 1182684 8577 0 2893 11454 0 0 0 0 0 0
 259       6 nvme0n1p6 43 0 3216 20 0 0 0 0 0 20 20 0 0 0 0 0 0
 259       7 nvme0n2 47 0 1728 40 0 0 0 0 0 43 40 0 0 0 0 0 0
 259       8 nvme0n3 63 0 3520 40 0 0 0 0 0 38 40 0 0 0 0 0 0
   7       0 loop0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       1 loop1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       2 loop2 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       3 loop3 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       4 loop4 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       5 loop5 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       6 loop6 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
   7       7 loop7 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
"#;

    let result = get_disk_io_info_from_str(
        &parse_diskstats_from_str(&initial_diskstats)?,
        &parse_diskstats_from_str(&final_diskstats)?,
        Duration::from_secs(1),
    )
    .unwrap();

    assert_eq!(
        result,
        &[
            DiskIoInfo {
                device: "nvme0n1".to_string(),
                read_iops: 10.0,
                write_iops: 5.0,
                read_mb_s: 0.05,
                write_mb_s: 0.02,
                utilization: 10.0,
                total_reads_completed: 27816,
                total_sectors_read: 2130780,
                total_writes_completed: 6271,
                total_sectors_written: 1263850,
                total_time_spent_io: 2692,
            },
            DiskIoInfo {
                device: "nvme0n1p1".to_string(),
                read_iops: 0.0,
                write_iops: 0.0,
                read_mb_s: 0.0,
                write_mb_s: 0.0,
                utilization: 0.0,
                total_reads_completed: 41,
                total_sectors_read: 3136,
                total_writes_completed: 0,
                total_sectors_written: 0,
                total_time_spent_io: 20,
            },
            DiskIoInfo {
                device: "nvme0n1p2".to_string(),
                read_iops: 0.0,
                write_iops: 0.0,
                read_mb_s: 0.0,
                write_mb_s: 0.0,
                utilization: 0.0,
                total_reads_completed: 41,
                total_sectors_read: 3136,
                total_writes_completed: 0,
                total_sectors_written: 0,
                total_time_spent_io: 19,
            },
            DiskIoInfo {
                device: "nvme0n1p3".to_string(),
                read_iops: 0.0,
                write_iops: 0.0,
                read_mb_s: 0.0,
                write_mb_s: 0.0,
                utilization: 0.0,
                total_reads_completed: 41,
                total_sectors_read: 3136,
                total_writes_completed: 0,
                total_sectors_written: 0,
                total_time_spent_io: 17,
            },
            DiskIoInfo {
                device: "nvme0n1p4".to_string(),
                read_iops: 0.0,
                write_iops: 0.0,
                read_mb_s: 0.0,
                write_mb_s: 0.0,
                utilization: 0.0,
                total_reads_completed: 547,
                total_sectors_read: 117584,
                total_writes_completed: 123,
                total_sectors_written: 81136,
                total_time_spent_io: 120,
            },
            DiskIoInfo {
                device: "nvme0n1p5".to_string(),
                read_iops: 5.0,
                write_iops: 2.0,
                read_mb_s: 0.02,
                write_mb_s: 0.01,
                utilization: 5.0,
                total_reads_completed: 27037,
                total_sectors_read: 1997834,
                total_writes_completed: 6143,
                total_sectors_written: 1182684,
                total_time_spent_io: 2893,
            },
            DiskIoInfo {
                device: "nvme0n1p6".to_string(),
                read_iops: 0.0,
                write_iops: 0.0,
                read_mb_s: 0.0,
                write_mb_s: 0.0,
                utilization: 0.0,
                total_reads_completed: 43,
                total_sectors_read: 3216,
                total_writes_completed: 0,
                total_sectors_written: 0,
                total_time_spent_io: 20,
            },
            DiskIoInfo {
                device: "nvme0n2".to_string(),
                read_iops: 0.0,
                write_iops: 0.0,
                read_mb_s: 0.0,
                write_mb_s: 0.0,
                utilization: 0.0,
                total_reads_completed: 47,
                total_sectors_read: 1728,
                total_writes_completed: 0,
                total_sectors_written: 0,
                total_time_spent_io: 43,
            },
            DiskIoInfo {
                device: "nvme0n3".to_string(),
                read_iops: 0.0,
                write_iops: 0.0,
                read_mb_s: 0.0,
                write_mb_s: 0.0,
                utilization: 0.0,
                total_reads_completed: 63,
                total_sectors_read: 3520,
                total_writes_completed: 0,
                total_sectors_written: 0,
                total_time_spent_io: 38,
            },
        ]
    );
    Ok(())
}
