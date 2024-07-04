use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::slice;

use anyhow::{bail, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use libc::c_void;
use libloading::Library;
use log::{debug, error, info, warn};
use openvaf::{
    AbsPathBuf, CompilationDestination, CompilationTermination, LintLevel, OptLevel, Target,
};

use crate::devices::DeviceImpl;
use crate::veriloga::osdi_0_4::{
    OsdiDescriptor, LOG_FMT_ERR, LOG_LVL_DEBUG, LOG_LVL_DISPLAY, LOG_LVL_ERR, LOG_LVL_FATAL,
    LOG_LVL_INFO, LOG_LVL_MASK, LOG_LVL_WARN,
};
use crate::veriloga::osdi_device::OsdiDevice;

pub(crate) use osdi_0_4::{
    ANALYSIS_AC, ANALYSIS_DC, ANALYSIS_IC, ANALYSIS_NOISE, ANALYSIS_STATIC, ANALYSIS_TRAN,
    CALC_NOISE, CALC_REACT_JACOBIAN, CALC_REACT_RESIDUAL, CALC_RESIST_JACOBIAN,
    CALC_RESIST_RESIDUAL,
};

// autogenerated
#[allow(warnings)]
mod osdi_0_4;
mod osdi_device;

#[derive(Default)]
pub struct Opts {
    pub defines: Vec<String>,
    pub codegen_opts: Vec<String>,
    pub cache_dir: Option<Utf8PathBuf>,
    pub lints: Vec<(String, LintLevel)>,
    include: Vec<AbsPathBuf>,
    pub opt_lvl: Option<OptLevel>,
}

impl Opts {
    pub fn add_include_dir(&mut self, dir: &Utf8Path) -> Result<()> {
        let dir = dir.canonicalize().with_context(|| format!("directory {dir} not found"))?;
        self.include.push(AbsPathBuf::assert(dir));
        Ok(())
    }
}

pub fn compile_va(path: &Utf8Path, opts: &Opts) -> Result<Vec<Box<dyn DeviceImpl>>> {
    let cache_dir = if let Some(dir) = &opts.cache_dir {
        dir.clone()
    } else {
        let path = directories_next::ProjectDirs::from("com", "semimod", "melange")
            .context("failed to find cache directory\nhelp: consider setting it manually")?
            .cache_dir()
            .to_owned();
        if let Ok(path) = Utf8PathBuf::from_path_buf(path) {
            path
        } else {
            bail!("failed to find cache directory\nhelp: consider setting it manually")
        }
    };
    let openvaf_opts = openvaf::Opts {
        defines: opts.defines.clone(),
        codegen_opts: opts.codegen_opts.clone(),
        lints: opts.lints.clone(),
        input: path.to_owned(),
        output: CompilationDestination::Cache { cache_dir },
        include: opts.include.clone(),
        opt_lvl: opts.opt_lvl.unwrap_or(OptLevel::Aggressive),
        target: Target::host_target()
            .context("openvaf does currently not support this hardware/os")?,
        target_cpu: "native".to_owned(),
        dry_run: false,
    };

    let res = openvaf::compile(&openvaf_opts);
    let res = res.with_context(|| format!("openvaf: compilation of {path} failed"))?;
    let lib_file = match res {
        CompilationTermination::Compiled { lib_file } => lib_file,
        CompilationTermination::FatalDiagnostic => {
            bail!("openvaf: compilation of {path} failed");
        }
    };
    let libs = unsafe { load_osdi_lib(&lib_file)? };
    let libs = libs.iter().map(|descriptor| Box::new(OsdiDevice { descriptor }) as _).collect();
    Ok(libs)
}

unsafe fn load_osdi_lib(path: &Utf8Path) -> Result<&'static [OsdiDescriptor]> {
    let lib = Library::new(path)?;
    let lib = Box::leak(Box::new(lib));

    let major_version: &u32 = *lib.get(b"OSDI_VERSION_MAJOR\0")?;
    let minor_version: &u32 = *lib.get(b"OSDI_VERSION_MINOR\0")?;

    if *major_version != 0 || *minor_version != 3 {
        bail!(
            "melange only supports OSDI v0.3 but {path} targets v{major_version}.{minor_version}",
        );
    }

    let num_descriptors: &u32 = *lib.get(b"OSDI_NUM_DESCRIPTORS\0")?;
    let descriptors: *const OsdiDescriptor = *lib.get(b"OSDI_DESCRIPTORS\0")?;

    let descriptors: &[OsdiDescriptor] =
        slice::from_raw_parts(descriptors, *num_descriptors as usize);

    if let Ok(osdi_log_ptr) =
        lib.get::<*mut unsafe fn(*mut c_void, *const c_char, u32)>(b"osdi_log\0")
    {
        osdi_log_ptr.write(osdi_log)
    }
    Ok(descriptors)
}

unsafe fn osdi_log(handle: *mut c_void, msg: *const c_char, lvl: u32) {
    let _ = catch_unwind(|| osdi_log_impl(handle, msg, lvl));
}

unsafe fn osdi_log_impl(handle: *mut c_void, msg: *const c_char, lvl: u32) {
    let instance = handle as *const c_char;
    let instance = CStr::from_ptr(instance).to_str().expect("all OSDI strings must be valid utf-8");
    let msg = CStr::from_ptr(msg).to_str().expect("all OSDI strings must be valid utf-8");

    if (lvl & LOG_FMT_ERR) == 0 {
        match lvl & LOG_LVL_MASK {
            LOG_LVL_DEBUG => debug!("{instance} - {msg}"),
            LOG_LVL_DISPLAY => print!("{instance} - {msg}"),
            LOG_LVL_INFO => info!("{instance} - {msg}"),
            LOG_LVL_WARN => warn!("{instance} - {msg}"),
            LOG_LVL_ERR => error!("{instance} - {msg}"),
            LOG_LVL_FATAL => error!("{instance} - FATAL {msg}"),
            _ => error!("{instance} - UNKNOWN_LOG_LVL {msg}"),
        }
    } else {
        match lvl & LOG_LVL_MASK {
            LOG_LVL_DEBUG => debug!("{instance} - failed to format\"{msg}\""),
            LOG_LVL_DISPLAY => println!("{instance} - failed to format\"{msg}\""),
            LOG_LVL_INFO => info!("{instance} - failed to format\"{msg}\""),
            LOG_LVL_WARN => warn!("{instance} - failed to format\"{msg}\""),
            LOG_LVL_ERR => error!("{instance} - failed to format\"{msg}\""),
            LOG_LVL_FATAL => error!("{instance} - FATAL failed to format\"{msg}\""),
            _ => error!("{instance} - UNKNOWN_LOG_LVL failed to format\"{msg}\""),
        }
    }
}
