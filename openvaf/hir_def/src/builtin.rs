//! Generated by `generate_builtins`, do not edit by hand.

use ahash::RandomState;
use indexmap::IndexMap;
use syntax::name::{kw, sysfun, Name};

use crate::nameres::ScopeDefItem;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
#[allow(nonstandard_style, unreachable_pub)]
#[repr(u8)]
pub enum BuiltIn {
    abs = 0u8,
    acos = 1u8,
    acosh = 2u8,
    asin = 3u8,
    asinh = 4u8,
    atan = 5u8,
    atan2 = 6u8,
    atanh = 7u8,
    cos = 8u8,
    cosh = 9u8,
    exp = 10u8,
    floor = 11u8,
    flow = 12u8,
    potential = 13u8,
    hypot = 14u8,
    ln = 15u8,
    log = 16u8,
    max = 17u8,
    min = 18u8,
    pow = 19u8,
    sin = 20u8,
    sinh = 21u8,
    sqrt = 22u8,
    tan = 23u8,
    tanh = 24u8,
    display = 25u8,
    strobe = 26u8,
    write = 27u8,
    monitor = 28u8,
    debug = 29u8,
    fclose = 30u8,
    fopen = 31u8,
    fdisplay = 32u8,
    fwrite = 33u8,
    fstrobe = 34u8,
    fmonitor = 35u8,
    fgets = 36u8,
    fscanf = 37u8,
    swrite = 38u8,
    sformat = 39u8,
    sscanf = 40u8,
    rewind = 41u8,
    fseek = 42u8,
    ftell = 43u8,
    fflush = 44u8,
    ferror = 45u8,
    feof = 46u8,
    fdebug = 47u8,
    finish = 48u8,
    stop = 49u8,
    fatal = 50u8,
    warning = 51u8,
    error = 52u8,
    info = 53u8,
    abstime = 54u8,
    dist_chi_square = 55u8,
    dist_exponential = 56u8,
    dist_poisson = 57u8,
    dist_uniform = 58u8,
    dist_erlang = 59u8,
    dist_normal = 60u8,
    dist_t = 61u8,
    random = 62u8,
    arandom = 63u8,
    rdist_chi_square = 64u8,
    rdist_exponential = 65u8,
    rdist_poisson = 66u8,
    rdist_uniform = 67u8,
    rdist_erlang = 68u8,
    rdist_normal = 69u8,
    rdist_t = 70u8,
    clog2 = 71u8,
    log10 = 72u8,
    ceil = 73u8,
    temperature = 74u8,
    vt = 75u8,
    simparam = 76u8,
    simparam_str = 77u8,
    simprobe = 78u8,
    discontinuity = 79u8,
    param_given = 80u8,
    port_connected = 81u8,
    analog_node_alias = 82u8,
    analog_port_alias = 83u8,
    test_plusargs = 84u8,
    value_plusargs = 85u8,
    bound_step = 86u8,
    analysis = 87u8,
    ac_stim = 88u8,
    noise_table = 89u8,
    noise_table_log = 90u8,
    white_noise = 91u8,
    flicker_noise = 92u8,
    limit = 93u8,
    absdelay = 94u8,
    ddt = 95u8,
    idt = 96u8,
    idtmod = 97u8,
    ddx = 98u8,
    zi_nd = 99u8,
    zi_np = 100u8,
    zi_zd = 101u8,
    zi_zp = 102u8,
    laplace_nd = 103u8,
    laplace_np = 104u8,
    laplace_zd = 105u8,
    laplace_zp = 106u8,
    limexp = 107u8,
    last_crossing = 108u8,
    slew = 109u8,
    transition = 110u8,
}
#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
#[allow(nonstandard_style, unreachable_pub)]
pub enum ParamSysFun {
    mfactor,
    xposition,
    yposition,
    angle,
    hflip,
    vflip,
}
impl ParamSysFun {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::mfactor, Self::xposition, Self::yposition, Self::angle, Self::hflip, Self::vflip]
            .into_iter()
    }
}
impl BuiltIn {
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_analog_operator(self) -> bool {
        match self {
            BuiltIn::absdelay
            | BuiltIn::ddt
            | BuiltIn::idt
            | BuiltIn::idtmod
            | BuiltIn::ddx
            | BuiltIn::zi_nd
            | BuiltIn::zi_np
            | BuiltIn::zi_zd
            | BuiltIn::zi_zp
            | BuiltIn::laplace_nd
            | BuiltIn::laplace_np
            | BuiltIn::laplace_zd
            | BuiltIn::laplace_zp
            | BuiltIn::limexp
            | BuiltIn::last_crossing
            | BuiltIn::slew
            | BuiltIn::transition => true,
            _ => false,
        }
    }
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_analog_operator_sysfun(self) -> bool {
        match self {
            BuiltIn::limit => true,
            _ => false,
        }
    }
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_unsupported(self) -> bool {
        match self {
            BuiltIn::simprobe
            | BuiltIn::analog_node_alias
            | BuiltIn::analog_port_alias
            | BuiltIn::test_plusargs
            | BuiltIn::value_plusargs
            | BuiltIn::zi_nd
            | BuiltIn::zi_np
            | BuiltIn::zi_zd
            | BuiltIn::zi_zp
            | BuiltIn::laplace_nd
            | BuiltIn::laplace_np
            | BuiltIn::laplace_zd
            | BuiltIn::laplace_zp
            | BuiltIn::last_crossing
            | BuiltIn::slew
            | BuiltIn::transition
            | BuiltIn::fclose
            | BuiltIn::fopen
            | BuiltIn::fdisplay
            | BuiltIn::fwrite
            | BuiltIn::fstrobe
            | BuiltIn::fmonitor
            | BuiltIn::fgets
            | BuiltIn::fscanf
            | BuiltIn::swrite
            | BuiltIn::sformat
            | BuiltIn::sscanf
            | BuiltIn::rewind
            | BuiltIn::fseek
            | BuiltIn::ftell
            | BuiltIn::fflush
            | BuiltIn::ferror
            | BuiltIn::feof
            | BuiltIn::fdebug
            | BuiltIn::dist_chi_square
            | BuiltIn::dist_exponential
            | BuiltIn::dist_poisson
            | BuiltIn::dist_uniform
            | BuiltIn::dist_erlang
            | BuiltIn::dist_normal
            | BuiltIn::dist_t
            | BuiltIn::random
            | BuiltIn::arandom
            | BuiltIn::rdist_chi_square
            | BuiltIn::rdist_exponential
            | BuiltIn::rdist_poisson
            | BuiltIn::rdist_uniform
            | BuiltIn::rdist_erlang
            | BuiltIn::rdist_normal
            | BuiltIn::rdist_t => true,
            _ => false,
        }
    }
    #[allow(clippy::match_like_matches_macro)]
    pub fn is_analysis_var(self) -> bool {
        match self {
            BuiltIn::analysis
            | BuiltIn::ac_stim
            | BuiltIn::noise_table
            | BuiltIn::noise_table_log
            | BuiltIn::white_noise
            | BuiltIn::flicker_noise => true,
            _ => false,
        }
    }
}
pub fn insert_builtin_scope(dst: &mut IndexMap<Name, ScopeDefItem, RandomState>) {
    dst.insert(kw::abs, BuiltIn::abs.into());
    dst.insert(kw::acos, BuiltIn::acos.into());
    dst.insert(kw::acosh, BuiltIn::acosh.into());
    dst.insert(kw::asin, BuiltIn::asin.into());
    dst.insert(kw::asinh, BuiltIn::asinh.into());
    dst.insert(kw::atan, BuiltIn::atan.into());
    dst.insert(kw::atan2, BuiltIn::atan2.into());
    dst.insert(kw::atanh, BuiltIn::atanh.into());
    dst.insert(kw::cos, BuiltIn::cos.into());
    dst.insert(kw::cosh, BuiltIn::cosh.into());
    dst.insert(kw::exp, BuiltIn::exp.into());
    dst.insert(kw::floor, BuiltIn::floor.into());
    dst.insert(kw::flow, BuiltIn::flow.into());
    dst.insert(kw::potential, BuiltIn::potential.into());
    dst.insert(kw::hypot, BuiltIn::hypot.into());
    dst.insert(kw::ln, BuiltIn::ln.into());
    dst.insert(kw::log, BuiltIn::log.into());
    dst.insert(kw::max, BuiltIn::max.into());
    dst.insert(kw::min, BuiltIn::min.into());
    dst.insert(kw::pow, BuiltIn::pow.into());
    dst.insert(kw::sin, BuiltIn::sin.into());
    dst.insert(kw::sinh, BuiltIn::sinh.into());
    dst.insert(kw::sqrt, BuiltIn::sqrt.into());
    dst.insert(kw::tan, BuiltIn::tan.into());
    dst.insert(kw::tanh, BuiltIn::tanh.into());
    dst.insert(sysfun::display, BuiltIn::display.into());
    dst.insert(sysfun::strobe, BuiltIn::strobe.into());
    dst.insert(sysfun::write, BuiltIn::write.into());
    dst.insert(sysfun::monitor, BuiltIn::monitor.into());
    dst.insert(sysfun::debug, BuiltIn::debug.into());
    dst.insert(sysfun::fclose, BuiltIn::fclose.into());
    dst.insert(sysfun::fopen, BuiltIn::fopen.into());
    dst.insert(sysfun::fdisplay, BuiltIn::fdisplay.into());
    dst.insert(sysfun::fwrite, BuiltIn::fwrite.into());
    dst.insert(sysfun::fstrobe, BuiltIn::fstrobe.into());
    dst.insert(sysfun::fmonitor, BuiltIn::fmonitor.into());
    dst.insert(sysfun::fgets, BuiltIn::fgets.into());
    dst.insert(sysfun::fscanf, BuiltIn::fscanf.into());
    dst.insert(sysfun::swrite, BuiltIn::swrite.into());
    dst.insert(sysfun::sformat, BuiltIn::sformat.into());
    dst.insert(sysfun::sscanf, BuiltIn::sscanf.into());
    dst.insert(sysfun::rewind, BuiltIn::rewind.into());
    dst.insert(sysfun::fseek, BuiltIn::fseek.into());
    dst.insert(sysfun::ftell, BuiltIn::ftell.into());
    dst.insert(sysfun::fflush, BuiltIn::fflush.into());
    dst.insert(sysfun::ferror, BuiltIn::ferror.into());
    dst.insert(sysfun::feof, BuiltIn::feof.into());
    dst.insert(sysfun::fdebug, BuiltIn::fdebug.into());
    dst.insert(sysfun::finish, BuiltIn::finish.into());
    dst.insert(sysfun::stop, BuiltIn::stop.into());
    dst.insert(sysfun::fatal, BuiltIn::fatal.into());
    dst.insert(sysfun::warning, BuiltIn::warning.into());
    dst.insert(sysfun::error, BuiltIn::error.into());
    dst.insert(sysfun::info, BuiltIn::info.into());
    dst.insert(sysfun::abstime, BuiltIn::abstime.into());
    dst.insert(sysfun::dist_chi_square, BuiltIn::dist_chi_square.into());
    dst.insert(sysfun::dist_exponential, BuiltIn::dist_exponential.into());
    dst.insert(sysfun::dist_poisson, BuiltIn::dist_poisson.into());
    dst.insert(sysfun::dist_uniform, BuiltIn::dist_uniform.into());
    dst.insert(sysfun::dist_erlang, BuiltIn::dist_erlang.into());
    dst.insert(sysfun::dist_normal, BuiltIn::dist_normal.into());
    dst.insert(sysfun::dist_t, BuiltIn::dist_t.into());
    dst.insert(sysfun::random, BuiltIn::random.into());
    dst.insert(sysfun::arandom, BuiltIn::arandom.into());
    dst.insert(sysfun::rdist_chi_square, BuiltIn::rdist_chi_square.into());
    dst.insert(sysfun::rdist_exponential, BuiltIn::rdist_exponential.into());
    dst.insert(sysfun::rdist_poisson, BuiltIn::rdist_poisson.into());
    dst.insert(sysfun::rdist_uniform, BuiltIn::rdist_uniform.into());
    dst.insert(sysfun::rdist_erlang, BuiltIn::rdist_erlang.into());
    dst.insert(sysfun::rdist_normal, BuiltIn::rdist_normal.into());
    dst.insert(sysfun::rdist_t, BuiltIn::rdist_t.into());
    dst.insert(sysfun::clog2, BuiltIn::clog2.into());
    dst.insert(sysfun::ln, BuiltIn::ln.into());
    dst.insert(sysfun::log10, BuiltIn::log10.into());
    dst.insert(sysfun::exp, BuiltIn::exp.into());
    dst.insert(sysfun::sqrt, BuiltIn::sqrt.into());
    dst.insert(sysfun::pow, BuiltIn::pow.into());
    dst.insert(sysfun::floor, BuiltIn::floor.into());
    dst.insert(sysfun::ceil, BuiltIn::ceil.into());
    dst.insert(sysfun::sin, BuiltIn::sin.into());
    dst.insert(sysfun::cos, BuiltIn::cos.into());
    dst.insert(sysfun::tan, BuiltIn::tan.into());
    dst.insert(sysfun::asin, BuiltIn::asin.into());
    dst.insert(sysfun::acos, BuiltIn::acos.into());
    dst.insert(sysfun::atan, BuiltIn::atan.into());
    dst.insert(sysfun::atan2, BuiltIn::atan2.into());
    dst.insert(sysfun::hypot, BuiltIn::hypot.into());
    dst.insert(sysfun::sinh, BuiltIn::sinh.into());
    dst.insert(sysfun::cosh, BuiltIn::cosh.into());
    dst.insert(sysfun::tanh, BuiltIn::tanh.into());
    dst.insert(sysfun::asinh, BuiltIn::asinh.into());
    dst.insert(sysfun::acosh, BuiltIn::acosh.into());
    dst.insert(sysfun::atanh, BuiltIn::atanh.into());
    dst.insert(sysfun::temperature, BuiltIn::temperature.into());
    dst.insert(sysfun::vt, BuiltIn::vt.into());
    dst.insert(sysfun::simparam, BuiltIn::simparam.into());
    dst.insert(sysfun::simparam_str, BuiltIn::simparam_str.into());
    dst.insert(sysfun::simprobe, BuiltIn::simprobe.into());
    dst.insert(sysfun::discontinuity, BuiltIn::discontinuity.into());
    dst.insert(sysfun::param_given, BuiltIn::param_given.into());
    dst.insert(sysfun::port_connected, BuiltIn::port_connected.into());
    dst.insert(sysfun::analog_node_alias, BuiltIn::analog_node_alias.into());
    dst.insert(sysfun::analog_port_alias, BuiltIn::analog_port_alias.into());
    dst.insert(sysfun::test_plusargs, BuiltIn::test_plusargs.into());
    dst.insert(sysfun::value_plusargs, BuiltIn::value_plusargs.into());
    dst.insert(sysfun::bound_step, BuiltIn::bound_step.into());
    dst.insert(kw::analysis, BuiltIn::analysis.into());
    dst.insert(kw::ac_stim, BuiltIn::ac_stim.into());
    dst.insert(kw::noise_table, BuiltIn::noise_table.into());
    dst.insert(kw::noise_table_log, BuiltIn::noise_table_log.into());
    dst.insert(kw::white_noise, BuiltIn::white_noise.into());
    dst.insert(kw::flicker_noise, BuiltIn::flicker_noise.into());
    dst.insert(sysfun::limit, BuiltIn::limit.into());
    dst.insert(kw::absdelay, BuiltIn::absdelay.into());
    dst.insert(kw::ddt, BuiltIn::ddt.into());
    dst.insert(kw::idt, BuiltIn::idt.into());
    dst.insert(kw::idtmod, BuiltIn::idtmod.into());
    dst.insert(kw::ddx, BuiltIn::ddx.into());
    dst.insert(kw::zi_nd, BuiltIn::zi_nd.into());
    dst.insert(kw::zi_np, BuiltIn::zi_np.into());
    dst.insert(kw::zi_zd, BuiltIn::zi_zd.into());
    dst.insert(kw::zi_zp, BuiltIn::zi_zp.into());
    dst.insert(kw::laplace_nd, BuiltIn::laplace_nd.into());
    dst.insert(kw::laplace_np, BuiltIn::laplace_np.into());
    dst.insert(kw::laplace_zd, BuiltIn::laplace_zd.into());
    dst.insert(kw::laplace_zp, BuiltIn::laplace_zp.into());
    dst.insert(kw::limexp, BuiltIn::limexp.into());
    dst.insert(kw::last_crossing, BuiltIn::last_crossing.into());
    dst.insert(kw::slew, BuiltIn::slew.into());
    dst.insert(kw::transition, BuiltIn::transition.into());
}
pub fn insert_module_builtin_scope(dst: &mut IndexMap<Name, ScopeDefItem, RandomState>) {
    dst.insert(sysfun::mfactor, ParamSysFun::mfactor.into());
    dst.insert(sysfun::xposition, ParamSysFun::xposition.into());
    dst.insert(sysfun::yposition, ParamSysFun::yposition.into());
    dst.insert(sysfun::angle, ParamSysFun::angle.into());
    dst.insert(sysfun::hflip, ParamSysFun::hflip.into());
    dst.insert(sysfun::vflip, ParamSysFun::vflip.into());
}
