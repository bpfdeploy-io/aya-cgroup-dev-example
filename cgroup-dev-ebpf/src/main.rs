#![no_std]
#![no_main]

use aya_bpf::{
    macros::cgroup_sysctl,
    programs::SysctlContext,
};
use aya_log_ebpf::info;

#[cgroup_sysctl(name="cgroup_dev")]
pub fn cgroup_dev(ctx: SysctlContext) -> i32 {
    match try_cgroup_dev(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_cgroup_dev(ctx: SysctlContext) -> Result<i32, i32> {
    info!(&ctx, "sysctl operation called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
