use crate::models::CPUInfo;
use std::{
    io::{BufRead, BufReader},
    path::Path,
};

impl CPUInfo {
    pub fn new() -> Option<Self> {
        let content = Self::read_file("/proc/cpuinfo")?;
        let (brand, core_num, frequency) = Self::parse_content(content)?;

        Some(Self {
            brand,
            core_num,
            frequency,
        })
    }

    fn read_file(path: impl AsRef<Path>) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    fn parse_content(content: String) -> Option<(String, (u64, u64), f64)> {
        let mut brand = None;
        let mut physical_core_num = None;
        let mut logical_core_num = 0;
        let mut frequency = None;


        for line in content.lines().filter(|x| !x.is_empty()) {
            let (head, tail) = line.split_once(':').map(|(a, b)| (a.trim(), b.trim()))?;

            if brand.is_none() && head == "model name" {
                brand = Some(tail.to_string());
            } else if physical_core_num.is_none() && head == "cpu cores" {
                physical_core_num = tail.parse().ok();
            } else if frequency.is_none() && head == "cpu MHz" {
                frequency = tail.parse().ok();
            } else if head == "processor" {
                logical_core_num += 1;
            }

            if brand.is_some() && physical_core_num.is_some() && frequency.is_some() {
                continue;
            }
        }
        Some((
            brand?,
            (physical_core_num?, logical_core_num),
            frequency?,
        ))
    }
}

#[test]
fn cpuinfo() {
    let content = String::from("
processor	: 0
vendor_id	: AuthenticAMD
cpu family	: 23
model		: 104
model name	: AMD Ryzen 7 5700U with Radeon Graphics
stepping	: 1
microcode	: 0x8608108
cpu MHz		: 1775.083
cache size	: 512 KB
physical id	: 0
siblings	: 16
core id		: 0
cpu cores	: 8
apicid		: 0
initial apicid	: 0
fpu		: yes
fpu_exception	: yes
cpuid level	: 16
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc cpuid extd_apicid aperfmperf rapl pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate ssbd mba ibrs ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd cppc arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif v_spec_ctrl umip rdpid overflow_recov succor smca
bugs		: sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass retbleed smt_rsb srso ibpb_no_ret
bogomips	: 3593.04
TLB size	: 3072 4K pages
clflush size	: 64
cache_alignment	: 64
address sizes	: 48 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate cpb eff_freq_ro [13] [14]

processor	: 1
vendor_id	: AuthenticAMD
cpu family	: 23
model		: 104
model name	: AMD Ryzen 7 5700U with Radeon Graphics
stepping	: 1
microcode	: 0x8608108
cpu MHz		: 1776.519
cache size	: 512 KB
physical id	: 0
siblings	: 16
core id		: 0
cpu cores	: 8
apicid		: 1
initial apicid	: 1
fpu		: yes
fpu_exception	: yes
cpuid level	: 16
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc cpuid extd_apicid aperfmperf rapl pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate ssbd mba ibrs ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr rdpru wbnoinvd cppc arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif v_spec_ctrl umip rdpid overflow_recov succor smca
bugs		: sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass retbleed smt_rsb srso ibpb_no_ret
bogomips	: 3593.04
TLB size	: 3072 4K pages
clflush size	: 64
cache_alignment	: 64
address sizes	: 48 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate cpb eff_freq_ro [13] [14]
");

    let (brand, core_num, frequency) = CPUInfo::parse_content(content).unwrap();
    assert_eq!(brand, "AMD Ryzen 7 5700U with Radeon Graphics");
    assert_eq!(core_num, 8);
    assert_eq!(frequency, 1775.083);
}
